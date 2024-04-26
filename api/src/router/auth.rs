use cookie::Cookie;
use rspc::{integrations::httpz::CookieJar, Error, ErrorCode, Router};
use serde::{Deserialize, Serialize};
use specta::Type;

use crate::{
    core::context::{query, Context},
    middleware::{auth, cookies},
    service::{
        auth::{Auth, Session},
        users::Users,
    },
};

use super::R;

#[derive(Debug, Clone, Deserialize, Serialize, Type)]
struct AuthArgs {
    email: String,
    password: String,
}

#[derive(Serialize, Type)]
struct VerifiedUser {
    id: i32,
    email: String,
}

async fn verify(ctx: Context, token: String) -> Result<VerifiedUser, Error> {
    let (auth, users) = query!(ctx, Auth, Users);
    let session = auth
        .validate_session(token.as_str())
        .await
        .map_err(|e| Error::new(ErrorCode::BadRequest, e.to_string()))?
        .ok_or_else(|| Error::new(ErrorCode::BadRequest, "Invalid session".to_string()))?;
    let user = users
        .get_user_by_id(session.user_id)
        .await
        .map_err(|e| Error::new(ErrorCode::BadRequest, e.to_string()))?
        .ok_or_else(|| Error::new(ErrorCode::BadRequest, "User not found".to_string()))?;
    Ok(VerifiedUser {
        id: user.id,
        email: user.email,
    })
}

async fn login(ctx: Context, args: AuthArgs) -> Result<(), Error> {
    let AuthArgs { email, password } = args;
    let (auth, users, cookies) = query!(ctx, Auth, Users, CookieJar);

    let user = users
        .get_user_by_email(email.as_str())
        .await
        .map_err(|e| Error::new(ErrorCode::BadRequest, e.to_string()))?
        .ok_or_else(|| Error::new(ErrorCode::BadRequest, "User not found".to_string()))?;

    let valid = tokio::task::spawn_blocking(move || {
        bcrypt::verify(password, &user.password).unwrap_or(false)
    })
    .await
    .map_err(|e| Error::new(ErrorCode::InternalServerError, e.to_string()))?;

    if !valid {
        return Err(Error::new(
            ErrorCode::BadRequest,
            "Invalid email or password".to_string(),
        ));
    }

    let session = auth
        .create_session(user.id)
        .await
        .map_err(|e| Error::new(ErrorCode::BadRequest, e.to_string()))?;

    let mut cookie = Cookie::new("auth_session", session);

    cookie.set_http_only(true);
    cookie.set_domain("localtest.me");
    cookie.set_path("/");

    cookies.add(cookie);

    Ok(())
}

async fn register(ctx: Context, args: AuthArgs) -> Result<(), Error> {
    let AuthArgs { email, password } = args;
    let (auth, users, cookies) = query!(ctx, Auth, Users, CookieJar);

    let password =
        tokio::task::spawn_blocking(move || bcrypt::hash(password, bcrypt::DEFAULT_COST))
            .await
            .map_err(|e| Error::new(ErrorCode::InternalServerError, e.to_string()))?
            .map_err(|e| Error::new(ErrorCode::InternalServerError, e.to_string()))?;

    let user = users
        .create_user(email.as_str(), password.as_str())
        .await
        .map_err(|_| Error::new(ErrorCode::BadRequest, "Error creating user".to_string()))?;

    let session = auth
        .create_session(user.id)
        .await
        .map_err(|e| Error::new(ErrorCode::BadRequest, e.to_string()))?;

    let mut cookie = Cookie::new("auth_session", session);

    cookie.set_http_only(true);
    cookie.set_domain("localtest.me");
    cookie.set_path("/");

    cookies.add(cookie);

    Ok(())
}

async fn logout(ctx: Context, _: ()) -> Result<(), Error> {
    let (cookies, auth, session) = query!(ctx, CookieJar, Auth, Session);

    auth.invalidate_session(&session.token)
        .await
        .map_err(|e| Error::new(ErrorCode::BadRequest, e.to_string()))?;

    let mut cookie = Cookie::new("auth_session", "");

    cookie.set_http_only(true);
    cookie.set_domain("localtest.me");
    cookie.set_path("/");

    cookies.remove(cookie);

    Ok(())
}

pub fn mount() -> Router<Context> {
    R.router()
        .procedure("verify", R.query(verify))
        .procedure("login", R.with(cookies()).mutation(login))
        .procedure("register", R.with(cookies()).mutation(register))
        .procedure("logout", R.with(cookies()).with(auth()).query(logout))
}
