use sea_orm::Database;

#[tokio::main]
pub async fn run(database_uri: String) {
    let database = Database::connect(database_uri).await;

}