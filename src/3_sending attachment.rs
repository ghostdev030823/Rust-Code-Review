use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use lettre::message::{Mailbox, MultiPart, SinglePart, Attachment, Body};
use std::fs;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {


    let image = fs::read("picture.png")?;
    let image_body = Body::new(image);

    let from_email = "Your Name <ghostdev030823@gmail.com>".parse::<Mailbox>().unwrap();
    let to_email = "Recipient Name <recipient-email@gmail.com>".parse::<Mailbox>().unwrap();

    let email = Message::builder()
                    .from(from_email)
                    .to(to_email)
                    .subject("Hello")
                    .multipart(
                        MultiPart::mixed()
                            .multipart(
                                MultiPart::alternative()
                                    .singlepart(SinglePart::plain(String::from("Hello, world! :)")))
                                    .multipart(
                                        MultiPart::related()
                                            .singlepart(SinglePart::html(String::from(
                                                "<p><b>Hello</b>, <i>world</i>! <img src=cid:123></p>",
                                            )))
                                            .singlepart(
                                                Attachment::new_inline(String::from("123"))
                                                    .body(image_body, "image/png".parse().unwrap()),
                                            ),
                                    ),
                            )
                            .singlepart(Attachment::new(String::from("example.rs")).body(
                                String::from("fn main() { println!(\"Hello, World!\") }"),
                                "text/plain".parse().unwrap(),
                            )),
                    )?;

    let creds = Credentials::new("12da98fc8a4a19".to_string(), "7e3dce5fb5a334".to_string());

    
    // Open a remote connection to the SMTP server with STARTTLS
    let mailer = SmtpTransport::starttls_relay("sandbox.smtp.mailtrap.io").unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => eprintln!("Could not send email: {:?}", e),
    }

    Ok(())
}
