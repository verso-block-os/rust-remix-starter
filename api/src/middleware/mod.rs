use std::sync::Mutex;

use rspc::integrations::httpz::{CookieJar, Request};

use crate::{core::context, service::auth::Auth};

pub fn cookies() -> context::middleware!() {
    |mw, mut ctx| async move {
        let request = context::query!(ctx, Mutex<Request>);
        let mut request = request.lock().unwrap();
        let cookies = request.cookies().ok_or_else(|| {
            rspc::Error::new(
                rspc::ErrorCode::InternalServerError,
                "Failed to find cookies in the request.".to_string(),
            )
        })?;

        context::add!(ctx, cookies);

        Ok(mw.next(ctx))
    }
}

pub fn auth() -> context::middleware!() {
    |mw, mut ctx| async move {
        let (cookies, auth) = context::query!(ctx, CookieJar, Auth);
        let cookie = cookies.get("auth_session").ok_or_else(|| {
            rspc::Error::new(rspc::ErrorCode::BadRequest, "Not authenticated".to_string())
        })?;

        let session = auth
            .validate_session(cookie.value())
            .await
            .map_err(|e| rspc::Error::new(rspc::ErrorCode::BadRequest, e.to_string()))?
            .ok_or_else(|| {
                rspc::Error::new(rspc::ErrorCode::BadRequest, "Invalid session".to_string())
            })?;

        context::add!(ctx, session);

        Ok(mw.next(ctx))
    }
}
