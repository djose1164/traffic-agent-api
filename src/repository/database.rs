use dotenv::dotenv;
use diesel::r2d2::{ConnectionManager, Pool, PoolError};
use diesel::mysql::MysqlConnection;
use std::env;

pub type DbConnection = MysqlConnection;
pub type DbPool = Pool<ConnectionManager<DbConnection>>;
pub type DbError = Box<dyn std::error::Error + Send + Sync>;
pub type DbBackend = diesel::mysql::Mysql;

fn init_pool(database_url: &str) -> Result<DbPool, PoolError> {
    let manager = ConnectionManager::<DbConnection>::new(database_url);
    Pool::builder().build(manager)
}

pub fn establish_connection() -> Result<DbPool, PoolError> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("Couldn't find the database's url.");
    init_pool(&database_url)
}