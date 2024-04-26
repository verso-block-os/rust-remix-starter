use cookie::Cookie;
use rspc::{integrations::httpz::CookieJar, Router};
use serde::{Deserialize, Serialize};
use specta::{selection, Type};

use crate::{
    core::context::{query, Context},
    service::{
        auth::{Auth, Session},
        users::Users,
    },
};

use super::{auth, cookies, R};

#[derive(Debug, Clone, Deserialize, Serialize, Type)]
struct AuthArgs {
    email: String,
    password: String,
}

pub fn mount() -> Router<Context> {
    R.router()
        .procedure(
            "verify",
            R.query(|ctx, token: String| async move {
                let (auth, users) = query!(ctx, Auth, Users);
                let session = auth
                    .validate_session(token.as_str())
                    .await
                    .map_err(|e| rspc::Error::new(rspc::ErrorCode::BadRequest, e.to_string()))?
                    .ok_or_else(|| {
                        rspc::Error::new(rspc::ErrorCode::BadRequest, "Invalid session".to_string())
                    })?;

                let user = users
                    .get_user_by_id(session.user_id)
                    .await
                    .map_err(|e| rspc::Error::new(rspc::ErrorCode::BadRequest, e.to_string()))?
                    .ok_or_else(|| {
                        rspc::Error::new(rspc::ErrorCode::BadRequest, "User not found".to_string())
                    })?;

                Ok(selection!(user, {
                    id,
                    email
                }))
            }),
        )
        .procedure(
            "login",
            R.with(cookies())
                .mutation(|ctx, args: AuthArgs| async move {
                    let AuthArgs { email, password } = args;
                    let (auth, users, cookies) = query!(ctx, Auth, Users, CookieJar);

                    let user = users
                        .get_user_by_email(email.as_str())
                        .await
                        .map_err(|e| rspc::Error::new(rspc::ErrorCode::BadRequest, e.to_string()))?
                        .ok_or_else(|| {
                            rspc::Error::new(
                                rspc::ErrorCode::BadRequest,
                                "User not found".to_string(),
                            )
                        })?;

                    let valid = tokio::task::spawn_blocking(move || {
                        bcrypt::verify(password, &user.password).unwrap_or(false)
                    })
                    .await
                    .map_err(|e| {
                        rspc::Error::new(rspc::ErrorCode::InternalServerError, e.to_string())
                    })?;

                    if !valid {
                        return Err(rspc::Error::new(
                            rspc::ErrorCode::BadRequest,
                            "Invalid email or password".to_string(),
                        ));
                    }

                    let session = auth.create_session(user.id).await.map_err(|e| {
                        rspc::Error::new(rspc::ErrorCode::BadRequest, e.to_string())
                    })?;

                    let mut cookie = Cookie::new("auth_session", session);

                    cookie.set_http_only(true);
                    cookie.set_domain("localtest.me");
                    cookie.set_path("/");

                    cookies.add(cookie);

                    Ok(())
                }),
        )
        .procedure(
            "register",
            R.with(cookies())
                .mutation(|ctx, args: AuthArgs| async move {
                    let AuthArgs { email, password } = args;
                    let (auth, users, cookies) = query!(ctx, Auth, Users, CookieJar);

                    let password = tokio::task::spawn_blocking(move || {
                        bcrypt::hash(password, bcrypt::DEFAULT_COST)
                    })
                    .await
                    .map_err(|e| {
                        rspc::Error::new(rspc::ErrorCode::InternalServerError, e.to_string())
                    })?
                    .map_err(|e| {
                        rspc::Error::new(rspc::ErrorCode::InternalServerError, e.to_string())
                    })?;

                    let user = users
                        .create_user(email.as_str(), password.as_str())
                        .await
                        .map_err(|_| {
                            rspc::Error::new(
                                rspc::ErrorCode::BadRequest,
                                "Error creating user".to_string(),
                            )
                        })?;

                    let session = auth.create_session(user.id).await.map_err(|e| {
                        rspc::Error::new(rspc::ErrorCode::BadRequest, e.to_string())
                    })?;

                    let mut cookie = Cookie::new("auth_session", session);

                    cookie.set_http_only(true);
                    cookie.set_domain("localtest.me");
                    cookie.set_path("/");

                    cookies.add(cookie);

                    Ok(())
                }),
        )
        .procedure(
            "logout",
            R.with(cookies())
                .with(auth())
                .mutation(|ctx, _: ()| async move {
                    let (cookies, auth, session) = query!(ctx, CookieJar, Auth, Session);

                    auth.invalidate_session(&session.token).await.map_err(|e| {
                        rspc::Error::new(rspc::ErrorCode::BadRequest, e.to_string())
                    })?;

                    if let Some(cookie) = cookies.get("auth_session") {
                        cookies.remove(cookie)
                    }

                    Ok(())
                }),
        )
}
