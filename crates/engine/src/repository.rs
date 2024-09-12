use crate::net::Net;
use crate::specification::Specification;
use crate::version::Version;
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
            Err(err) => Err(PersistenceError::NotFound),
        }
    }
}


#[test]
fn test_initialize_repository() {
    let repo = Repository::new();
    assert!(repo.is_ok());
}


#[test]
fn test_create_specification() {
    let repo = Repository::new().unwrap();
    let spec = Specification::new(
        "spec-id".to_string(),
        "spec-name".to_string(),
        Version::parse("1.1".to_string()).unwrap(),
        "some descriptoin".to_string(),
        Net::new());
    let res = repo.create_specification(&spec);
    assert!(res.is_ok())
}
