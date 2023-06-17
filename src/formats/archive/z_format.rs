use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct ZFormat;

impl Detector for ZFormat {
    fn detect(&self, data: &[u8], _file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::archive::is_z(data))
    }
}
impl Validator for ZFormat {
}
