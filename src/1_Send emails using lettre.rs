//dependencies
/*
lettre = "0.10"
lettre_email = "0.9"
native-tls = "0.2.4"
*/

use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::{authentication::{Credentials}};

fn main() ->  std::result::Result<(), Box<dyn std::error::Error>> {
    let email = Message::builder()
        .from("Your Name <ghostdev030823@gmail.com>".parse().unwrap())
        .to("Recipient Name <ghostdev030823@gmail.com>".parse().unwrap())
        .subject("Rust Email")
        .body(String::from("Hello World, this is a test email from Rust!"))
        .unwrap();

    // Set up the SMTP client credentials
    let creds = Credentials::new("12da98fc8a4a19".to_string(), "7e3dce5fb5a334".to_string());

    // Open a remote connection to the SMTP server with STARTTLS
    let mailer = SmtpTransport::starttls_relay("sandbox.smtp.mailtrap.io")
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => eprintln!("Could not send email: {:?}", e),
    }

    Ok(())
}
