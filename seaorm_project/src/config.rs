use dotenv::dotenv;
use std::env;

// the whole database URL string follows the following format:
// "protocol://username:password@host:port/database"
// We put the database name (that last bit) in a separate variable simply for convenience.

pub struct DBConfig {
    username: String,
    password: String,
    host: String,
    port: String,
    db_name: String,
}

impl DBConfig {
    pub fn new() -> Result<Self, String> {
        dotenv().ok();

        let username = env::var("DATABASE_USERNAME").unwrap_or_else(|_| "root".to_owned());
        let password = env::var("DATABASE_PASSWORD").unwrap_or_else(|_| "123456".to_owned());
        let host = env::var("DATABASE_host").unwrap_or_else(|_| "localhost".to_owned());
        let port = env::var("DATABASE_port").unwrap_or_else(|_| "3306".to_owned());
        let db_name = env::var("DB_NAME").unwrap_or_else(|_| "rust_db".to_owned());

        Ok(Self {
            username,
            password,
            host,
            port,
            db_name,
        })
    }

    pub fn get_database_url(&self, protocol: &str) -> String {
        format!(
            "{}://{}:{}@{}:{}",
            protocol, self.username, self.password, self.host, self.port
        )
    }

    pub fn get_database_name(&self) -> String {
        format!("{}", self.db_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_database_url() {
        let db_config = DBConfig::new().unwrap();

        let expected = format!(
            "{}://{}:{}@{}:{}",
            "mysql",
            env::var("DATABASE_USERNAME").unwrap_or_else(|_| "root".to_owned()),
            env::var("DATABASE_PASSWORD").unwrap_or_else(|_| "123456".to_owned()),
            env::var("DATABASE_host").unwrap_or_else(|_| "localhost".to_owned()),
            env::var("DATABASE_port").unwrap_or_else(|_| "3306".to_owned())
        );

        assert_eq!(expected, db_config.get_database_url("mysql"));
    }

    #[test]
    fn test_get_database_name() {
        let db_config = DBConfig::new().unwrap();

        assert_eq!(
            env::var("DB_NAME").unwrap_or_else(|_| "rust_db".to_owned()),
            db_config.get_database_name()
        );
    }
}
