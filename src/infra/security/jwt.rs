use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

fn jwt_secret() -> String {
    std::env::var("JWT_SECRET").expect("JWT_SECRET must be set in .env file")
}

pub struct JwtService;

impl JwtService {
    pub fn generate_token(user_id: &str) -> Result<String, String> {
        let days_in_seconds = 60 * 60 * 24 * 7;

        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let claims = Claims {
            sub: user_id.to_owned(),
            exp: (now + Duration::from_secs(days_in_seconds as u64)).as_secs() as usize,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(jwt_secret().as_ref()),
        )
        .map_err(|e| e.to_string())
    }

    pub fn decode(token: &str) -> Result<Claims, String> {
        match decode::<Claims>(
            token,
            &DecodingKey::from_secret(jwt_secret().as_ref()),
            &Validation::new(Algorithm::HS256),
        ) {
            Ok(token_data) => Ok(token_data.claims),
            Err(err) => Err(err.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_and_decode_token() {
        let token_result = JwtService::generate_token("123");
        assert!(token_result.is_ok());

        let token = token_result.unwrap();
        assert!(token.contains('.'));

        let decoded_result = JwtService::decode(&token);
        assert!(decoded_result.is_ok());

        let decoded = decoded_result.unwrap();
        assert_eq!(decoded.sub, "123");
    }
}
