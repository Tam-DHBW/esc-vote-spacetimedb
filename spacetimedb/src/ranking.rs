use crate::{
    country::{CountryId, GetRotwCountryRow, ParticipatingCountryId},
    round::RoundId,
};
use spacetimedb::SpacetimeType;
use spacetimedsl::WriteContext;

#[derive(SpacetimeType, Clone, Copy, PartialEq, strum::Display, Debug)]
pub enum RankingKind {
    TeleVote,
    JuryVote,
    Overall,
}

/// Table to efficiently compute the 1224-type ranking based on a score.
/// For overall ranking calculations, the `from` country is ROTW.
#[spacetimedsl::dsl(
    plural_name = rankings,
    method(update = true, delete = true),
    hook(after(update)),
    unique_index(name = round_kind_from_to)
)]
#[spacetimedb::table(
    accessor = ranking,
    public,
    index(accessor = round_kind_from, btree(columns = [round_id, kind, from_country_id])),
    index(accessor = round_kind_from_to, btree(columns = [round_id, kind, from_country_id, to_country_id])),
    index(accessor = round_kind_from_score, btree(columns = [round_id, kind, from_country_id, score])),
)]
pub struct Ranking {
    #[primary_key]
    #[auto_inc]
    #[create_wrapper]
    id: u64,

    #[use_wrapper(RoundId)]
    #[index(btree)]
    #[foreign_key(path = crate::round, table = round, column = id, on_delete = Delete)]
    round_id: u16,

    kind: RankingKind,

    #[use_wrapper(CountryId)]
    #[index(btree)]
    #[foreign_key(path = crate::country, table = country, column = id, on_delete = Delete)]
    from_country_id: u16,

    #[use_wrapper(ParticipatingCountryId)]
    #[index(btree)]
    #[foreign_key(path = crate::country, table = participating_country, column = id, on_delete = Delete)]
    to_country_id: u16,

    pub score: u32,

    /// Rank starting at 0
    pub rank: u16,
}

// Very efficient rank update algorithm for 1224-type ranking
//
// ON INCREMENT(P):
//    P.rank -= size(group(P.score + 1))
//    P.score += 1
//    for each Q in group(P.score - 1): Q.rank += 1
//
// ON DECREMENT(P):
//    for each Q in group(P.score - 1): Q.rank -= 1
//    P.score -= 1
//    P.rank += size(group(P.score + 1))
//

macro_rules! impl_update_ranking {
    ($name:ident, $add:tt, $sub:tt, [$($op:ident),+]) => {
        #[allow(unused_assignments)]
        pub fn $name<T: WriteContext>(
            dsl: &spacetimedsl::DSL<'_, T>,
            round_id: &RoundId,
            kind: &RankingKind,
            from_country_id: &CountryId,
            to_country_id: &ParticipatingCountryId,
        ) -> Result<(), spacetimedsl::SpacetimeDSLError> {
            let mut entry: Ranking = dsl.get_ranking_by_round_kind_from_to(round_id, kind, from_country_id, to_country_id)?;

            $(impl_update_ranking!(@$op, dsl, round_id, kind, from_country_id, entry, $add, $sub);)*

            Ok(())
        }
    };
    (@rank, $dsl:ident, $round:ident, $kind:ident, $from:ident, $entry:ident, $add:tt, $sub:tt) => {
        *$entry.get_rank_mut() $sub $dsl
            .get_rankings_by_round_kind_from_score($round, $kind, $from, &($entry.get_score() + 1))
            .count() as u16;
        $entry = $dsl.update_ranking_by_id($entry)?;

    };
    (@score, $dsl:ident, $round:ident, $kind:ident, $from:ident, $entry:ident, $add:tt, $sub:tt) => {
        *$entry.get_score_mut() $add 1;
        $entry = $dsl.update_ranking_by_id($entry)?;
    };
    (@ties, $dsl:ident, $round:ident, $kind:ident, $from:ident, $entry:ident, $add:tt, $sub:tt) => {
        for mut prev_tied in $dsl.get_rankings_by_round_kind_from_score($round, $kind, $from, &($entry.get_score() - 1)) {
            *prev_tied.get_rank_mut() $add 1;
            $dsl.update_ranking_by_id(prev_tied)?;
        }
    };
}

impl_update_ranking!(increment_ranking_score, +=, -=, [rank, score, ties]);
impl_update_ranking!(decrement_ranking_score, -=, +=, [ties, score, rank]);

/// Calculate the points for a ranking, starting from 0
pub fn get_points(ranking: &Ranking) -> u8 {
    if  *ranking.get_score() == 0 {
        return 0;
    }

    *[12, 10, 8, 7, 6, 5, 4, 3, 2, 1]
        .get(*ranking.get_rank() as usize)
        .unwrap_or(&0)
}

#[spacetimedsl::hook]
fn after_ranking_update(
    dsl: &spacetimedsl::DSL<'_, T>,
    old: &Ranking,
    new: &Ranking,
) -> Result<(), spacetimedsl::SpacetimeDSLError> {
    if matches!(
        old.get_kind(),
        RankingKind::TeleVote | RankingKind::JuryVote
    ) {
        let rotw_country_id = dsl.get_rotw_country()?.get_country_id();

        let mut difference = get_points(new) as i8 - get_points(old) as i8;

        while difference > 0 {
            increment_ranking_score(
                dsl,
                &old.get_round_id(),
                &RankingKind::Overall,
                &rotw_country_id,
                &old.get_to_country_id(),
            )?;
            difference -= 1;
        }

        while difference < 0 {
            decrement_ranking_score(
                dsl,
                &old.get_round_id(),
                &RankingKind::Overall,
                &rotw_country_id,
                &old.get_to_country_id(),
            )?;
            difference += 1;
        }
    }

    Ok(())
}
