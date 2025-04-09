use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::CookieJar;
use chrono::Local;
use serde_json::json;
use uuid::Uuid;

use crate::{
    AppState,
    error::Error,
    model::{
        ToResponse,
        task::{TaskModel, TaskResponseModel},
    },
    schema::{
        FilterOptions,
        task::{CreateTaskSchema, QueryTaskSchema, UpdateTaskSchema},
    },
    util::{AddToQuery, PostgresCmp, SQLQueryBuilder, extract_user_id},
};

pub async fn create_task_handler(
    State(data): State<Arc<AppState>>,
    jar: CookieJar,
    Json(body): Json<CreateTaskSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Get cookie for user id
    let user_id = extract_user_id(&jar).map_err(|e| e.err_map())?;

    // Get connection from pool and then start transaction
    let mut conn = data.get_conn().await.map_err(|e| e.err_map())?;
    let transaction = conn
        .transaction()
        .await
        .map_err(|e| Error::from(e).err_map())?;

    // Build SQL query
    let mut query_builder = SQLQueryBuilder::new();
    query_builder.add_column(TaskModel::USER_ID, &user_id);
    body.add_to_query(&mut query_builder);
    query_builder.set_return_all();

    let (statement, params) = query_builder.build_insert();

    // Insert task into database
    let row = transaction
        .query_one(&statement, &params)
        .await
        .map_err(|e| Error::from(e).err_map())?;

    // Commit transaction
    transaction
        .commit()
        .await
        .map_err(|e| Error::from(e).err_map())?;

    // Get created task
    let task = TaskModel::from(row);

    // Return success response
    let json_response = json!({
        "status": "success",
        "data": json!({
            "task": task.to_response(),
        }),
    });

    Ok(Json(json_response))
}

pub async fn retrieve_task_handler(
    State(data): State<Arc<AppState>>,
    jar: CookieJar,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Get cookie for user id
    let user_id = extract_user_id(&jar).map_err(|e| e.err_map())?;

    // Get connection from pool
    let conn = data.get_conn().await.map_err(|e| e.err_map())?;

    // Build SQL query
    let mut query_builder = SQLQueryBuilder::new();
    query_builder.set_table(TaskModel::TABLE);
    query_builder.add_condition(TaskModel::USER_ID, PostgresCmp::Equal, &user_id);
    query_builder.add_condition(TaskModel::ID, PostgresCmp::Equal, &id);
    query_builder.set_return_all();

    let (statement, params) = query_builder.build_select();

    // Retrieve task from database
    let row_opt = conn
        .query_opt(&statement, &params)
        .await
        .map_err(|e| Error::from(e).err_map())?;

    // Get retrieved task
    let task = match row_opt {
        Some(row) => TaskModel::from(row),
        None => {
            let json_message = json!({
                "status": "unsuccessful",
                "message": format!("task not found"),
            });

            return Err((StatusCode::NOT_FOUND, Json(json_message)));
        }
    };

    // Return success response
    let json_response = json!({
        "status": "success",
        "data": json!({
            "task": task.to_response(),
        }),
    });

    Ok(Json(json_response))
}

pub async fn update_task_handler(
    State(data): State<Arc<AppState>>,
    jar: CookieJar,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateTaskSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Get cookie for user id
    let user_id = extract_user_id(&jar).map_err(|e| e.err_map())?;

    // Get connection from pool and then start transaction
    let mut conn = data.get_conn().await.map_err(|e| e.err_map())?;
    let transaction = conn
        .transaction()
        .await
        .map_err(|e| Error::from(e).err_map())?;

    // Build SQL query
    let timestamp = Local::now();
    let mut query_builder = SQLQueryBuilder::new();
    query_builder.add_column(TaskModel::UPDATED, &timestamp);
    body.add_to_query(&mut query_builder);
    query_builder.add_condition(TaskModel::USER_ID, PostgresCmp::Equal, &user_id);
    query_builder.add_condition(TaskModel::ID, PostgresCmp::Equal, &id);
    query_builder.set_return_all();

    let (statement, params) = query_builder.build_update();

    // Update task in database
    let row_opt = transaction
        .query_opt(&statement, &params)
        .await
        .map_err(|e| Error::from(e).err_map())?;

    // Commit transaction
    transaction
        .commit()
        .await
        .map_err(|e| Error::from(e).err_map())?;

    // Get updated task
    let task = match row_opt {
        Some(row) => TaskModel::from(row),
        None => {
            let json_message = json!({
                "status": "unsuccessful",
                "message": format!("task not found"),
            });

            return Err((StatusCode::NOT_FOUND, Json(json_message)));
        }
    };

    // Return success response
    let json_message = json!({
        "status": "success",
        "data": json!({
            "task": task.to_response(),
        }),
    });

    Ok(Json(json_message))
}

