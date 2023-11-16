// Import necessary items from the lettre library
use lettre::{Message, SmtpTransport, Transport, transport::smtp::authentication::Credentials};

fn main() {
    // Construct an email message with builder pattern
    let email = Message::builder()
        // Set the sender. .parse().unwrap() converts the string into a valid mail address, panicking if it can't
        .from("Your name <ghostdev030823@gmail.com>".parse().unwrap())
        // Set the recipient in the same way as above
        .to("Recipient Name <ghostdev030823@gmail.com>".parse().unwrap())
        // Set the subject of the email
        .subject("Rust Email")
        // Set the body/content of the email
        .body(String::from("Hello, this is a test email from Rust!"))
        // Builds the message, potentially returning an error which gets unwrapped to cause a panic on failure
        .unwrap();

    // Create SMTP credentials for the mail server
    let creds = Credentials::new(
            "postmaster@sandbox36d6b53caec34e84a9d432d383093aa3.mailgun.org".to_string(),
            "ghost0823".to_string());

    // Establishes a SMTP connection to the specified relay (here, mailgun).
    let mailer = SmtpTransport::relay("smtp.mailgun.org") 
        // Unwraps the Result returned by 'relay', causing a panic on failure
        .unwrap()
        // Provides the necessary credentials to authenticate with the server
        .credentials(creds)
        // Finally builds the smtp transport
        .build();

    // Send the email and store the result
    let result = mailer.send(&email);

    // If the send was successful then print success message
    if result.is_ok() {
        println!("Email sent successfully!");
    } 
    // If the email wasn't sent, print out error message
    else {
        println!("Could not send email: {:?}", result);
    }
}
