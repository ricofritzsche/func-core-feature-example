use reqwest::Client;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::test]
async fn test_track_endpoint_end_to_end() {
    let client = Client::new();
    let base_url = "http://localhost:8080/track";

    sleep(Duration::from_millis(100)).await;

    let payload = serde_json::json!({
        "asset_id": "test-asset-1",
        "lat": 40.5,
        "lon": -73.9
    });

    let resp = client
        .post(base_url)
        .json(&payload)
        .send()
        .await
        .expect("Request failed");

    assert!(resp.status().is_success());

    let json: serde_json::Value = resp.json().await.expect("Invalid JSON");

    assert_eq!(json["asset_id"], "test-asset-1");

    let allowed = ["Entered", "StayedInside", "Exited", "StayedOutside", "Unknown"];
    let movement = json["movement"].as_str().unwrap();

    assert!(allowed.contains(&movement));
}
