use macro_rules_attribute::apply;
use spacetimedb::{SpacetimeType, ViewContext};
use spacetimedsl::dsl;

use crate::country::CountryId;

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

    countries: Vec<CountryId>,
}

#[apply(crate::singleton!)]
#[spacetimedsl::dsl(plural_name = active_rounds, method(update = false, delete = true))]
#[spacetimedb::table(accessor = active_round, public)]
pub struct ActiveRound {
    #[use_wrapper(RoundId)]
    #[index(btree)]
    #[foreign_key(path=crate::round, table=round, column = id, on_delete = Delete)]
    round_id: u16,
}

#[spacetimedb::view(accessor = active_round_view, public)]
fn active_round_view(ctx: &ViewContext) -> Option<ActiveRound> {
    let dsl = spacetimedsl::read_only_dsl(ctx);

    dsl.get_active_round_by_singleton(()).ok()
}
