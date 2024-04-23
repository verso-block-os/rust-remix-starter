use std::sync::Arc;

use crate::database::Database;

#[derive(Debug, Clone)]
pub struct Context {
    pub database: Arc<Database>,
}

pub fn get_router() -> Arc<rspc::Router<Context>> {
    rspc::Router::<Context>::new()
        .query("version", |t| t(|_ctx: Context, _: ()| "1.0.0"))
        .query("getTodos", |t| {
            t(|ctx: Context, _: ()| async move {
                let todos = ctx.database.todos.get_all().await.unwrap();

                Ok(todos)
            })
        })
        .mutation("createTodo", |t| {
            t(|ctx: Context, title: String| async move {
                let todo = ctx.database.todos.create(&title).await.unwrap();

                Ok(todo)
            })
        })
        .mutation("toggleTodo", |t| {
            t(|ctx: Context, id: i32| async move {
                let todo = ctx.database.todos.toggle(id).await.unwrap();

                Ok(todo)
            })
        })
        .mutation("deleteTodo", |t| {
            t(|ctx: Context, id: i32| async move {
                ctx.database.todos.delete(id).await.unwrap();

                Ok(())
            })
        })
        .config(rspc::Config::new().export_ts_bindings("../web/app/generated/bindings.ts"))
        .build()
        .arced()
}
