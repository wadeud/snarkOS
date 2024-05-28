// Copyright headers...

#![forbid(unsafe_code)]

#[macro_use]
extern crate thiserror;

pub mod commands;
pub mod helpers;

/// Errors that can be encountered in this crate
pub mod error {
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum MyError {
        #[error("An error occurred: {0}")]
        GeneralError(String),
        #[error("Failed due to IO error")]
        IOError(#[from] std::io::Error),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_some_functionality() {
        assert_eq!(2 + 2, 4);
    }
}
