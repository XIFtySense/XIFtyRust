fn main() {
    println!("XIFty version: {}", xifty_rust::version());
    let output = xifty_rust::extract("fixtures/happy.jpg", xifty_rust::ViewMode::Normalized)
        .expect("extract failed");
    println!(
        "{}",
        serde_json::to_string_pretty(&output).expect("json encode failed")
    );
}
