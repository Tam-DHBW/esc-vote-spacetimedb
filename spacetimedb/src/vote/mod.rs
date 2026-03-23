use crate::{
    country::{CountryId, GetParticipatingCountryRowOptionById, ParticipatingCountryId},
    ranking::RankingKind,
    round::{GetParticipationRowsByRoundId, RoundId},
    voter::{GetJurorRowOptionById, GetViewerRowOptionById},
};

pub mod submit;

const AVAILABLE_TELE_VOTES: usize = 20;

/// A single televote tap. Each viewer can have up to 20 rows per round.
#[spacetimedsl::dsl(
    plural_name = tele_votes,
    method(update = false, delete = true),
    hook(after(insert, delete))
)]
#[spacetimedb::table(
    accessor = tele_vote,
    index(accessor = viewer_and_round, btree(columns=[viewer_id, round_id])),
)]
pub struct TeleVote {
    #[primary_key]
    #[auto_inc]
    #[create_wrapper]
    id: u64,

    #[use_wrapper(crate::voter::ViewerId)]
    #[index(btree)]
    #[foreign_key(path = crate::voter, table = viewer, column = id, on_delete = Delete)]
    viewer_id: u64,

    #[use_wrapper(crate::round::RoundId)]
    #[index(btree)]
    #[foreign_key(path = crate::round, table = round, column = id, on_delete = Delete)]
    round_id: u16,

    #[use_wrapper(crate::country::ParticipatingCountryId)]
    #[index(btree)]
    #[foreign_key(path = crate::country, table = participating_country, column = id, on_delete = Delete)]
    to_country_id: u16,
}

/// A juror's rank for one country in a round
#[spacetimedsl::dsl(
    plural_name = juror_votes,
    method(update = false, delete = true),
    hook(after(insert, delete)),
    unique_index(name = juror_round_rank),
    unique_index(name = juror_round_country),
)]
#[spacetimedb::table(
    accessor = juror_vote,
    index(accessor = juror_and_round, btree(columns = [juror_id, round_id])),
    index(accessor = juror_round_rank, btree(columns = [juror_id, round_id, rank])),
    index(accessor = juror_round_country, btree(columns = [juror_id, round_id, ranked_country_id])),
)]
pub struct JurorVote {
    #[primary_key]
    #[auto_inc]
    #[create_wrapper]
    id: u64,

    #[use_wrapper(crate::voter::JurorId)]
    #[index(btree)]
    #[foreign_key(path = crate::voter, table = juror, column = id, on_delete = Delete)]
    juror_id: u64,

    #[use_wrapper(crate::round::RoundId)]
    #[index(btree)]
    #[foreign_key(path = crate::round, table = round, column = id, on_delete = Delete)]
    round_id: u16,

    #[use_wrapper(crate::country::ParticipatingCountryId)]
    #[index(btree)]
    #[foreign_key(path = crate::country, table = participating_country, column = id, on_delete = Delete)]
    ranked_country_id: u16,

    rank: u16,
}

fn update_tele_ranking<T: spacetimedsl::WriteContext>(
    dsl: &spacetimedsl::DSL<'_, T>,
    vote: &TeleVote,
    apply: fn(
        &spacetimedsl::DSL<'_, T>,
        &RoundId,
        &RankingKind,
        &CountryId,
        &ParticipatingCountryId,
    ) -> Result<(), spacetimedsl::SpacetimeDSLError>,
) -> Result<(), spacetimedsl::SpacetimeDSLError> {
    let viewer_country_id = dsl.get_viewer_by_id(vote.get_viewer_id())?.get_country_id();
    apply(
        dsl,
        &vote.get_round_id(),
        &RankingKind::TeleVote,
        &viewer_country_id,
        &vote.get_to_country_id(),
    )
}

#[spacetimedsl::hook]
fn after_tele_vote_insert(
    dsl: &spacetimedsl::DSL<'_, T>,
    vote: &TeleVote,
) -> Result<(), spacetimedsl::SpacetimeDSLError> {
    update_tele_ranking(dsl, vote, crate::ranking::increment_ranking_score)
}

#[spacetimedsl::hook]
fn after_tele_vote_delete(
    dsl: &spacetimedsl::DSL<'_, T>,
    vote: &TeleVote,
) -> Result<(), spacetimedsl::SpacetimeDSLError> {
    update_tele_ranking(dsl, vote, crate::ranking::decrement_ranking_score)
}

fn update_jury_ranking<T: spacetimedsl::WriteContext>(
    dsl: &spacetimedsl::DSL<'_, T>,
    vote: &JurorVote,
    apply: fn(
        &spacetimedsl::DSL<'_, T>,
        &RoundId,
        &RankingKind,
        &CountryId,
        &ParticipatingCountryId,
    ) -> Result<(), spacetimedsl::SpacetimeDSLError>,
) -> Result<(), spacetimedsl::SpacetimeDSLError> {
    let juror_country_id = dsl
        .get_participating_country_by_id(
            dsl.get_juror_by_id(vote.get_juror_id())?
                .get_participating_country_id(),
        )?
        .get_country_id();

    let score = dsl
        .get_participations_by_round_id(vote.get_round_id())
        .count()
        - *vote.get_rank() as usize;

    for _ in 0..score {
        apply(
            dsl,
            &vote.get_round_id(),
            &RankingKind::JuryVote,
            &juror_country_id,
            &vote.get_ranked_country_id(),
        )?;
    }

    Ok(())
}

#[spacetimedsl::hook]
fn after_juror_vote_insert(
    dsl: &spacetimedsl::DSL<'_, T>,
    vote: &JurorVote,
) -> Result<(), spacetimedsl::SpacetimeDSLError> {
    update_jury_ranking(dsl, vote, crate::ranking::increment_ranking_score)
}

#[spacetimedsl::hook]
fn after_juror_vote_delete(
    dsl: &spacetimedsl::DSL<'_, T>,
    vote: &JurorVote,
) -> Result<(), spacetimedsl::SpacetimeDSLError> {
    update_jury_ranking(dsl, vote, crate::ranking::decrement_ranking_score)
}
