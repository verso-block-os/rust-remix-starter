use std::sync::Arc;

use tower_cookies::Cookies;

use crate::service::Service;

mod auth;
mod todos;

#[derive(Debug, Clone)]
pub struct Context {
    pub service: Arc<Service>,
    pub cookies: Cookies,
}

pub struct ProtectedContext {
    pub service: Arc<Service>,
    pub cookies: Cookies,
    pub user_id: i32,
    pub session_token: String,
}

pub fn get_router() -> Arc<rspc::Router<Context>> {
    rspc::Router::<Context>::new()
        .query("version", |t| t(|_ctx: Context, _: ()| "1.0.0"))
        .merge("todos.", todos::mount())
        .merge("auth.", auth::mount())
        .middleware(|mw| {
            mw.middleware(|mw| async move {
                let ctx = mw.ctx.clone();
                let cookie = ctx.cookies.get("auth_session").ok_or_else(|| {
                    rspc::Error::new(rspc::ErrorCode::BadRequest, "Not authenticated".to_string())
                })?;

                let session = ctx
                    .service
                    .auth
                    .validate_session(cookie.value())
                    .await
                    .map_err(|e| rspc::Error::new(rspc::ErrorCode::BadRequest, e.to_string()))?
                    .ok_or_else(|| {
                        rspc::Error::new(rspc::ErrorCode::BadRequest, "Invalid session".to_string())
                    })?;

                Ok(mw.with_ctx(ProtectedContext {
                    service: ctx.service.clone(),
                    cookies: ctx.cookies.clone(),
                    user_id: session.user_id,
                    session_token: session.token,
                }))
            })
        })
        .merge("auth.", auth::mount_protected())
        .config(rspc::Config::new().export_ts_bindings("../web/app/generated/bindings.ts"))
        .build()
        .arced()
}
