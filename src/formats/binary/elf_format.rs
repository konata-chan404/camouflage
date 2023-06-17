use crate::formats::Detector;
use crate::formats::Validator;
use std::error::Error;
use infer;

pub struct ElfFormat;

impl Detector for ElfFormat {
    fn detect(&self, data: &[u8], _file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::app::is_elf(data))
    }
}
impl Validator for ElfFormat {
}
