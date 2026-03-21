use spacetimedb::{Identity, ReducerContext, SpacetimeType, ViewContext};

use crate::country::CountryId;

impl VoterRole {
    fn country_id(&self) -> Option<&CountryId> {
        match self {
            VoterRole::Rep(r) => Some(&r.country_id),
            VoterRole::Juror(j) => Some(&j.country_id),
            VoterRole::World => None,
        }
    }
}

/// Any kind of voter: rest-of-the-world, country representative, or country juror.
#[spacetimedsl::dsl(plural_name = voters, method(update = false))]
#[spacetimedb::table(accessor = voter)]
pub struct Voter {
    #[primary_key]
    #[auto_inc]
    #[create_wrapper]
    #[referenced_by(path = crate::voter, table = rep)]
    #[referenced_by(path = crate::vote, table = tele_vote)]
    id: u64,

    #[unique]
    #[index(btree)]
    identity: Identity,
}

/// Voter representing a participating country.
#[spacetimedsl::dsl(plural_name = reps, method(update = false, delete = true))]
#[spacetimedb::table(accessor = rep)]
pub struct Rep {
    #[primary_key]
    #[auto_inc]
    #[create_wrapper]
    #[referenced_by(path = crate::voter, table = juror)]
    id: u64,

    #[unique]
    #[use_wrapper(VoterId)]
    #[index(btree)]
    #[foreign_key(path = crate::voter, table = voter, column = id, on_delete = Delete)]
    voter_id: u64,

    #[use_wrapper(crate::country::CountryId)]
    #[index(btree)]
    #[foreign_key(path = crate::country, table = country, column = id, on_delete = Delete)]
    country_id: u16,
}

/// Jury member of a participating country.
#[spacetimedsl::dsl(plural_name = jurors, method(update = false, delete = true))]
#[spacetimedb::table(accessor = juror)]
pub struct Juror {
    #[primary_key]
    #[auto_inc]
    #[create_wrapper]
    #[referenced_by(path = crate::vote, table = juror_vote)]
    id: u64,

    #[unique]
    #[use_wrapper(RepId)]
    #[index(btree)]
    #[foreign_key(path = crate::voter, table = rep, column = id, on_delete = Delete)]
    rep_id: u64,

    name: String,
}

#[derive(SpacetimeType, Debug)]
pub struct RepInfo {
    pub country_id: CountryId,
}

#[derive(SpacetimeType, Debug)]
pub struct JurorInfo {
    pub country_id: CountryId,
    pub name: String,
}

#[derive(SpacetimeType, Debug)]
pub enum VoterRole {
    World,
    Rep(RepInfo),
    Juror(JurorInfo),
}

#[derive(SpacetimeType, Debug)]
pub struct CurrentVoter {
    pub role: VoterRole,
}

#[spacetimedb::reducer]
pub fn register(ctx: &ReducerContext, role: VoterRole) -> Result<(), String> {
    let dsl = spacetimedsl::dsl(ctx);

    if dsl.get_voter_by_identity(&ctx.sender()).is_ok() {
        return Err("Already registered".to_string());
    }

    let voter = dsl.create_voter(CreateVoter {
        identity: ctx.sender(),
    })?;

    if let Some(country_id) = role.country_id() {
        let rep = dsl.create_rep(CreateRep {
            voter_id: voter.get_id().clone(),
            country_id: country_id.clone(),
        })?;

        if let VoterRole::Juror(info) = &role {
            dsl.create_juror(CreateJuror {
                rep_id: rep.get_id().clone(),
                name: info.name.clone(),
            })?;
        }
    }

    Ok(())
}

#[spacetimedb::view(accessor = current_voter, public)]
fn current_voter(ctx: &ViewContext) -> Option<CurrentVoter> {
    let dsl = spacetimedsl::read_only_dsl(ctx);

    let voter = dsl.get_voter_by_identity(&ctx.sender()).ok()?;
    let rep = dsl.get_rep_by_voter_id(voter.get_id()).ok();

    let role = match rep {
        None => VoterRole::World,

        Some(rep) => match dsl.get_juror_by_rep_id(rep.get_id()).ok() {
            None => VoterRole::Rep(RepInfo {
                country_id: rep.get_country_id().clone(),
            }),

            Some(juror) => VoterRole::Juror(JurorInfo {
                country_id: rep.get_country_id().clone(),
                name: juror.get_name().clone(),
            }),
        },
    };

    Some(CurrentVoter { role })
}
