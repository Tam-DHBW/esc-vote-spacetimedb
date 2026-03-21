use spacetimedb::ReducerContext;
use spacetimedsl::dsl;

use crate::{
    country::CountryId,
    round::GetActiveRoundRow,
    voter::{GetJurorRowOptionByRepId, GetRepRowOptionByVoterId, GetVoterRowOptionByIdentity},
};

const AVAILABLE_TELE_VOTES: usize = 20;

/// A single televote tap. Each voter (World/Rep) can have up to 20 rows per round.
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

    #[use_wrapper(crate::voter::VoterId)]
    #[index(btree)]
    #[foreign_key(path = crate::voter, table = voter, column = id, on_delete = Delete)]
    voter_id: u64,

    #[use_wrapper(crate::country::CountryId)]
    #[index(btree)]
    #[foreign_key(path = crate::country, table = country, column = id, on_delete = Delete)]
    country_id: u16,
}

/// A juror's rank for one country in a round. Unique on (juror_id, round_id, rank).
#[dsl(
    plural_name = juror_votes,
    method(update = false, delete = true),
    unique_index(name = juror_round_rank)
)]
#[spacetimedb::table(
    accessor = juror_vote,
    index(accessor = juror_round_rank, btree(columns = [juror_id, round_id, rank]))
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

    #[use_wrapper(crate::country::CountryId)]
    #[index(btree)]
    #[foreign_key(path = crate::country, table = country, column = id, on_delete = Delete)]
    country_id: u16,

    rank: u16,
}

#[spacetimedb::reducer]
fn submit_tele_votes(ctx: &ReducerContext, votes: Vec<CountryId>) -> Result<(), String> {
    let dsl = spacetimedsl::dsl(ctx);

    if votes.len() > AVAILABLE_TELE_VOTES {
        return Err(format!("You can only submit {AVAILABLE_TELE_VOTES} votes"));
    }

    let round = dsl.get_active_round()?;
    let voter = dsl.get_voter_by_identity(&ctx.sender())?;

    dsl.delete_tele_votes_by_voter_id(&voter)?;

    for vote in votes {
        dsl.create_tele_vote(CreateTeleVote {
            round_id: round.get_round_id(),
            voter_id: voter.get_id(),
            country_id: vote.clone(),
        })?;
    }

    Ok(())
}

#[spacetimedb::reducer]
fn submit_juror_votes(ctx: &ReducerContext, ranking: Vec<CountryId>) -> Result<(), String> {
    let dsl = spacetimedsl::dsl(ctx);

    let round = dsl.get_active_round()?;
    let voter = dsl.get_voter_by_identity(&ctx.sender())?;
    let rep = dsl.get_rep_by_voter_id(voter.get_id())?;
    let juror = dsl.get_juror_by_rep_id(rep.get_id())?;

    if ranking.iter().any(|c| c == &rep.get_country_id()) {
        return Err("Jurors cannot vote for their own country".to_string());
    }

    dsl.delete_juror_votes_by_juror_id(&juror)?;

    for (i, country_id) in ranking.into_iter().enumerate() {
        dsl.create_juror_vote(CreateJurorVote {
            round_id: round.get_round_id(),
            juror_id: juror.get_id(),
            country_id,
            rank: (i + 1) as u16,
        })?;
    }

    Ok(())
}
