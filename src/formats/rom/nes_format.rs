use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct NesFormat;

impl Detector for NesFormat {
    fn detect(&self, data: &[u8], _file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::archive::is_nes(data))
    }
}
impl Validator for NesFormat {
}
