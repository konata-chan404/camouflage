use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct SqliteFormat;

impl Detector for SqliteFormat {
    fn detect(&self, data: &[u8], _file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::archive::is_sqlite(data))
    }
}
impl Validator for SqliteFormat {
}
