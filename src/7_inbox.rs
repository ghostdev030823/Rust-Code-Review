use reqwest;
use serde_json::json;
use tokio;

#[tokio::main]

async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let api_url = "https://sandbox.api.mailtrap.io/api/send/2117479";
    let api_token = "0f655592a6fbd72f020b4d00085851c4";

    let client = reqwest::Client::new();

    let payload = json!({
        "to": [
            {
                "email": "receiver@gmail.com",
                "name": "sender name"
            }
        ],
        "from": {
            "email": "anyname@freelance.mailtrap.link",
            "name": "Example Sales Team"
        },
        "subject": "Your Example Order Confirmation",
        "text": "Congratulations on your order no. 1234",
        "category": "API Test"
    });


    let response = client.post(api_url)
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .header("Api-Token", api_token)
        .json(&payload)
        .send()
        .await?;

    if response.status().is_success() {
        println!("Email sent successfully!");
    } else {
        println!("Failed to send email. Status: {:?}", response.status());

        // Print the response body for additional information
        let body = response.text().await?;
        println!("Response body: {}", body);
    }

    Ok(())
}
