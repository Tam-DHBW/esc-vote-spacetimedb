#[spacetimedsl::dsl(plural_name = countries, method(update = false, delete = false))]
#[spacetimedb::table(accessor = country, public)]
pub struct Country {
    #[primary_key]
    #[auto_inc]
    #[create_wrapper]
    #[referenced_by(path = crate::voter, table = viewer)]
    #[referenced_by(path = crate::country, table = participating_country)]
    #[referenced_by(path = crate::country, table = rotw_country)]
    #[referenced_by(path = crate::ranking, table = ranking)]
    id: u16,

    name: String,

    emoji: Option<String>,
}

/// A country participating in the contest (excludes "Rest of the World").
#[spacetimedsl::dsl(
    plural_name = participating_countries,
    method(update = false, delete = true),
)]
#[spacetimedb::table(accessor = participating_country, public)]
pub struct ParticipatingCountry {
    #[primary_key]
    #[auto_inc]
    #[create_wrapper]
    #[referenced_by(path = crate::voter, table = juror)]
    #[referenced_by(path = crate::round, table = participation)]
    #[referenced_by(path = crate::vote, table = tele_vote)]
    #[referenced_by(path = crate::vote, table = juror_vote)]
    #[referenced_by(path = crate::ranking, table = ranking)]
    id: u16,

    #[use_wrapper(CountryId)]
    #[unique]
    #[foreign_key(path = crate::country, table = country, column = id, on_delete = Delete)]
    country_id: u16,
}

/// Rest of the World
#[spacetimedsl::dsl(singleton, method(update = false, delete = true))]
#[spacetimedb::table(accessor = rotw_country, public)]
pub struct RotwCountry {
    #[use_wrapper(CountryId)]
    #[foreign_key(path = crate::country, table = country, column = id, on_delete = Delete)]
    country_id: u16,
}

#[spacetimedb::reducer(init)]
fn init(ctx: &spacetimedb::ReducerContext) {
    let dsl = spacetimedsl::dsl(ctx);

    let countries = [
        ("Sweden", "🇸🇪", false),
        ("Switzerland", "🇨🇭", false),
        ("France", "🇫🇷", false),
        ("Germany", "🇩🇪", false),
        ("Italy", "🇮🇹", false),
        ("Spain", "🇪🇸", false),
        ("United Kingdom", "🇬🇧", false),
        ("Norway", "🇳🇴", false),
        ("Finland", "🇫🇮", false),
        ("Ukraine", "🇺🇦", false),
        ("Rest of the World", "🌍", true),
    ];

    for (name, emoji, is_rotw) in countries {
        let country = dsl
            .create_country(CreateCountry {
                name: name.to_string(),
                emoji: Some(emoji.to_string()),
            })
            .expect("Failed to create country");

        if !is_rotw {
            dsl.create_participating_country(CreateParticipatingCountry {
                country_id: country.get_id(),
            })
            .expect("Unable to create participating country");
        } else {
            dsl.create_rotw_country(CreateRotwCountry {
                country_id: country.get_id(),
            })
            .expect("Unable to create ROTW country");
        }
    }
}
