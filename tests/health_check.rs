#[tokio::test]
async fn health_check_works() {
    let client = reqwest::Client::new();
    let hand1 = spawn_app();

    let hand2 = tokio::spawn(async move {
        client
            .get("http://127.0.0.1:8080/health_check")
            .send()
            .await
            .expect("FAIL TO SEND MSG")
    });

    let (_, response) = tokio::join!(hand1, hand2);
    match response {
        Ok(response) => {
            assert!(response.status().is_success());
            assert_eq!(Some(0), response.content_length());
        }
        Err(_) => println!("Response couldn't be"),
    };
}

async fn spawn_app() {
    let server = zero2prod::run().expect("Failed to bind address");
    let _ = tokio::spawn(server);
}
