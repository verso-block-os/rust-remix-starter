use std::{path::PathBuf, sync::Arc};

use rspc::{BuiltRouter, ExportConfig, Rspc};

use crate::core::context::Context;

mod auth;
mod todos;

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
