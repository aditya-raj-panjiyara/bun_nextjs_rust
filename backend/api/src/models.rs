use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::schema::todos;

#[derive(Debug, Queryable, Selectable, Serialize)]
#[diesel(table_name = todos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Todo {
    pub id: Uuid,
    pub todo: String,
    pub done: bool,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = todos)]
pub struct NewTodo {
    pub todo: String,
    pub done: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTodo {
    pub todo: Option<String>,
    pub done: Option<bool>,
}

// Keep the old Data struct for backward compatibility
#[derive(Debug, Deserialize, Serialize)]
pub struct Data {
    pub todo: String,
    pub done: bool,
}
