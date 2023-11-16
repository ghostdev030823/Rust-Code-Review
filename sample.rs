use lettre::{Message, SmtpTransport, Transport, ClientSecurity};
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::client::Tls;

fn main() ->  std::result::Result<(), Box<dyn std::error::Error>> {
    let email = Message::builder()
        .from("Your Name <your.email@example.com>".parse().unwrap())
        .reply_to("your.email@example.com".parse().unwrap())
        .to("Recipient Name <recipient.email@example.com>".parse().unwrap())
        .subject("Rust Email")
        .body(String::from("Hello, this is a test email from Rust!"))
        .unwrap();

    // Set up the SMTP client credentials
    let creds = Credentials::new("8382a82526761f".to_string(), "952106364e8818".to_string());

    // Open a remote connection to the SMTP server with STARTTLS
    let mailer = SmtpTransport::builder_dangerous("sandbox.smtp.mailtrap.io")
        .credentials(creds)
        .security(ClientSecurity::Required(Tls::Wrapper))
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => eprintln!("Could not send email: {:?}", e),
    }

    Ok(())
}