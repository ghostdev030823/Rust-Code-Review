use sendmail::email::EmailBuilder;
use sendmail::sendmail;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build the email
    let email = EmailBuilder::new()
        // Addresses can be specified by the tuple (email, alias)
        .to(("recipient.email@example.com", "Recipient Name"))
        .from(("your.email@example.com", "Your Name"))
        .subject("Rust Email")
        .body("Hello, this is a test email from Rust!")
        .build()?;

    // Send the email
    match sendmail(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => eprintln!("Could not send email: {:?}", e),
    }

    Ok(())
}
