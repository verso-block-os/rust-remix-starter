use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use rspc::{
    integrations::httpz::{CookieJar, Request},
    BuiltRouter, ExportConfig, Rspc,
};

use crate::{
    core::context::{self, Context},
    service::auth::Auth,
};

mod auth;
mod todos;

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

pub const R: Rspc<Context> = Rspc::new();

pub fn get() -> Arc<BuiltRouter<Context>> {
    let router = R
        .router()
        .procedure("version", R.query(|_, _: ()| Ok("0.0.1")))
        .merge("auth", auth::mount())
        .merge("todos", todos::mount())
        .build()
        .unwrap()
        .arced();

    #[cfg(debug_assertions)]
    router
        .export_ts(ExportConfig::new(
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../web/app/generated/bindings.ts"),
        ))
        .unwrap();

    router
}
