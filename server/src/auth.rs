use crate::error::Error;
use crate::routes::AppState;
use axum::async_trait;
use axum::extract::{FromRef, FromRequestParts};
use axum::http::{header::AUTHORIZATION, request::Parts, HeaderValue};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

const DEFAULT_SESSION_LENGTH: time::Duration = time::Duration::weeks(2);
const SCHEME_PREFIX: &str = "Bearer ";

pub struct AuthUser {
    pub id: i64,
}

pub struct MaybeAuthUser(pub Option<AuthUser>);

impl MaybeAuthUser {
    pub fn id(&self) -> Option<i64> {
        self.0.as_ref().map(|auth_user| auth_user.id)
    }
}

#[derive(Serialize, Deserialize)]
struct AuthUserClaims {
    id: i64,
    exp: i64,
}

impl AuthUser {
    pub(crate) fn to_jwt(&self, state: &AppState) -> String {
        let claims = AuthUserClaims {
            id: self.id,
            exp: (OffsetDateTime::now_utc() + DEFAULT_SESSION_LENGTH).unix_timestamp(),
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(state.key.as_ref()),
        )
        .expect("JWT encode failed")
    }

    fn from_authorization(state: &AppState, auth_header: &HeaderValue) -> Result<Self, Error> {
        let auth_header = auth_header.to_str().map_err(|_e| {
            log::debug!("Authorization header is not UTF-8");
            Error::Unauthorized
        })?;

        if !auth_header.starts_with(SCHEME_PREFIX) {
            log::debug!(
                "Authorization header is using the wrong scheme: {:?}",
                auth_header
            );
            return Err(Error::Unauthorized);
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
            Error::Unauthorized
        })?;

        let claims = jwt.claims;

        if claims.exp < OffsetDateTime::now_utc().unix_timestamp() {
            log::debug!("Token expired");
            return Err(Error::Unauthorized);
        }

        Ok(Self { id: claims.id })
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
    AppState: FromRef<S>,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = AppState::from_ref(state);
        let auth_header = parts
            .headers
            .get(AUTHORIZATION)
            .ok_or(Error::Unauthorized)?;

        Self::from_authorization(&state, auth_header)
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for MaybeAuthUser
where
    S: Send + Sync,
    AppState: FromRef<S>,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state: AppState = AppState::from_ref(state);

        Ok(Self(
            parts
                .headers
                .get(AUTHORIZATION)
                .map(|auth_header| AuthUser::from_authorization(&state, auth_header))
                .transpose()?,
        ))
    }
}
