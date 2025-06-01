use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

use crate::infra::security::jwt::JwtService;

#[derive(Clone, Debug)]
pub struct AuthenticatedUser {
    pub user_id: String,
}

pub async fn auth_middleware(req: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok());

    if let Some(auth_header) = auth_header {
        if let Some(token) = auth_header.strip_prefix("Bearer ") {
            match JwtService::decode(token) {
                Ok(claims) => {
                    let user = AuthenticatedUser {
                        user_id: claims.sub,
                    };
                    let mut req = req;
                    req.extensions_mut().insert(user);
                    return Ok(next.run(req).await);
                }
                Err(_) => return Err(StatusCode::UNAUTHORIZED),
            }
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}
