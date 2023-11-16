// // [dependencies]
// // reqwest = { version = "0.11", features = ["json"] }
// // serde_json = "1.0"
// // serde = { version = "1", features = ["derive"] }
// // tokio = { version = "1", features = ["full"] }

// use reqwest;
// use serde_json::json;

// #[tokio::main]
// async fn main() -> Result<(), reqwest::Error> {
//     let api_url = "https://send.api.mailtrap.io/api/send";
//     let api_key = "0f655592a6fbd72f020b4d00085851c4";
//     let email_payload = json!({
//         "from": {"email" : "anyname@freelance.mailtrap.link"},
//         "to": [{"email": "730e82205d-7e378a@inbox.mailtrap.io"}],
//         "subject": "Test Email",
//         "text": "This is a test email using Rust and Mailtrap API!",
//     });

//     let client = reqwest::Client::new();
//     let response = client
//         .post(api_url)
//         .header("Content-Type", "application/json")
//         .header("Api-Token", api_key)
//         .body(email_payload.to_string()) // Serialize the JSON payload to a string
//         .send()
//         .await?;

//     if response.status().is_success() {
//         println!("Email sent successfully!");
//     } else {
//         println!("Failed to send email. Status: {:?}", response.status());

//         // Print the response body for additional information
//         let body = response.text().await?;
//         println!("Response body: {}", body);
//     }

//     Ok(())
// }


use reqwest;
use serde_json::json;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_url = "https://sandbox.api.mailtrap.io/api/send/730e82205d-7e378a@inbox.mailtrap.io"; // Replace 'inbox_id' with your actual Mailtrap inbox ID
    let api_token = "0f655592a6fbd72f020b4d00085851c4"; // Replace with your actual Mailtrap API token

    let client = reqwest::Client::new();

    let payload = json!({
        "to": [
            {
                "email": "john_doe@example.com",
                "name": "John Doe"
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
