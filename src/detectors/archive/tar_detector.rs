use crate::Detector;
use std::error::Error;
use infer;

pub struct TarDetector;

impl Detector for TarDetector {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::archive::is_tar(data))
    }
}
