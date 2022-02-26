use diesel::Connection;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};
use dotenv::dotenv;
use std::env;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub type DbPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub enum DatabaseKind { 
    ProductDb, 
    ExampleTest
}


fn init_pool(database_url: &str) -> Result<DbPool, PoolError> {
    //  Connection manager for user with Diesel 
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    //  Returns a new connection manager, which establishes connections to the given database URL
    //  Returns a builder type to configure a new pool
    Pool::builder().build(manager)
    //  Consumers the builder, returning a new, initialised pool 
    //  This will consume connection manager
}

pub fn establish_connection(db_kind: DatabaseKind) -> DbPool {
    dotenv().ok();

    let postgres_db_host = 
        env::var("POSTGRES_DB_HOST")
        .expect("POSTGRES_DB_HOST must be set");

    let postgres_db = match db_kind {
        DatabaseKind::ProductDb => 
            env::var("POSTGRES_DB")
            .expect("POSTGRES_DB must be set"),
        _ => env::var("POSTGRES_DB_TEST")
            .expect("POSTGRES_DB_TEST must be set"),
    };

    let postgres_user = 
        env::var("POSTGRES_USER")
        .expect("POSTGRES_USER must be set");
    let postgres_password = 
        env::var("POSTGRES_PASSWORD")
        .expect("POSTGRES_PASSWORD must be set");

    // Load the DATABASE_URL env variable into database_url, in case of error
    // it will through a message "DATABASE_URL must be set"
    let database_url = format!(
        "postgres://{}:{}@{}/{}",
        postgres_user, postgres_password, postgres_db_host, postgres_db
    );

    init_pool(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

// pub fn establish_connection() -> PgConnection {
//     dotenv().ok(); // Grab ENV vars from `.env`

//     // Pull value from `DATABASE_URL` ENV var
//     let database_url = env::var("DATABASE_URL")
//         .expect("DATABASE_URL must be set");

//     // Establishes a connection to the DB
//     // https://docs.diesel.rs/diesel/connection/trait.Connection.html#tymethod.establish

// }