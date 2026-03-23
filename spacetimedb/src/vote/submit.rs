use std::collections::HashSet;

use spacetimedb::{ReducerContext, SpacetimeType, ViewContext};

use crate::{
    country::{
        CountryId, GetParticipatingCountryRowOptionByCountryId,
        GetParticipatingCountryRowOptionById, ParticipatingCountryId,
    },
    round::{
        GetActiveRoundRow, GetParticipationRowsByRoundId, GetRoundRowOptionById, RoundId, RoundKind,
    },
    user::GetUserRowOptionByIdentity,
    voter::{GetJurorRowOptionByUserId, GetViewerRowOptionByUserId},
};

use super::{
    CreateJurorVote, CreateJurorVoteRow, CreateTeleVote, CreateTeleVoteRow,
    DeleteJurorVoteRowsByJurorAndRound, DeleteTeleVoteRowsByViewerAndRound, AVAILABLE_TELE_VOTES,
};

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

    let round = dsl.get_round_by_id(dsl.get_active_round()?.get_round_id())?;

    if !round.get_voting_open() {
        return Err("Voting is closed for this round".to_string());
    }

    if votes.len() > AVAILABLE_TELE_VOTES {
        return Err(format!("You can only submit {AVAILABLE_TELE_VOTES} votes"));
    }

    validate_votes(ctx, &round.get_id(), &viewer.get_country_id(), &votes)?;

    dsl.delete_tele_votes_by_viewer_and_round(&viewer, &round.get_id())?;

    for to_country_id in votes {
        dsl.create_tele_vote(CreateTeleVote {
            round_id: round.get_id(),
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

    let user = dsl.get_user_by_identity(&ctx.sender())?;
    let juror = dsl.get_juror_by_user_id(&user)?;
    let juror_country_id = dsl
        .get_participating_country_by_id(juror.get_participating_country_id())?
        .get_country_id();

    let round = dsl.get_round_by_id(dsl.get_active_round()?.get_round_id())?;

    if !round.get_voting_open() {
        return Err("Voting is closed for this round".to_string());
    }

    if !matches!(round.get_kind(), crate::round::RoundKind::GrandFinal) {
        return Err("Jurors can only vote in the grand final!".to_string());
    }

    validate_votes(ctx, &round.get_id(), &juror_country_id, &ranking)?;

    dsl.delete_juror_votes_by_juror_and_round(&juror, &round)?;

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

#[derive(SpacetimeType, Debug)]
pub struct VotableCountry {
    pub participating_country_id: ParticipatingCountryId,
}

#[spacetimedb::view(accessor = votable_countries, public)]
fn votable_countries(ctx: &ViewContext) -> Vec<VotableCountry> {
    let dsl = spacetimedsl::read_only_dsl(ctx);

    let Ok(active_round) = dsl.get_active_round() else {
        return Vec::default();
    };

    let Ok(user) = dsl.get_user_by_identity(&ctx.sender()) else {
        return Vec::default();
    };

    let disallowed_country = if let Ok(juror) = dsl.get_juror_by_user_id(&user) {
        if *dsl
            .get_round_by_id(active_round.get_round_id())
            .unwrap()
            .get_kind()
            != RoundKind::GrandFinal
        {
            return Vec::default();
        }

        Some(juror.get_participating_country_id())
    } else if let Ok(viewer) = dsl.get_viewer_by_user_id(&user) {
        dsl.get_participating_country_by_country_id(viewer.get_country_id())
            .ok()
            .map(|p| p.get_id())
    } else {
        return Vec::default();
    };

    dsl.get_participations_by_round_id(active_round.get_round_id())
        .filter(|p| Some(p.get_participating_country_id()) != disallowed_country)
        .map(|p| VotableCountry {
            participating_country_id: p.get_participating_country_id(),
        })
        .collect()
}
