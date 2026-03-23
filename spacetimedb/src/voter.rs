use spacetimedb::{ReducerContext, SpacetimeType, ViewContext};

use crate::{
    country::{CountryId, ParticipatingCountryId},
    user::{CreateUser, CreateUserRow, GetUserRowOptionByIdentity},
};

/// A televote viewer, tied to a country.
#[spacetimedsl::dsl(plural_name = viewers, method(update = false, delete = true))]
#[spacetimedb::table(accessor = viewer)]
pub struct Viewer {
    #[primary_key]
    #[auto_inc]
    #[create_wrapper]
    #[referenced_by(path = crate::vote, table = tele_vote)]
    id: u64,

    #[use_wrapper(crate::user::UserId)]
    #[unique]
    #[foreign_key(path = crate::user, table = user, column = id, on_delete = Delete)]
    user_id: u64,

    #[use_wrapper(crate::country::CountryId)]
    #[index(btree)]
    #[foreign_key(path = crate::country, table = country, column = id, on_delete = Delete)]
    country_id: u16,
}

/// A juror for a participating country.
#[spacetimedsl::dsl(plural_name = jurors, method(update = false, delete = true))]
#[spacetimedb::table(accessor = juror, public)]
pub struct Juror {
    #[primary_key]
    #[auto_inc]
    #[create_wrapper]
    #[referenced_by(path = crate::vote, table = juror_vote)]
    id: u64,

    #[use_wrapper(crate::user::UserId)]
    #[unique]
    #[foreign_key(path = crate::user, table = user, column = id, on_delete = Delete)]
    user_id: u64,

    #[use_wrapper(crate::country::ParticipatingCountryId)]
    #[index(btree)]
    #[foreign_key(path = crate::country, table = participating_country, column = id, on_delete = Delete)]
    participating_country_id: u16,

    name: String,
}

#[derive(SpacetimeType, Debug)]
pub struct ViewerInfo {
    pub country_id: CountryId,
}

#[derive(SpacetimeType, Debug)]
pub struct JurorInfo {
    pub participating_country_id: ParticipatingCountryId,
    pub name: String,
}

#[derive(SpacetimeType, Debug)]
pub enum VoterRole {
    Viewer(ViewerInfo),
    Juror(JurorInfo),
}

#[derive(SpacetimeType, Debug)]
pub struct CurrentUser {
    pub role: VoterRole,
}

#[spacetimedb::reducer]
pub fn register(ctx: &ReducerContext, role: VoterRole) -> Result<(), String> {
    let dsl = spacetimedsl::dsl(ctx);

    let user = dsl.create_user(CreateUser {
        identity: ctx.sender(),
    })?;

    match role {
        VoterRole::Viewer(ViewerInfo { country_id }) => {
            dsl.create_viewer(CreateViewer {
                user_id: user.get_id(),
                country_id,
            })?;
        }
        VoterRole::Juror(JurorInfo {
            participating_country_id,
            name,
        }) => {
            dsl.create_juror(CreateJuror {
                user_id: user.get_id(),
                participating_country_id,
                name,
            })?;
        }
    }

    Ok(())
}

#[spacetimedb::view(accessor = current_user, public)]
fn current_user(ctx: &ViewContext) -> Option<CurrentUser> {
    let dsl = spacetimedsl::read_only_dsl(ctx);

    let user = dsl.get_user_by_identity(&ctx.sender()).ok()?;

    if let Ok(juror) = dsl.get_juror_by_user_id(&user) {
        return Some(CurrentUser {
            role: VoterRole::Juror(JurorInfo {
                participating_country_id: juror.get_participating_country_id(),
                name: juror.get_name().clone(),
            }),
        });
    }

    if let Ok(viewer) = dsl.get_viewer_by_user_id(&user) {
        return Some(CurrentUser {
            role: VoterRole::Viewer(ViewerInfo {
                country_id: viewer.get_country_id(),
            }),
        });
    }

    None
}
