use std::env;

pub struct RdbmsConfig {
    pub db_user: String,
    pub db_pass: String,
    pub db_host: String,
    pub db_port: String,
    pub db_name: String,
    pub external_lib: String,
    pub db_driver: String,
    pub db_store: String,
}

pub fn rdbms_config_from_env() -> RdbmsConfig {
    return RdbmsConfig {
        db_user: env::var("DBUSER").unwrap_or(String::from("")),
        db_pass: env::var("DBPASS").unwrap_or(String::from("")),
        db_host: env::var("DBHOST").unwrap_or(String::from("")),
        db_port: env::var("DBPORT").unwrap_or(String::from("")),
        db_name: env::var("DBNAME").unwrap_or(String::from("")),
        external_lib: env::var("EXTERNAL_LIB").unwrap_or(String::from("")),
        db_driver: env::var("DBDRIVER").unwrap_or(String::from("")),
        db_store: env::var("DBSTORE").unwrap_or(String::from("")),
    }
}
