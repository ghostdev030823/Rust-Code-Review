// [dependencies]
// sendmail = "2.0.0"

extern crate sendmail;
use sendmail::email;

fn main() {

    // Configure email body and header
    // Send the email
    match email::send(
        // From Address
        "ghostdev030823@gmail.com",
        // To Address
        &["ghostdev030823@gmail.com"],
        // Subject
        "Subject - Hello World!",
        // Body
        "<html><body><h1>I am the body. Hello Wolrd!<br/><br/>And I accept html.</h1></body></html>"
    ) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => eprintln!("Could not send email: {:?}", e),
    }
    
}






// use std::io::Write;
// use std::process::{Command, Stdio};
// use std::io::BufWriter;

// fn main() {
//     // Replace these values with your email details
//     let to_email = "ghostdev030823@gmail.com";
//     let subject = "Hello from Rust!";
//     let body = "This is the email body.";

//     // Command to send an email using sendmail
//     let sendmail_command = format!(
//         "/usr/sbin/sendmail -t -i -f {}",
//         "ghostdev030823@gmail.com"
//     );

//     let email_content = format!(
//         "To: {}\nSubject: {}\n\n{}",
//         to_email, subject, body
//     );

//     // Execute the sendmail command with the email content as input
//     let mut child = Command::new("sh")
//         .arg("-c")
//         .arg(&sendmail_command)
//         .stdin(Stdio::piped()) // Use stdin with piped
//         .spawn()
//         .expect("Failed to execute sendmail command.");

//     if let Some(mut stdin) = child.stdin.take() {
//         let mut writer = BufWriter::new(&mut stdin);
//         writer
//             .write_all(email_content.as_bytes())
//             .expect("Failed to write to sendmail stdin.");
//     }

//     let status = child.wait().expect("Failed to wait for sendmail command.");

//     if status.success() {
//         println!("Email sent successfully!");
//     } else {
//         println!("Failed to send email.");
//     }
// }
