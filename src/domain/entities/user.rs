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
