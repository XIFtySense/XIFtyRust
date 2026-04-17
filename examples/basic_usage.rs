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
            .expect("field missing")["value"]["value"]
            .clone()
    };

    println!("XIFty version: {}", xifty_rust::version());
    println!("Detected format: {}", output["input"]["detected_format"]);
    println!("Camera: {} {}", field("device.make"), field("device.model"));
    println!("Captured at: {}", field("captured_at"));
    println!(
        "Dimensions: {}x{}",
        field("dimensions.width"),
        field("dimensions.height")
    );
    println!(
        "{}",
        serde_json::to_string_pretty(&output).expect("json encode failed")
    );
}
