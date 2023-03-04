use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::{header::AUTHORIZATION, request::Parts, HeaderValue, StatusCode},
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::routes::AppState;

const DEFAULT_SESSION_LENGTH: time::Duration = time::Duration::weeks(2);
const SCHEME_PREFIX: &str = "Bearer ";

pub struct AuthUser {
    pub user_id: Uuid,
}

pub struct MaybeAuthUser(pub Option<AuthUser>);

#[derive(Serialize, Deserialize)]
struct AuthUserClaims {
    user_id: Uuid,
    exp: i64,
}

impl AuthUser {
    pub(crate) fn to_jwt(&self, state: &AppState) -> String {
        let claims = AuthUserClaims {
            user_id: self.user_id,
            exp: (OffsetDateTime::now_utc() + DEFAULT_SESSION_LENGTH).unix_timestamp(),
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(state.key.as_ref()),
        )
        .expect("JWT encode failed")
    }

    fn from_authorization(state: &AppState, auth_header: &HeaderValue) -> Result<Self, StatusCode> {
        let auth_header = auth_header.to_str().map_err(|_e| {
            log::debug!("Authorization header is not UTF-8");
            StatusCode::UNAUTHORIZED
        })?;

        if !auth_header.starts_with(SCHEME_PREFIX) {
            log::debug!(
                "Authorization header is using the wrong scheme: {:?}",
                auth_header
            );
            return Err(StatusCode::UNAUTHORIZED);
        }

        let token = &auth_header[SCHEME_PREFIX.len()..];

        let jwt = decode::<AuthUserClaims>(
            &token,
            &DecodingKey::from_secret(state.key.as_ref()),
            &Validation::new(jsonwebtoken::Algorithm::HS256),
        )
        .map_err(|e| {
            log::debug!(
                "Failed to parse and verify Authorization header {:?}: {}",
                auth_header,
                e
            );
            StatusCode::UNAUTHORIZED
        })?;

        let claims = jwt.claims;

        if claims.exp < OffsetDateTime::now_utc().unix_timestamp() {
            log::debug!("Token expired");
            return Err(StatusCode::UNAUTHORIZED);
        }

        Ok(Self {
            user_id: claims.user_id,
        })
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
    AppState: FromRef<S>,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = AppState::from_ref(state);
        let auth_header = parts
            .headers
            .get(AUTHORIZATION)
            .ok_or(StatusCode::UNAUTHORIZED)?;

        Self::from_authorization(&state, auth_header)
    }
}
