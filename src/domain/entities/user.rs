use regex::Regex;

pub struct User {
    id: i32,
    email: String,
    password_hash: String,
}

impl User {
    pub fn new(id: i32, email: String, password_hash: String) -> Self {
        Self {
            id,
            email,
            password_hash,
        }
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
