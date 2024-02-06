use sheets_database::entity::Entity;
use sheets_database_derive::Entity;

#[derive(Entity)]
struct ExampleString {
    test: String,
    test2: String,
}
