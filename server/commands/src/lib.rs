mod registeruser;
mod sendmessage;
mod create_auth;

pub use registeruser::*;
pub use sendmessage::*;
pub use create_auth::*;

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
