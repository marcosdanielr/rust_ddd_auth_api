use regex::Regex;
use uuid::Uuid;

#[derive(Clone)]
pub struct User {
    id: Uuid,
    email: String,
    password_hash: String,
}

impl User {
    pub fn new(email: String, password_hash: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            email,
            password_hash,
        }
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn validate_email(email: &str) -> bool {
        let re = Regex::new(r"^[\w\.-]+@([\w-]+\.)+[\w-]{2,4}$").unwrap();
        re.is_match(email)
    }

    pub fn validate_password(password: &str) -> bool {
        password.len() >= 8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_user() {
        let user = User::new("marcos@example.com".to_string(), "hash-example".to_string());

        assert_eq!(user.id.get_version_num(), 4);
        assert_eq!(user.email, "marcos@example.com");
        assert_eq!(user.password_hash, "hash-example");
    }

    #[test]
    fn test_validate_email() {
        assert!(User::validate_email("test@example.com"));
        assert!(!User::validate_email("invalid-email"));
    }

    #[test]
    fn test_validate_password() {
        assert!(User::validate_password("12345678"));
        assert!(!User::validate_password("1234567"));
    }
}
