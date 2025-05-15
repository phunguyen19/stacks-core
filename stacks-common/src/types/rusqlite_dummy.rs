// This is a dummy module to satisfy imports when rusqlite is not available
// It provides empty implementations of the types and traits used in the codebase

#[derive(Debug)]
pub struct Error;

pub mod types {
    use super::Error;

    #[derive(Debug)]
    pub enum ValueRef<'a> {
        Null,
        Integer(i64),
        Real(f64),
        Text(&'a str),
        Blob(&'a [u8]),
    }

    #[derive(Debug)]
    pub enum ToSqlOutput<'a> {
        Borrowed(ValueRef<'a>),
        Owned(ValueRef<'static>),
    }

    pub trait FromSql: Sized {
        fn column_result(_value: ValueRef) -> FromSqlResult<Self> {
            Err(FromSqlError::InvalidType)
        }
    }

    #[derive(Debug)]
    pub enum FromSqlError {
        InvalidType,
        OutOfRange,
        InvalidBlobSize,
        Other(Box<dyn std::error::Error + Send + Sync>),
    }

    pub type FromSqlResult<T> = Result<T, FromSqlError>;

    pub trait ToSql {
        fn to_sql(&self) -> Result<ToSqlOutput, Error>;
    }

    impl<'a> From<&'a str> for ToSqlOutput<'a> {
        fn from(s: &'a str) -> Self {
            ToSqlOutput::Borrowed(ValueRef::Text(s))
        }
    }
}

#[derive(Debug)]
pub struct Connection;

#[derive(Debug)]
pub enum ErrorCode {
    CannotOpen,
}

#[derive(Debug)]
pub struct SqliteFailure {
    pub code: ErrorCode,
    pub extended_code: i32,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Dummy SQLite error")
    }
}

impl std::error::Error for Error {}

#[derive(Debug)]
pub enum OpenFlags {
    SQLITE_OPEN_READ_WRITE,
    SQLITE_OPEN_READ_ONLY,
    SQLITE_OPEN_CREATE,
}

pub fn params(_: &str) -> Vec<&str> {
    vec![]
}

pub const NO_PARAMS: &[&dyn types::ToSql] = &[];
