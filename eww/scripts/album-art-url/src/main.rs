use tokio;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let body = client.post("https://api.spotify.com/v1/me/player")
        .header("token", "test")
        .send()
        .await;
    println!("{:?}", body.unwrap());
    Ok(())
}