pub async fn delete_task_handler(
    State(data): State<Arc<AppState>>,
    jar: CookieJar,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Get cookie for user id
    let user_id = extract_user_id(&jar).map_err(|e| e.err_map())?;

    // Get connection from pool and then start transaction
    let mut conn = data.get_conn().await.map_err(|e| e.err_map())?;
    let transaction = conn
        .transaction()
        .await
        .map_err(|e| Error::from(e).err_map())?;

    // Build SQL query
    let mut query_builder = SQLQueryBuilder::new();
    query_builder.set_table(TaskModel::TABLE);
    query_builder.add_condition(TaskModel::USER_ID, PostgresCmp::Equal, &user_id);
    query_builder.add_condition(TaskModel::ID, PostgresCmp::Equal, &id);
    query_builder.set_return(vec![TaskModel::ID]);

    let (statement, params) = query_builder.build_delete();

    // Delete task in database
    let row_opt = transaction
        .query_opt(&statement, &params)
        .await
        .map_err(|e| Error::from(e).err_map())?;

    // Commit transaction
    transaction
        .commit()
        .await
        .map_err(|e| Error::from(e).err_map())?;

    // Get deleted task id
    let task_id: Uuid = match row_opt {
        Some(row) => row.get(TaskModel::ID),
        None => {
            let json_message = json!({
                "status": "unsuccessful",
                "message": format!("task not found"),
            });

            return Err((StatusCode::NOT_FOUND, Json(json_message)));
        }
    };

    // Return success message
    let json_message = json!({
        "status": "successful",
        "data": json!({
            "task_id": task_id,
        }),
    });

    Ok(Json(json_message))
}

pub async fn query_task_handler(
    State(data): State<Arc<AppState>>,
    jar: CookieJar,
    Query(opts): Query<FilterOptions>,
    Json(body): Json<QueryTaskSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Get pagination info
    let page = opts.page.unwrap_or(1);
    let limit = opts.limit.unwrap_or(25);
    let offset = (page - 1) * limit;

    // Get cookie for user id
    let user_id = extract_user_id(&jar).map_err(|e| e.err_map())?;

    // Get connection from pool
    let conn = data.get_conn().await.map_err(|e| e.err_map())?;

    // Build SQL query
    let mut query_builder = SQLQueryBuilder::new();
    body.add_to_query(&mut query_builder);
    query_builder.add_condition(TaskModel::USER_ID, PostgresCmp::Equal, &user_id);
    query_builder.set_limit(limit);
    query_builder.set_offset(offset);

    let (statement, params) = query_builder.build_select();

    // Query tasks in database
    let rows = conn
        .query(&statement, &params)
        .await
        .map_err(|e| Error::from(e).err_map())?;

    // Get queried tasks
    let tasks: Vec<TaskModel> = rows.iter().map(|r| TaskModel::from(r.to_owned())).collect();

    // Return success response
    let task_responses: Vec<TaskResponseModel> = tasks.iter().map(|t| t.to_response()).collect();
    let json_message = json!({
        "status": "ok",
        "data": json!({
            "count": task_responses.len(),
            "tasks": task_responses,
        }),
    });

    Ok(Json(json_message))
}

// TODO: handler tests?
