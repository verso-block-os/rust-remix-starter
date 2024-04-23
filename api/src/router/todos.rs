use super::Context;

pub fn mount() -> rspc::RouterBuilder<Context> {
    rspc::Router::<Context>::new()
        .query("getTodos", |t| {
            t(|ctx: Context, _: ()| async move {
                let todos = ctx.service.todos.get_all().await.map_err(|e| {
                    rspc::Error::new(rspc::ErrorCode::InternalServerError, e.to_string())
                })?;

                Ok(todos)
            })
        })
        .mutation("createTodo", |t| {
            t(|ctx: Context, title: String| async move {
                let todo = ctx.service.todos.create(&title).await.map_err(|e| {
                    rspc::Error::new(rspc::ErrorCode::InternalServerError, e.to_string())
                })?;

                Ok(todo)
            })
        })
        .mutation("toggleTodo", |t| {
            t(|ctx: Context, id: i32| async move {
                let todo = ctx.service.todos.toggle(id).await.map_err(|e| {
                    rspc::Error::new(rspc::ErrorCode::InternalServerError, e.to_string())
                })?;

                Ok(todo)
            })
        })
        .mutation("deleteTodo", |t| {
            t(|ctx: Context, id: i32| async move {
                ctx.service.todos.delete(id).await.map_err(|e| {
                    rspc::Error::new(rspc::ErrorCode::InternalServerError, e.to_string())
                })?;

                Ok(())
            })
        })
}
