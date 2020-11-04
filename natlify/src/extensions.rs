use anyhow::{format_err, Result};
use surf::Error as SurfError;

pub trait SurfErrorExt<T> {
    fn anyhow(self) -> Result<T>;
}

impl<T> SurfErrorExt<T> for Result<T, SurfError> {
    fn anyhow(self) -> Result<T> {
        match self {
            Ok(v) => Ok(v),
            Err(e) => Err(format_err!("{}", e)),
        }
    }
}
