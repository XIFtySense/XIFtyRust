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
    fn version_looks_semantic() {
        assert!(version().chars().any(|ch| ch == '.'));
    }

    #[test]
    fn probe_returns_input_summary() {
        let output = probe("fixtures/happy.jpg").unwrap();
        assert_eq!(output["schema_version"], "0.1.0");
        assert_eq!(output["input"]["detected_format"], "jpeg");
        assert_eq!(output["input"]["container"], "jpeg");
    }

    #[test]
    fn extract_defaults_to_full_envelope() {
        let output = extract("fixtures/happy.jpg", ViewMode::Full).unwrap();
        assert!(output.get("raw").is_some());
        assert!(output.get("interpreted").is_some());
        assert!(output.get("normalized").is_some());
        assert!(output.get("report").is_some());
    }

    #[test]
    fn raw_view_preserves_metadata_evidence() {
        let output = extract("fixtures/happy.jpg", ViewMode::Raw).unwrap();
        assert_eq!(output["raw"]["containers"][0]["label"], "jpeg");
        assert_eq!(output["raw"]["metadata"][0]["tag_name"], "ImageWidth");
    }

    #[test]
    fn interpreted_view_exposes_decoded_tags() {
        let output = extract("fixtures/happy.jpg", ViewMode::Interpreted).unwrap();
        let fields = output["interpreted"]["metadata"].as_array().unwrap();
        let names: Vec<_> = fields.iter().map(|field| field["tag_name"].as_str().unwrap()).collect();
        assert!(names.contains(&"Make"));
        assert!(names.contains(&"Model"));
        assert!(names.contains(&"DateTimeOriginal"));
    }

    #[test]
    fn normalized_view_returns_expected_fields() {
        let output = extract("fixtures/happy.jpg", ViewMode::Normalized).unwrap();
        let fields = output["normalized"]["fields"].as_array().unwrap();
        let field = |name: &str| fields.iter().find(|entry| entry["field"] == name).unwrap();
        assert_eq!(field("captured_at")["value"]["value"], "2024-04-16T12:34:56");
        assert_eq!(field("device.make")["value"]["value"], "XIFtyCam");
        assert_eq!(field("device.model")["value"]["value"], "IterationOne");
        assert_eq!(field("software")["value"]["value"], "XIFtyTestGen");
        assert_eq!(field("dimensions.width")["value"]["value"], 800);
        assert_eq!(field("dimensions.height")["value"]["value"], 600);
    }

    #[test]
    fn report_view_stays_explicit_when_empty() {
        let output = extract("fixtures/happy.jpg", ViewMode::Report).unwrap();
        assert_eq!(output["report"]["issues"].as_array().unwrap().len(), 0);
        assert_eq!(output["report"]["conflicts"].as_array().unwrap().len(), 0);
    }
}
