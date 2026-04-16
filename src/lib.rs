use std::ffi::{CStr, CString, c_char};

use serde_json::Value;
use thiserror::Error;

#[repr(C)]
struct XiftyBuffer {
    ptr: *mut u8,
    len: usize,
    capacity: usize,
}

#[repr(C)]
struct XiftyResult {
    status: i32,
    output: XiftyBuffer,
    error_message: XiftyBuffer,
}

#[repr(C)]
enum XiftyViewMode {
    Full = 0,
    Raw = 1,
    Interpreted = 2,
    Normalized = 3,
    Report = 4,
}

unsafe extern "C" {
    fn xifty_probe_json(path: *const c_char) -> XiftyResult;
    fn xifty_extract_json(path: *const c_char, view_mode: XiftyViewMode) -> XiftyResult;
    fn xifty_free_buffer(buffer: XiftyBuffer);
    fn xifty_version() -> *const c_char;
}

#[derive(Debug, Clone, Copy)]
pub enum ViewMode {
    Full,
    Raw,
    Interpreted,
    Normalized,
    Report,
}

impl From<ViewMode> for XiftyViewMode {
    fn from(value: ViewMode) -> Self {
        match value {
            ViewMode::Full => XiftyViewMode::Full,
            ViewMode::Raw => XiftyViewMode::Raw,
            ViewMode::Interpreted => XiftyViewMode::Interpreted,
            ViewMode::Normalized => XiftyViewMode::Normalized,
            ViewMode::Report => XiftyViewMode::Report,
        }
    }
}

#[derive(Debug, Error)]
pub enum XiftyError {
    #[error("ffi error {status}: {message}")]
    Ffi { status: i32, message: String },
    #[error("invalid utf-8 from xifty")]
    InvalidUtf8,
    #[error("invalid json from xifty: {0}")]
    InvalidJson(#[from] serde_json::Error),
    #[error("path contains interior nul")]
    Nul(#[from] std::ffi::NulError),
}

pub fn version() -> String {
    unsafe { CStr::from_ptr(xifty_version()) }
        .to_string_lossy()
        .into_owned()
}

pub fn probe(path: impl AsRef<str>) -> Result<Value, XiftyError> {
    let path = CString::new(path.as_ref())?;
    let result = unsafe { xifty_probe_json(path.as_ptr()) };
    decode_result(result)
}

pub fn extract(path: impl AsRef<str>, view: ViewMode) -> Result<Value, XiftyError> {
    let path = CString::new(path.as_ref())?;
    let result = unsafe { xifty_extract_json(path.as_ptr(), view.into()) };
    decode_result(result)
}

fn decode_result(result: XiftyResult) -> Result<Value, XiftyError> {
    let status = result.status;
    let output = buffer_to_string(&result.output)?;
    let error = buffer_to_string(&result.error_message)?;
    unsafe {
        xifty_free_buffer(result.output);
        xifty_free_buffer(result.error_message);
    }

    if status != 0 {
        return Err(XiftyError::Ffi {
            status,
            message: error,
        });
    }

    Ok(serde_json::from_str(&output)?)
}

fn buffer_to_string(buffer: &XiftyBuffer) -> Result<String, XiftyError> {
    if buffer.ptr.is_null() || buffer.len == 0 {
        return Ok(String::new());
    }

    let bytes = unsafe { std::slice::from_raw_parts(buffer.ptr, buffer.len) };
    String::from_utf8(bytes.to_vec()).map_err(|_| XiftyError::InvalidUtf8)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_is_non_empty() {
        assert!(!version().is_empty());
    }

    #[test]
    fn probe_returns_detected_format() {
        let output = probe("fixtures/happy.jpg").unwrap();
        assert_eq!(output["input"]["detected_format"], "jpeg");
    }

    #[test]
    fn extract_normalized_returns_expected_field() {
        let output = extract("fixtures/happy.jpg", ViewMode::Normalized).unwrap();
        let fields = output["normalized"]["fields"].as_array().unwrap();
        let make = fields
            .iter()
            .find(|field| field["field"] == "device.make")
            .unwrap();
        assert_eq!(make["value"]["value"], "XIFtyCam");
    }
}

