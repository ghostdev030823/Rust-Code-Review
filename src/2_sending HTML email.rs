use lettre::{transport::smtp::authentication::Credentials, Message, SmtpTransport, Transport};
use lettre::message::{Mailbox, MultiPart, SinglePart};

fn main() ->  std::result::Result<(), Box<dyn std::error::Error>> {

    // Define the HTML content
    let html_content = r#"
        <html>
            <body>
                <h1>Hello!</h1>
                <p>This is a <strong>test email</strong> from Rust!</p>
            </body>
        </html>
    "#;

    let from_email = "Your Name <ghostdev030823@gmail.com>".parse::<Mailbox>().unwrap();
    let to_email = "Recipient Name <receiver@gmail.com>".parse::<Mailbox>().unwrap();

    // Define the email with HTML part
    let email = Message::builder()
        .from(from_email)
        .to(to_email)
        .subject("Rust Email")
        .multipart(
            MultiPart::alternative().singlepart(SinglePart::html(html_content.to_string())),
        )
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
