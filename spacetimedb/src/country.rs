#[spacetimedsl::dsl(plural_name = countries, method(update = false, delete = false))]
#[spacetimedb::table(accessor = country, public)]
pub struct Country {
    #[primary_key]
    #[auto_inc]
    #[create_wrapper]
    #[referenced_by(path = crate::voter, table = rep)]
    id: u16,

    name: String,

    code: String,
}

#[spacetimedb::reducer(init)]
fn init(ctx: &spacetimedb::ReducerContext) {
    let dsl = spacetimedsl::dsl(ctx);
    let countries = [
        ("Sweden", "SE"),
        ("Switzerland", "CH"),
        ("France", "FR"),
        ("Germany", "DE"),
        ("Italy", "IT"),
        ("Spain", "ES"),
        ("United Kingdom", "GB"),
        ("Norway", "NO"),
        ("Finland", "FI"),
        ("Ukraine", "UA"),
    ];
    for (name, code) in countries {
        dsl.create_country(CreateCountry {
            name: name.to_string(),
            code: code.to_string(),
        })
        .expect("Failed to seed country");
    }
}
