use crate::{model::current_user::CurrentUser, AppState};
use axum::{
    extract::State,
    http::{self, Request, StatusCode},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{
    decode, decode_header, encode, jwk::JwkSet, Algorithm, DecodingKey, EncodingKey, Header,
    Validation,
};
use serde::{Deserialize, Serialize};

pub async fn authentication<T>(
    State(state): State<AuthConfig>,
    mut req: Request<T>,
    next: Next<T>,
) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let auth_header = if let Some(auth_header) = auth_header {
        auth_header.trim_start_matches("Bearer ")
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    if let Some(current_user) = authorize_current_user(auth_header, &state).await {
        // insert the current user into a request extension so the handler can
        // extract it
        req.extensions_mut().insert(current_user);
        Ok(next.run(req).await)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

async fn authorize_current_user(auth_token: &str, config: &AuthConfig) -> Option<CurrentUser> {
    let header = decode_header(auth_token).ok()?;
    let kid = header.kid?;
    let jwk = config.jwks.find(&kid)?;

    match jwk.algorithm {
        jsonwebtoken::jwk::AlgorithmParameters::RSA(ref rsa) => {
            let mut validation = Validation::new(Algorithm::RS256);
            validation.set_audience(&[&config.audience]);
            validation.set_issuer(&[&config.authority]);

            let key = DecodingKey::from_rsa_components(&rsa.n, &rsa.e).ok()?;

            let token_data = decode::<Claims>(&auth_token, &key, &validation);

            match token_data {
                Ok(data) => Some(CurrentUser {
                    tenant_id: data.claims.api_meta_data.tenant_id,
                }),
                Err(e) => {
                    println!("token error: {}", e);
                    None
                }
            }
        }
        _ => unreachable!("should always be RSA"),
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    api_meta_data: MetaData,
}
#[derive(Debug, Serialize, Deserialize)]
struct MetaData {
    tenant_id: i32,
    //company: String,
}
#[derive(Clone)]
pub struct AuthConfig {
    pub authority: String,
    pub audience: String,
    pub jwks: JwkSet,
}
