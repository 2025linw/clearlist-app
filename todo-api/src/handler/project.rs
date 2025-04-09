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
        project::{ProjectModel, ProjectResponseModel},
    },
    schema::{
        FilterOptions,
        project::{CreateProjectSchema, QueryProjectSchema, UpdateProjectSchema},
    },
    util::{AddToQuery, PostgresCmp, SQLQueryBuilder, extract_user_id},
};

// TODO: convert everything to use `map_err` to allow for `?` operator to auto return

pub async fn create_project_handler(
    State(data): State<Arc<AppState>>,
    jar: CookieJar,
    Json(body): Json<CreateProjectSchema>,
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
    query_builder.add_column(ProjectModel::USER_ID, &user_id);
    body.add_to_query(&mut query_builder);
    query_builder.set_return_all();

    let (statement, params) = query_builder.build_insert();

    // Insert project into database
    let row = transaction
        .query_one(&statement, &params)
        .await
        .map_err(|e| Error::from(e).err_map())?;

    // Commit transaction
    transaction
        .commit()
        .await
        .map_err(|e| Error::from(e).err_map())?;

    // Get created project
    let project = ProjectModel::from(row);

    // Return success response
    let json_response = json!({
        "status": "success",
        "data": json!({
            "project": project.to_response(),
        }),
    });

    Ok(Json(json_response))
}

pub async fn retrieve_project_handler(
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
    query_builder.set_table(ProjectModel::TABLE);
    query_builder.add_condition(ProjectModel::USER_ID, PostgresCmp::Equal, &user_id);
    query_builder.add_condition(ProjectModel::ID, PostgresCmp::Equal, &id);
    query_builder.set_return_all();

    let (statement, params) = query_builder.build_select();

    // Retrieve project from database
    let row_opt = conn
        .query_opt(&statement, &params)
        .await
        .map_err(|e| Error::from(e).err_map())?;

    // Get retrieved project
    let project = match row_opt {
        Some(row) => ProjectModel::from(row),
        None => {
            let json_message = json!({
                "status": "unsuccessful",
                "message": format!("project not found"),
            });

            return Err((StatusCode::NOT_FOUND, Json(json_message)));
        }
    };

    // Return success response
    let json_response = json!({
        "status": "success",
        "data": json!({
            "project": project.to_response(),
        }),
    });

    Ok(Json(json_response))
}

pub async fn update_project_handler(
    State(data): State<Arc<AppState>>,
    jar: CookieJar,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateProjectSchema>,
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
    query_builder.add_column(ProjectModel::UPDATED, &timestamp);
    body.add_to_query(&mut query_builder);
    query_builder.add_condition(ProjectModel::USER_ID, PostgresCmp::Equal, &user_id);
    query_builder.add_condition(ProjectModel::ID, PostgresCmp::Equal, &id);
    query_builder.set_return_all();

    let (statement, params) = query_builder.build_update();

    // Update project in database
    let row_opt = transaction
        .query_opt(&statement, &params)
        .await
        .map_err(|e| Error::from(e).err_map())?;

    // Commit transaction
    transaction
        .commit()
        .await
        .map_err(|e| Error::from(e).err_map())?;

    // Get updated project
    let project = match row_opt {
        Some(row) => ProjectModel::from(row),
        None => {
            let json_message = json!({
                "status": "unsuccessful",
                "message": format!("project not found"),
            });

            return Err((StatusCode::NOT_FOUND, Json(json_message)));
        }
    };

    // Return success response
    let json_message = json!({
        "status": "success",
        "data": json!({
            "project": project.to_response(),
        }),
    });

    Ok(Json(json_message))
}

pub async fn delete_project_handler(
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
    query_builder.set_table(ProjectModel::TABLE);
    query_builder.add_condition(ProjectModel::USER_ID, PostgresCmp::Equal, &user_id);
    query_builder.add_condition(ProjectModel::ID, PostgresCmp::Equal, &id);
    query_builder.set_return(vec![ProjectModel::ID]);

    let (statement, params) = query_builder.build_delete();

    // Delete project in database
    let row_opt = transaction
        .query_opt(&statement, &params)
        .await
        .map_err(|e| {
            let json_message = json!({
                "status": "error",
                "message": format!("error deleting project from database: {:#}", e),
            });

            (StatusCode::INTERNAL_SERVER_ERROR, Json(json_message))
        })?;

    // Commit transaction
    transaction
        .commit()
        .await
        .map_err(|e| Error::from(e).err_map())?;

    // Get deleted project id
    let project_id: Uuid = match row_opt {
        Some(row) => row.get(ProjectModel::ID),
        None => {
            let json_message = json!({
                "status": "unsuccessful",
                "message": format!("project not found"),
            });

            return Err((StatusCode::NOT_FOUND, Json(json_message)));
        }
    };

    // Return success message
    let json_message = json!({
        "status": "successful",
        "data": json!({
            "project_id": project_id,
        }),
    });

    Ok(Json(json_message))
}

pub async fn query_project_handler(
    State(data): State<Arc<AppState>>,
    jar: CookieJar,
    Query(opts): Query<FilterOptions>,
    Json(body): Json<QueryProjectSchema>,
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
    query_builder.add_condition(ProjectModel::USER_ID, PostgresCmp::Equal, &user_id);
    query_builder.set_limit(limit);
    query_builder.set_offset(offset);

    let (statement, params) = query_builder.build_select();

    // Query projects in database
    let rows = conn
        .query(&statement, &params)
        .await
        .map_err(|e| Error::from(e).err_map())?;

    // Get queried projects
    let projects: Vec<ProjectModel> = rows
        .iter()
        .map(|r| ProjectModel::from(r.to_owned()))
        .collect();

    // Return success response
    let project_responses: Vec<ProjectResponseModel> =
        projects.iter().map(|p| p.to_response()).collect();
    let json_message = json!({
        "status": "ok",
        "data": json!({
            "count": project_responses.len(),
            "projects": project_responses,
        }),
    });

    Ok(Json(json_message))
}

// TODO: handler tests?
