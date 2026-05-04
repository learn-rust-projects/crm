use std::time::SystemTime;

use prost_types::Timestamp;

use crate::pb::crm::User;

impl User {
    pub fn new(id: u64, name: &str, email: &str) -> Self {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        User {
            id,
            name: name.to_string(),
            email: email.to_string(),
            created_at: Some(Timestamp {
                seconds: now.as_secs() as i64,
                nanos: now.subsec_nanos() as i32,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use prost::Message;

    use super::*;
    #[test]
    fn test_user_new() {
        let user = User::new(1, "l", "l@example.com");
        assert_eq!(user.id, 1);
        assert_eq!(user.name, "l");
        assert_eq!(user.email, "l@example.com");
        let bytes = user.encode_to_vec();
        assert!(!bytes.is_empty());
        let user2 = User::decode(&*bytes).unwrap();
        assert_eq!(user2.id, 1);
        assert_eq!(user2.name, "l");
        assert_eq!(user2.email, "l@example.com");
        assert_eq!(user2.created_at, Some(user.created_at.unwrap()));
    }
}
