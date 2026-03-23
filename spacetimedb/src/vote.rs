use std::collections::HashSet;

use spacetimedb::ReducerContext;
use spacetimedsl::dsl;

use crate::{
    country::{CountryId, GetParticipatingCountryRowOptionById, ParticipatingCountryId},
    round::{GetActiveRoundRow, GetParticipationRowsByRoundId, GetRoundRowOptionById, RoundId},
    user::GetUserRowOptionByIdentity,
    voter::{GetJurorRowOptionByUserId, GetViewerRowOptionByUserId},
};

const AVAILABLE_TELE_VOTES: usize = 20;

/// A single televote tap. Each viewer can have up to 20 rows per round.
#[dsl(plural_name = tele_votes, method(update = false, delete = true))]
#[spacetimedb::table(accessor = tele_vote)]
pub struct TeleVote {
    #[primary_key]
    #[auto_inc]
    #[create_wrapper]
    id: u64,

    #[use_wrapper(crate::round::RoundId)]
    #[index(btree)]
    #[foreign_key(path = crate::round, table = round, column = id, on_delete = Delete)]
    round_id: u16,

    #[use_wrapper(crate::voter::ViewerId)]
    #[index(btree)]
    #[foreign_key(path = crate::voter, table = viewer, column = id, on_delete = Delete)]
    viewer_id: u64,

    #[use_wrapper(crate::country::ParticipatingCountryId)]
    #[index(btree)]
    #[foreign_key(path = crate::country, table = participating_country, column = id, on_delete = Delete)]
    to_country_id: u16,
}

/// A juror's rank for one country in a round
#[dsl(
    plural_name = juror_votes,
    method(update = false, delete = true),
    unique_index(name = juror_round_rank),
    unique_index(name = juror_round_country),
)]
#[spacetimedb::table(
    accessor = juror_vote,
    index(accessor = juror_round_rank, btree(columns = [juror_id, round_id, rank])),
    index(accessor = juror_round_country, btree(columns = [juror_id, round_id, ranked_country_id])),
)]
pub struct JurorVote {
    #[primary_key]
    #[auto_inc]
    #[create_wrapper]
    id: u64,

    #[use_wrapper(crate::round::RoundId)]
    #[index(btree)]
    #[foreign_key(path = crate::round, table = round, column = id, on_delete = Delete)]
    round_id: u16,

    #[use_wrapper(crate::voter::JurorId)]
    #[index(btree)]
    #[foreign_key(path = crate::voter, table = juror, column = id, on_delete = Delete)]
    juror_id: u64,

    #[use_wrapper(crate::country::ParticipatingCountryId)]
    #[index(btree)]
    #[foreign_key(path = crate::country, table = participating_country, column = id, on_delete = Delete)]
    ranked_country_id: u16,

    rank: u16,
}

fn validate_votes<'a>(
    ctx: &ReducerContext,
    round_id: &RoundId,
    from_country_id: &CountryId,
    votes: impl IntoIterator<Item = &'a ParticipatingCountryId>,
) -> Result<(), String> {
    let dsl = spacetimedsl::dsl(ctx);

    let round_participants: HashSet<_> = dsl
        .get_participations_by_round_id(round_id)
        .map(|p| p.get_participating_country_id())
        .collect();

    for vote in votes {
        if dsl.get_participating_country_by_id(vote)?.get_country_id() == *from_country_id {
            return Err("Voting for your own country is not allowed".to_string());
        }

        if !round_participants.contains(vote) {
            return Err(
                "Can only vote for countries that participate in the active round".to_string(),
            );
        }
    }

    Ok(())
}

#[spacetimedb::reducer]
fn submit_tele_votes(
    ctx: &ReducerContext,
    votes: Vec<ParticipatingCountryId>,
) -> Result<(), String> {
    let dsl = spacetimedsl::dsl(ctx);

    let user = dsl.get_user_by_identity(&ctx.sender())?;
    let viewer = dsl.get_viewer_by_user_id(&user)?;

    let round_id = dsl.get_active_round()?.get_round_id();

    if votes.len() > AVAILABLE_TELE_VOTES {
        return Err(format!("You can only submit {AVAILABLE_TELE_VOTES} votes"));
    }

    validate_votes(ctx, &round_id, &viewer.get_country_id(), &votes)?;

    dsl.delete_tele_votes_by_viewer_id(&viewer)?;

    for to_country_id in votes {
        dsl.create_tele_vote(CreateTeleVote {
            round_id: round_id.clone(),
            viewer_id: viewer.get_id(),
            to_country_id,
        })?;
    }

    Ok(())
}

#[spacetimedb::reducer]
fn submit_juror_votes(
    ctx: &ReducerContext,
    ranking: Vec<ParticipatingCountryId>,
) -> Result<(), String> {
    let dsl = spacetimedsl::dsl(ctx);

    let round = dsl.get_round_by_id(dsl.get_active_round()?.get_round_id())?;

    if !matches!(round.get_kind(), crate::round::RoundKind::GrandFinal) {
        return Err("Jurors can only vote in the grand final!".to_string());
    }

    let user = dsl.get_user_by_identity(&ctx.sender())?;
    let juror = dsl.get_juror_by_user_id(&user)?;
    let juror_country_id = dsl
        .get_participating_country_by_id(juror.get_participating_country_id())?
        .get_country_id();

    validate_votes(ctx, &round.get_id(), &juror_country_id, &ranking)?;

    dsl.delete_juror_votes_by_juror_id(&juror)?;

    for (i, ranked_country_id) in ranking.into_iter().enumerate() {
        dsl.create_juror_vote(CreateJurorVote {
            round_id: round.get_id(),
            juror_id: juror.get_id(),
            ranked_country_id,
            rank: (i + 1) as u16,
        })?;
    }

    Ok(())
}
