// This is a dummy module to satisfy imports when rusqlite is not available
// It provides empty implementations of the types and traits used in the codebase

use stacks_common::types::chainstate::StacksBlockId;

use crate::vm::errors::{InterpreterError, InterpreterResult as Result};

const SQL_FAIL_MESSAGE: &str = "PANIC: SQL Failure in Smart Contract VM.";

#[derive(Debug)]
pub struct SqliteConnection {
    conn: (),
}

pub struct MemoryBackingStore {
    side_store: (),
}

impl MemoryBackingStore {
    pub fn get_side_store(&self) -> &() {
        &self.side_store
    }
}

impl SqliteConnection {
    pub fn put(_conn: &(), _key: &str, _value: &str) -> Result<()> {
        Err(InterpreterError::DBError(SQL_FAIL_MESSAGE.into()).into())
    }

    pub fn get(_conn: &(), _key: &str) -> Result<Option<String>> {
        Err(InterpreterError::DBError(SQL_FAIL_MESSAGE.into()).into())
    }

    pub fn insert_metadata(
        _conn: &(),
        _bhh: &StacksBlockId,
        _contract_hash: &str,
        _key: &str,
        _value: &str,
    ) -> Result<()> {
        Err(InterpreterError::DBError(SQL_FAIL_MESSAGE.into()).into())
    }
}
