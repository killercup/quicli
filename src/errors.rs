use std::fmt::{Debug, Display};
use std::result::Result as StdResult;
use failure::{Error, err_msg};

/// A handy alias for `Result` that carries a generic error type.
pub type Result<T> = StdResult<T, Error>;

/// Treat `Option::None` as Error with context
pub trait NoneErrorContext<T> {
    /// Convert Option to Result by annotating what None means
    ///
    /// # Examples
    ///
    /// ```rust
    /// # extern crate quicli;
    /// # use quicli::prelude::*;
    /// # fn main() { assert!(run().is_err()); }
    /// # fn run() -> Result<()> {
    /// let xs = vec!["lorem", "ipsum"];
    /// let x = xs.get(66); // will return None
    /// let result = x.none_means("index not found")?;
    /// # Ok(()) }
    /// ```
    fn none_means<E: Display + Debug + Sync + Send + 'static>(self, explanation: E) -> Result<T>;
}

impl<T> NoneErrorContext<T> for Option<T> {
    fn none_means<E: Display + Debug + Sync + Send + 'static>(self, explanation: E) -> Result<T> {
        match self {
            Some(x) => Ok(x),
            None => Err(err_msg(explanation)),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn none_means_error_message() {
        use prelude::*;
        run().unwrap();

        fn run() -> Result<()> {
            let xs = vec!["lorem", "ipsum"];
            let x = xs.get(66); // will return None

            let result = x.none_means("index not found");

            assert!(result.is_err());
            if let Err(error) = result {
                assert_eq!(error.to_string(), "index not found".to_string());
            }

            Ok(())
        }
    }
}
