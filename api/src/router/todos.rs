use rspc::Router;

use crate::{
    core::context::{query, Context},
    service::todos::Todos,
};

use super::R;

pub fn mount() -> Router<Context> {
    R.router()
        .procedure(
            "getTodos",
            R.query(|ctx, _: ()| async move {
                let todos = query!(ctx, Todos);
                let todos = todos.get_all().await.map_err(|e| {
                    rspc::Error::new(rspc::ErrorCode::InternalServerError, e.to_string())
                })?;
                Ok(todos)
            }),
        )
        .procedure(
            "createTodo",
            R.mutation(|ctx, title: String| async move {
                let todos = query!(ctx, Todos);
                let todo = todos.create(&title).await.map_err(|e| {
                    rspc::Error::new(rspc::ErrorCode::InternalServerError, e.to_string())
                })?;
                Ok(todo)
            }),
        )
        .procedure(
            "toggleTodo",
            R.mutation(|ctx, id: i32| async move {
                let todos = query!(ctx, Todos);
                let todo = todos.toggle(id).await.map_err(|e| {
                    rspc::Error::new(rspc::ErrorCode::InternalServerError, e.to_string())
                })?;
                Ok(todo)
            }),
        )
        .procedure(
            "deleteTodo",
            R.mutation(|ctx, id: i32| async move {
                let todos = query!(ctx, Todos);
                todos.delete(id).await.map_err(|e| {
                    rspc::Error::new(rspc::ErrorCode::InternalServerError, e.to_string())
                })?;
                Ok(())
            }),
        )
}
