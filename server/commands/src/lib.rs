mod registeruser;
mod sendmessage;

pub use registeruser::{CreatedUser, RegisterUserCommand, build_register_command};
pub use sendmessage::{SendMessageCommand, SentMessage, build_send_message_command};

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
