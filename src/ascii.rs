use crate::StlModel;
use crate::error::{Error, Result};

pub fn parse_ascii_stl(_bytes: &[u8]) -> Result<StlModel> {
    Err(Error::ascii("Ascii files not implemented yet"))
}
