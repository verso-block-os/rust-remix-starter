use rspc::{Error, ErrorCode, Router};

use crate::{
    core::context::{query, Context},
    service::todos::{Todo, Todos},
};

use super::R;

async fn get_todos(ctx: Context, _: ()) -> Result<Vec<Todo>, Error> {
    let todos = query!(ctx, Todos);
    let todos = todos
        .get_all()
        .await
        .map_err(|e| Error::new(ErrorCode::InternalServerError, e.to_string()))?;
    Ok(todos)
}

async fn create_todo(ctx: Context, title: String) -> Result<Todo, Error> {
    let todos = query!(ctx, Todos);
    let todo = todos
        .create(&title)
        .await
        .map_err(|e| Error::new(ErrorCode::InternalServerError, e.to_string()))?;
    Ok(todo)
}

async fn toggle_todo(ctx: Context, id: i32) -> Result<Todo, Error> {
    let todos = query!(ctx, Todos);
    let todo = todos
        .toggle(id)
        .await
        .map_err(|e| Error::new(ErrorCode::InternalServerError, e.to_string()))?;
    Ok(todo)
}

async fn delete_todo(ctx: Context, id: i32) -> Result<(), Error> {
    let todos = query!(ctx, Todos);
    todos
        .delete(id)
        .await
        .map_err(|e| Error::new(ErrorCode::InternalServerError, e.to_string()))?;
    Ok(())
}

pub fn mount() -> Router<Context> {
    R.router()
        .procedure("getTodos", R.query(get_todos))
        .procedure("createTodo", R.mutation(create_todo))
        .procedure("toggleTodo", R.mutation(toggle_todo))
        .procedure("deleteTodo", R.mutation(delete_todo))
}
