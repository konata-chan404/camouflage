use crate::Detector;
use std::error::Error;
use infer;

pub struct PngDetector;

impl Detector for PngDetector {
    fn detect(&self, data: &[u8]) -> Result<bool, Box<dyn Error>> {
        Ok(infer::image::is_png(data))
    }
}
