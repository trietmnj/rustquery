use std::error::Error;

use postgres::Error;

use crate::{
    pgxdb::new_pgx_connection,
    config::RdbmsConfig,
    datastore::DataStore
};

struct RdbmsStore {
    db: DataStore,
}

pub async fn new_rdbms_store(c: &RdbmsConfig) -> Result<Box<dyn DataStore>, Box<dyn Error>> {
    match c.db_store {
        "pgx" => {
            let store = new_pgx_connection(c).await?;
            let mut store = match store {
                Ok(s) => s,
                Err(e) => panic!(format!("Unable to connect to pgx datastore: {}", e)),
                    // Error(format!("Unable to connect to pgx datastore: %s", e)),
            };
        }
        "sqlx" => {
            let store = new_sqlx_connection(c);
            let mut store = match store {
                Ok(s) => s,
                Err(e) => panic!(format!("Unable to connect to pgx datastore: {}", e)),
            }
        }
        _ => panic!(format!("Unsupported store type: {}", c.db_store)),
    }
}

impl RdbmsStore {
    fn rdbms_db(&self) -> &mut DataStore {
        &mut self.db
    }
}

impl DataSet for RdbmsStore {
    fn connection(&self) -> Any {
        self.db.connection()
    }

    fn transaction(&self) -> Result<Box<Tx>, Box<Error>> {
        self.db.transaction()
    }

    // fn fetch(tx: Box<Tx>, qi: QueryInput, dest: Any) -> Result<(), Error> {
    //     let statement = get_select_statement(qi.dataset, qi.key, qi.statement, qi.suffix, )
    // }
}
