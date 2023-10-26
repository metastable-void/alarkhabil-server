
use core::ops::Deref;


#[derive(Debug)]
pub struct RusqliteConnection {
    connection: rusqlite::Connection,
}

pub struct RusqliteTransaction<'a> {
    transaction: rusqlite::Transaction<'a>,
}

impl RusqliteConnection {
    pub fn new(connection: rusqlite::Connection) -> RusqliteConnection {
        RusqliteConnection {
            connection,
        }
    }

    /// Passing an empty db_path is equivalent to passing None.
    pub fn open(db_path: Option<&str>, init_queries: Option<&str>) -> Result<RusqliteConnection, rusqlite::Error> {
        let db_path = if let Some(db_path) = db_path {
            if db_path.is_empty() {
                None
            } else {
                Some(db_path)
            }
        } else {
            None
        };

        let mut conn = match db_path {
            Some(db_path) => rusqlite::Connection::open(db_path)?,
            None => rusqlite::Connection::open_in_memory()?,
        };

        if let Some(init_queries) = init_queries {
            let init_tx = conn.transaction()?;
            init_tx.execute_batch(init_queries)?;
            init_tx.commit()?;
        }

        Ok(RusqliteConnection::new(conn))
    }

    pub fn transaction(&mut self) -> rusqlite::Result<RusqliteTransaction> {
        let transaction = self.connection.transaction()?;
        Ok(RusqliteTransaction {
            transaction,
        })
    }
}

impl<'a> Deref for RusqliteTransaction<'a> {
    type Target = rusqlite::Connection;

    fn deref(&self) -> &Self::Target {
        &self.transaction
    }
}

impl<'a> RusqliteTransaction<'a> {
    pub fn commit(self) -> rusqlite::Result<()> {
        self.transaction.commit()
    }

    pub fn rollback(self) -> rusqlite::Result<()> {
        self.transaction.rollback()
    }
}

