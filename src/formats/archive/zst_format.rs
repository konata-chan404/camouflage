use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct ZstFormat;

impl Detector for ZstFormat {
    fn detect(&self, data: &[u8], _file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::archive::is_zst(data))
    }
}
impl Validator for ZstFormat {
}
