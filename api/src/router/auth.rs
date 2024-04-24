use rspc::selection;
use serde::{Deserialize, Serialize};
use specta::Type;
use tower_cookies::Cookie;

use super::{Context, ProtectedContext};

#[derive(Debug, Clone, Deserialize, Serialize, Type)]
struct AuthArgs {
    email: String,
    password: String,
}

pub fn mount() -> rspc::RouterBuilder<Context> {
    rspc::Router::<Context>::new()
        .query("verify", |t| {
            t(|ctx: Context, token: String| async move {
                let session = ctx
                    .service
                    .auth
                    .validate_session(token.as_str())
                    .await
                    .map_err(|e| rspc::Error::new(rspc::ErrorCode::BadRequest, e.to_string()))?
                    .ok_or_else(|| {
                        rspc::Error::new(rspc::ErrorCode::BadRequest, "Invalid session".to_string())
                    })?;

                let user = ctx
                    .service
                    .users
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
            })
        })
        .mutation("login", |t| {
            t(|ctx: Context, args: AuthArgs| async move {
                let AuthArgs { email, password } = args;

                let user = ctx
                    .service
                    .users
                    .get_user_by_email(email.as_str())
                    .await
                    .map_err(|e| rspc::Error::new(rspc::ErrorCode::BadRequest, e.to_string()))?
                    .ok_or_else(|| {
                        rspc::Error::new(rspc::ErrorCode::BadRequest, "User not found".to_string())
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
                        "Invalid password".to_string(),
                    ));
                }

                let session = ctx
                    .service
                    .auth
                    .create_session(user.id)
                    .await
                    .map_err(|e| rspc::Error::new(rspc::ErrorCode::BadRequest, e.to_string()))?;

                let mut cookie = Cookie::new("auth_session", session);

                cookie.set_http_only(true);
                cookie.set_domain("localtest.me");
                cookie.set_path("/");

                ctx.cookies.add(cookie);

                Ok(())
            })
        })
        .mutation("register", |t| {
            t(|ctx: Context, args: AuthArgs| async move {
                let AuthArgs { email, password } = args;

                let password = tokio::task::spawn_blocking(move || {
                    bcrypt::hash(password, bcrypt::DEFAULT_COST)
                })
                .await
                .map_err(|e| rspc::Error::new(rspc::ErrorCode::InternalServerError, e.to_string()))?
                .map_err(|e| {
                    rspc::Error::new(rspc::ErrorCode::InternalServerError, e.to_string())
                })?;

                let user = ctx
                    .service
                    .users
                    .create_user(email.as_str(), password.as_str())
                    .await
                    .map_err(|e| {
                        rspc::Error::new(
                            rspc::ErrorCode::BadRequest,
                            "Error creating user".to_string(),
                        )
                    })?;

                let session = ctx
                    .service
                    .auth
                    .create_session(user.id)
                    .await
                    .map_err(|e| rspc::Error::new(rspc::ErrorCode::BadRequest, e.to_string()))?;

                let mut cookie = Cookie::new("auth_session", session);

                cookie.set_http_only(true);
                cookie.set_domain("localtest.me");
                cookie.set_path("/");

                ctx.cookies.add(cookie);

                Ok(())
            })
        })
}

pub fn mount_protected() -> rspc::RouterBuilder<ProtectedContext> {
    rspc::Router::<ProtectedContext>::new().query("logout", |t| {
        t(|ctx: ProtectedContext, _: ()| async move {
            ctx.service
                .auth
                .invalidate_session(&ctx.session_token)
                .await
                .map_err(|e| rspc::Error::new(rspc::ErrorCode::BadRequest, e.to_string()))?;

            let mut cookie = Cookie::from("auth_session");

            cookie.set_http_only(true);
            cookie.set_domain("localtest.me");
            cookie.set_path("/");

            ctx.cookies.remove(cookie);

            Ok(())
        })
    })
}
