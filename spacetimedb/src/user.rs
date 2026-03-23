use spacetimedb::Identity;

#[spacetimedsl::dsl(plural_name = users, method(update = false, delete = false))]
#[spacetimedb::table(accessor = user)]
pub struct User {
    #[primary_key]
    #[auto_inc]
    #[create_wrapper]
    #[referenced_by(path = crate::voter, table = viewer)]
    #[referenced_by(path = crate::voter, table = juror)]
    id: u64,

    #[unique]
    identity: Identity,
}
