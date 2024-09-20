use crate::specification::Specification;
use rusqlite::{params, Connection};

/// This enum represents possible errors that can occur during persistence operations.
#[derive(Debug)]
pub enum PersistenceError {
    /// Indicates that the requested item was not found.
    NotFound,
    /// Represents an error in the connection, with an accompanying message.
    ConnectionError(String),
}

impl std::fmt::Display for PersistenceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Repository {
    conn: Connection,
}

impl Repository {
    pub fn new() -> Result<Self, PersistenceError> {
        let repository = match Connection::open_in_memory() {
            Ok(conn) => Repository { conn },
            Err(_) => return Err(PersistenceError::ConnectionError("Could not open database".to_string())),
        };
        match repository.conn.execute("CREATE TABLE specifications (
            id INTEGER PRIMARY KEY,
            name TEXT
        )", ()) {
            Ok(_) => Ok(repository),
            Err(err) => Err(PersistenceError::ConnectionError(err.to_string())),
        }
    }

    pub fn create_specification(&self, spec: &Specification) -> Result<(), PersistenceError> {
        match self.conn.execute("INSERT INTO specifications (name) VALUES (?1)", params![&spec.name]) {
            Ok(_) => Ok(()),
            Err(_) => Err(PersistenceError::NotFound),
        }
    }
}

