use crate::Detector;
use std::error::Error;
use infer;

pub struct ShellscriptDetector;

impl Detector for ShellscriptDetector {
    fn detect(&self, data: &[u8], file_path: &str) -> Result<bool, Box<dyn Error>> {
        Ok(infer::text::is_shellscript(data))
    }
}
