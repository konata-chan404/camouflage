use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct CpioFormat;

impl Detector for CpioFormat {
    fn detect(&self, data: &[u8], _file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::archive::is_cpio(data))
    }
}
impl Validator for CpioFormat {
}
