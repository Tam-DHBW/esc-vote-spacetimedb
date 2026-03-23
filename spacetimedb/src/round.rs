use std::collections::HashSet;

use spacetimedb::{SpacetimeType, ViewContext};
use spacetimedsl::dsl;

use crate::{
    country::{
        CountryId, GetAllCountryRows, GetAllParticipatingCountryRows, GetRotwCountryRow,
        ParticipatingCountryId,
    },
    ranking::{CreateRanking, CreateRankingRow, RankingKind},
};

#[derive(SpacetimeType, Clone, Copy, PartialEq, Debug, strum::Display)]
pub enum RoundKind {
    SemiFinal1,
    SemiFinal2,
    GrandFinal,
}

/// A voting round: Semi-Final 1, Semi-Final 2, or Grand Final.
#[dsl(
    plural_name = rounds,
    method(update = true, delete = true),
    unique_index(name = year_and_kind)
)]
#[spacetimedb::table(
    accessor = round,
    public,
    index(accessor = year_and_kind, btree(columns = [year, kind]))
)]
pub struct Round {
    #[primary_key]
    #[auto_inc]
    #[create_wrapper]
    #[referenced_by(path = crate::round, table = participation)]
    #[referenced_by(path = crate::round, table = active_round)]
    #[referenced_by(path = crate::vote, table = tele_vote)]
    #[referenced_by(path = crate::vote, table = juror_vote)]
    #[referenced_by(path = crate::ranking, table = ranking)]
    id: u16,

    year: u16,

    kind: RoundKind,
}

#[spacetimedsl::dsl(
    plural_name = participations,
    method(update = false, delete = false),
    hook(after(insert)),
)]
#[spacetimedb::table(accessor = participation)]
pub struct Participation {
    #[primary_key]
    #[auto_inc]
    #[create_wrapper]
    id: u16,

    #[use_wrapper(RoundId)]
    #[index(btree)]
    #[foreign_key(path = crate::round, table = round, column = id, on_delete = Error)]
    round_id: u16,

    #[use_wrapper(ParticipatingCountryId)]
    #[index(btree)]
    #[foreign_key(path = crate::country, table = participating_country, column = id, on_delete = Error)]
    participating_country_id: u16,
}

#[spacetimedsl::hook]
fn after_participation_insert(
    dsl: &spacetimedsl::DSL<'_, T>,
    participation: &Participation,
) -> Result<(), spacetimedsl::SpacetimeDSLError> {
    let round = dsl.get_round_by_id(participation.get_round_id())?;
    let rotw_country = dsl.get_rotw_country()?;
    let participating_country_ids: HashSet<_> = dsl
        .get_all_participating_countries()
        .map(|pc| pc.get_country_id())
        .collect();

    let make_create_country = |kind: RankingKind, from_country_id: CountryId| CreateRanking {
        round_id: participation.get_round_id(),
        kind,
        from_country_id,
        to_country_id: participation.get_participating_country_id(),
        score: 0,
        rank: 0,
    };

    for country in dsl.get_all_countries() {
        dsl.create_ranking(make_create_country(RankingKind::TeleVote, country.get_id()))?;

        if matches!(round.get_kind(), RoundKind::GrandFinal)
            && participating_country_ids.contains(&country.get_id())
        {
            dsl.create_ranking(make_create_country(RankingKind::JuryVote, country.get_id()))?;
        }
    }

    dsl.create_ranking(make_create_country(
        RankingKind::Overall,
        rotw_country.get_country_id(),
    ))?;

    Ok(())
}

#[spacetimedsl::dsl(singleton, method(update = false, delete = true))]
#[spacetimedb::table(accessor = active_round)]
pub struct ActiveRound {
    #[foreign_key(path=crate::round, table=round, column = id, on_delete = Delete)]
    #[use_wrapper(RoundId)]
    round_id: u16,
}

#[spacetimedb::view(accessor = get_active_round, public)]
fn get_active_round(ctx: &ViewContext) -> Option<Round> {
    let dsl = spacetimedsl::read_only_dsl(ctx);

    let round_id = dsl.get_active_round().ok()?.get_round_id();
    dsl.get_round_by_id(round_id).ok()
}
