use crate::Detector;
use std::error::Error;
use infer;

pub struct XlsDetector;

impl Detector for XlsDetector {
    fn detect(&self, data: &[u8]) -> Result<bool, Box<dyn Error>> {
        Ok(infer::doc::is_xls(data))
    }
}
