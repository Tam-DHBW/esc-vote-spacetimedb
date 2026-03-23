use spacetimedb::{SpacetimeType, ViewContext};
use spacetimedsl::dsl;

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
    #[referenced_by(path = crate::vote, table = tele_vote)]
    #[referenced_by(path = crate::vote, table = juror_vote)]
    #[referenced_by(path = crate::round, table = active_round)]
    id: u16,

    year: u16,

    kind: RoundKind,
}

#[spacetimedsl::dsl(singleton, method(update = false, delete = true))]
#[spacetimedb::table(accessor = active_round)]
pub struct ActiveRound {
    #[foreign_key(path=crate::round, table=round, column = id, on_delete = Delete)]
    #[use_wrapper(RoundId)]
    round_id: u16,
}

#[spacetimedb::view(accessor = active_round, public)]
fn active_round_view(ctx: &ViewContext) -> Option<Round> {
    let dsl = spacetimedsl::read_only_dsl(ctx);

    let round_id = dsl.get_active_round().ok()?.get_round_id();
    dsl.get_round_by_id(round_id).ok()
}
