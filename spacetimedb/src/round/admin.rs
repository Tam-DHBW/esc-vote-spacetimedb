use spacetimedb::ReducerContext;

use spacetimedsl::itertools::Itertools;

use crate::{
    country::{GetAllParticipatingCountryRows, GetRotwCountryRow},
    ranking::{GetRankingRowsByRoundKindFrom, RankingKind},
    round::{
        CreateActiveRound, CreateActiveRoundRow, CreateParticipation, CreateParticipationRow,
        CreateRound, CreateRoundRow, DeleteActiveRoundRow, GetActiveRoundRow,
        GetRoundRowOptionById, GetRoundRowOptionByYearAndKind, RoundKind, UpdateRoundRowById,
    },
};

const QUALIFIERS_PER_SEMI: usize = 10;

#[spacetimedb::reducer]
fn create_semi_finals(ctx: &ReducerContext, year: u16) -> Result<(), String> {
    let dsl = spacetimedsl::dsl(ctx);

    let sf1 = dsl.create_round(CreateRound {
        year,
        kind: RoundKind::SemiFinal1,
        voting_open: true,
    })?;

    let sf2 = dsl.create_round(CreateRound {
        year,
        kind: RoundKind::SemiFinal2,
        voting_open: true,
    })?;

    let rounds = [sf1.get_id(), sf2.get_id()];

    for (i, participating_country) in dsl.get_all_participating_countries().enumerate() {
        dsl.create_participation(CreateParticipation {
            round_id: rounds[i % 2].clone(),
            participating_country_id: participating_country.get_id(),
        })?;
    }

    let _ = dsl.delete_active_round();
    dsl.create_active_round(CreateActiveRound {
        round_id: sf1.get_id(),
    })?;

    Ok(())
}

#[spacetimedb::reducer]
fn advance_round(ctx: &ReducerContext) -> Result<(), String> {
    let dsl = spacetimedsl::dsl(ctx);

    let active = dsl.get_active_round()?;
    let mut current_round = dsl.get_round_by_id(active.get_round_id())?;

    // First call closes voting, second call advances the round.
    if *current_round.get_voting_open() {
        *current_round.get_voting_open_mut() = false;
        dsl.update_round_by_id(current_round)?;
        return Ok(());
    }

    match current_round.get_kind() {
        RoundKind::SemiFinal1 => {
            let sf2 =
                dsl.get_round_by_year_and_kind(&current_round.get_year(), &RoundKind::SemiFinal2)?;

            dsl.delete_active_round()?;
            dsl.create_active_round(CreateActiveRound {
                round_id: sf2.get_id(),
            })?;

            Ok(())
        }
        RoundKind::SemiFinal2 => {
            let sf1 =
                dsl.get_round_by_year_and_kind(&current_round.get_year(), &RoundKind::SemiFinal1)?;

            let rotw_country_id = dsl.get_rotw_country()?.get_country_id();

            let qualifiers = [&sf1.get_id(), &current_round.get_id()]
                .into_iter()
                .flat_map(|round_id| {
                    dsl.get_rankings_by_round_kind_from(
                        round_id,
                        &RankingKind::Overall,
                        &rotw_country_id,
                    )
                    .sorted_by_key(|r| *r.get_rank())
                    .take(QUALIFIERS_PER_SEMI)
                    .map(|r| r.get_to_country_id())
                })
                .collect::<Vec<_>>();

            let grand_final = dsl.create_round(CreateRound {
                year: *current_round.get_year(),
                kind: RoundKind::GrandFinal,
                voting_open: true,
            })?;

            for country_id in qualifiers {
                dsl.create_participation(CreateParticipation {
                    round_id: grand_final.get_id(),
                    participating_country_id: country_id,
                })?;
            }

            dsl.delete_active_round()?;
            dsl.create_active_round(CreateActiveRound {
                round_id: grand_final.get_id(),
            })?;

            Ok(())
        }
        RoundKind::GrandFinal => {
            dsl.delete_active_round()?;
            Ok(())
        }
    }
}
