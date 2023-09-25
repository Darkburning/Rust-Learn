use futures::executor::block_on;
use sea_orm::*;
mod config;
mod entities;
use entities::{prelude::*, *};

async fn run() -> Result<(), DbErr> {
    let db = connect().await.unwrap();
    ping(db.clone()).await?;

    // insert
    let happy_bakery = bakery::ActiveModel {
        name: ActiveValue::Set("Happy Bakery".to_owned()),
        profit_margin: ActiveValue::Set(0.0),
        ..Default::default()
    };
    let res = Bakery::insert(happy_bakery).exec(&db).await?;

    // // update
    let sad_bakery = bakery::ActiveModel {
        id: ActiveValue::Set(res.last_insert_id),
        name: ActiveValue::Set("Sad Bakery".to_owned()),
        profit_margin: ActiveValue::NotSet,
    };
    sad_bakery.update(&db).await?;

    // // insert
    let john = chef::ActiveModel {
        name: ActiveValue::Set("John".to_owned()),
        bakery_id: ActiveValue::Set(res.last_insert_id), // a foreign key
        ..Default::default()
    };
    Chef::insert(john).exec(&db).await?;

    // delete
    let john = chef::ActiveModel {
        id: ActiveValue::Set(2), // The primary key must be set
        ..Default::default()
    };
    john.delete(&db).await?;

    let sad_bakery = bakery::ActiveModel {
        id: ActiveValue::Set(2), // The primary key must be set
        ..Default::default()
    };
    sad_bakery.delete(&db).await?;

    let bakeries: Vec<bakery::Model> = Bakery::find().all(&db).await?;
    assert!(bakeries.is_empty());

    Ok(())
}

async fn connect() -> Result<DatabaseConnection, DbErr> {
    let db_config = config::DBConfig::new().unwrap();

    let db_url = db_config.get_database_url("mysql");
    let db_name = db_config.get_database_name();

    println!("db_url: {} db_name: {}", db_url, db_name);

    let db = Database::connect(&db_url).await?;

    let db: &sea_orm::DatabaseConnection = &match db.get_database_backend() {
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

    Ok(db.to_owned())
}

async fn ping(db: sea_orm::DatabaseConnection) -> Result<(), DbErr> {
    db.ping().await
}
fn main() {
    if let Err(err) = block_on(run()) {
        panic!("{}", err);
    }
}
