mod models;
mod schema;

use axum::{
    Router,
    extract::{Json, Path, State},
    routing::{get, post, put, delete},
    http::StatusCode,
    response::Json as ResponseJson,
};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use models::{Todo, NewTodo, UpdateTodo};
use std::sync::Arc;
use uuid::Uuid;
use tower::ServiceBuilder;
use tower_http::cors::{CorsLayer, Any};

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    let app = Router::new()
        .route("/todos", get(get_todos).post(create_todo))
        .route("/todos/{id}", get(get_todo).put(update_todo).delete(delete_todo))
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::new()
                    .allow_origin(Any)
                    .allow_methods(Any)
                    .allow_headers(Any))
        )
        .with_state(Arc::new(pool));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8001").await.unwrap();
    println!("Server running on http://0.0.0.0:8001");
    axum::serve(listener, app).await.unwrap();
}

async fn get_todos(State(pool): State<Arc<DbPool>>) -> Result<ResponseJson<Vec<Todo>>, StatusCode> {
    use schema::todos::dsl::*;
    
    let mut conn = pool.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let results = todos
        .select(Todo::as_select())
        .load(&mut conn)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(ResponseJson(results))
}

async fn get_todo(
    State(pool): State<Arc<DbPool>>,
    Path(todo_id): Path<Uuid>
) -> Result<ResponseJson<Todo>, StatusCode> {
    use schema::todos::dsl::*;
    
    let mut conn = pool.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let todo_result = todos
        .find(todo_id)
        .select(Todo::as_select())
        .first(&mut conn)
        .map_err(|_| StatusCode::NOT_FOUND)?;
    
    Ok(ResponseJson(todo_result))
}

async fn create_todo(
    State(pool): State<Arc<DbPool>>,
    Json(new_todo): Json<NewTodo>
) -> Result<ResponseJson<Todo>, StatusCode> {
    use schema::todos::dsl::*;
    
    let mut conn = pool.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let todo_result = diesel::insert_into(todos)
        .values(&new_todo)
        .returning(Todo::as_returning())
        .get_result(&mut conn)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(ResponseJson(todo_result))
}

async fn update_todo(
    State(pool): State<Arc<DbPool>>,
    Path(todo_id): Path<Uuid>,
    Json(update_data): Json<UpdateTodo>
) -> Result<ResponseJson<Todo>, StatusCode> {
    use schema::todos::dsl::*;
    
    let mut conn = pool.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let updated_todo = diesel::update(todos.filter(id.eq(todo_id)))
        .set((
            todo.eq(update_data.todo.unwrap_or_else(|| "".to_string())),
            done.eq(update_data.done.unwrap_or(false)),
            updated_at.eq(Some(chrono::Utc::now())),
        ))
        .returning(Todo::as_returning())
        .get_result(&mut conn)
        .map_err(|_| StatusCode::NOT_FOUND)?;
    
    Ok(ResponseJson(updated_todo))
}

async fn delete_todo(
    State(pool): State<Arc<DbPool>>,
    Path(todo_id): Path<Uuid>
) -> Result<StatusCode, StatusCode> {
    use schema::todos::dsl::*;
    
    let mut conn = pool.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let deleted_count = diesel::delete(todos.filter(id.eq(todo_id)))
        .execute(&mut conn)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    if deleted_count > 0 {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
