use futures::executor::block_on;
use sea_orm::{ConnectionTrait, Database, DbBackend, DbErr, Statement};
mod config;

async fn run() -> Result<(), DbErr> {
    let db_config = config::DBConfig::new().unwrap();

    let db_url = db_config.get_database_url("mysql");
    let db_name = db_config.get_database_name();

    println!("db_url: {} db_name: {}", db_url, db_name);

    let db = Database::connect(&db_url).await?;

    let db = &match db.get_database_backend() {
        DbBackend::MySql => {
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("CREATE DATABASE IF NOT EXISTS `{}`;", db_name),
            ))
            .await?;

            let url = format!("{}/{}", db_url, db_name);
            Database::connect(&url).await?
        }

        DbBackend::Postgres => {
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("DROP DATABASE IF EXISTS \"{}\";", db_name),
            ))
            .await?;
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("CREATE DATABASE \"{}\";", db_name),
            ))
            .await?;

            let url = format!("{}/{}", db_url, db_name);
            Database::connect(&url).await?
        }

        DbBackend::Sqlite => db,
    };

    db.ping().await
}

fn main() {
    if let Err(err) = block_on(run()) {
        panic!("{}", err);
    }
}
