use crate::Detector;
use std::error::Error;
use infer;

pub struct AacDetector;

impl Detector for AacDetector {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::audio::is_aac(data))
    }
}
