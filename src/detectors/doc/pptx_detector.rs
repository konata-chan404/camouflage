use crate::Detector;
use std::error::Error;
use infer;

pub struct PptxDetector;

impl Detector for PptxDetector {
    fn detect(&self, data: &[u8]) -> Result<bool, Box<dyn Error>> {
        Ok(infer::doc::is_pptx(data))
    }
}
