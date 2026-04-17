fn main() {
    let output = xifty_rust::extract("fixtures/happy.jpg", xifty_rust::ViewMode::Normalized)
        .expect("extract failed");
    let fields = output["normalized"]["fields"]
        .as_array()
        .expect("normalized fields missing");

    let field = |name: &str| {
        fields
            .iter()
            .find(|entry| entry["field"] == name)
            .and_then(|entry| entry["value"]["value"].as_str().map(str::to_owned))
    };

    let asset = serde_json::json!({
        "sourcePath": "fixtures/happy.jpg",
        "format": output["input"]["detected_format"],
        "capturedAt": field("captured_at"),
        "cameraMake": field("device.make"),
        "cameraModel": field("device.model"),
        "width": fields.iter().find(|entry| entry["field"] == "dimensions.width").unwrap()["value"]["value"],
        "height": fields.iter().find(|entry| entry["field"] == "dimensions.height").unwrap()["value"]["value"],
        "software": field("software"),
    });

    println!(
        "{}",
        serde_json::to_string_pretty(&asset).expect("json encode failed")
    );
}
