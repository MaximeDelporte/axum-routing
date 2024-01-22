use dotenvy::dotenv;
use data::run;

fn main() {
    dotenv().ok();
    let database_uri = dotenvy::var("DATABASE_URL").unwrap();
    run(database_uri);
}
