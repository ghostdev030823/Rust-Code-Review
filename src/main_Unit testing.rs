#[cfg(test)]
mod tests {
    use super::*;
    use mockall::{automock, predicate::*};
    use predicate:eq;

    #[automock]
    trait EmailSender {
        fn send_email(&self, to: &str, subject: &str, body: &str) -> Result<(), String>;
    }

    #[test]
    fn test_email_send() {
        let mut mock_sender = MockEmailSender::new();
        mock_sender.expect_send_email()
            .with(predicate::eq("test@example.com"), predicate::eq("Test"), predicate::eq("Body"))
            .times(1)
            .returning(|_, _, _| Ok(()));

        let result = mock_sender.send_email("test@example.com", "Test", "Body");
        assert!(result.is_ok());
    }
}
