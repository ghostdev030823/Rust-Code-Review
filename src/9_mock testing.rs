// #[cfg(test)]
// mod tests {
//     use super::*;
//     use mockall::{automock, predicate::*};
//     use predicate:eq;

//     #[automock]
//     trait EmailSender {
//         fn send_email(&self, to: &str, subject: &str, body: &str) -> Result<(), String>;
//     }

//     #[test]
//     fn test_email_send() {
//         let mut mock_sender = MockEmailSender::new();
//         mock_sender.expect_send_email()
//             .with(predicate::eq("test@example.com"), predicate::eq("Test"), predicate::eq("Body"))
//             .times(1)
//             .returning(|_, _, _| Ok(()));

//         let result = mock_sender.send_email("test@example.com", "Test", "Body");
//         assert!(result.is_ok());
//     }
// }

/*
reqwest = { version = "0.11", features = ["json"] }
serde_json = "1.0"
serde = { version = "1", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
lettre = "0.10"
mockall = "0.9"
*/

use mockall::predicate::*;
use mockall::Sequence;
use mockall::automock;

#[automock]
trait EmailSender {
    fn send_email(&self, recipient: &str, subject: &str, body: &str) -> Result<(), String>;
}

struct MyComponent<T: EmailSender> {
    email_sender: T,
}

impl<T: EmailSender> MyComponent<T> {
    pub fn new(email_sender: T) -> Self {
        MyComponent { email_sender }
    }

    pub fn do_something(&self) -> Result<(), String> {
        // Code that uses the email sender component
        let recipient = "ghostdev030823@gmail.com";
        let subject = "Test Subject";
        let body = "Test Body";

        self.email_sender.send_email(recipient, subject, body)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_do_something() {
        let mut email_sender = MockEmailSender::new();
        
        let recipient = "ghostdev030823@gmail.com";
        let subject = "Test Subject";
        let body = "Test Body";
        
        // Set up expectations for the mock object
        email_sender.expect_send_email()
            .once()
            .withf(move |r, s, b| r == recipient && s == subject && b == body)
            .returning(|_, _, _| Ok(()));

        let my_component = MyComponent::new(email_sender);
        let result = my_component.do_something();

        assert!(result.is_ok());
    }
}
