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
        area::{AreaModel, AreaResponseModel},
    },
    schema::{
        FilterOptions,
        area::{CreateAreaSchema, QueryAreaSchema, UpdateAreaSchema},
    },
    util::{AddToQuery, PostgresCmp, SQLQueryBuilder, extract_user_id},
};

// TODO: convert everything to use `map_err` to allow for `?` operator to auto return

pub async fn create_area_handler(
    State(data): State<Arc<AppState>>,
    jar: CookieJar,
    Json(body): Json<CreateAreaSchema>,
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
    query_builder.add_column(AreaModel::USER_ID, &user_id);
    body.add_to_query(&mut query_builder);
    query_builder.set_return_all();

    let (statement, params) = query_builder.build_insert();

    // Insert area into database
    let row = transaction
        .query_one(&statement, &params)
        .await
        .map_err(|e| Error::from(e).err_map())?;

    // Commit transaction
    transaction
        .commit()
        .await
        .map_err(|e| Error::from(e).err_map())?;

    // Get created area
    let area = AreaModel::from(row);

    // Return success response
    let json_message = json!({
        "status": "ok",
        "data": {
            "area": area.to_response(),
        },
    });

    Ok(Json(json_message))
}

pub async fn retrieve_area_handler(
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
    query_builder.set_table(AreaModel::TABLE);
    query_builder.add_condition(AreaModel::USER_ID, PostgresCmp::Equal, &user_id);
    query_builder.add_condition(AreaModel::ID, PostgresCmp::Equal, &id);
    query_builder.set_return_all();

    let (statement, params) = query_builder.build_select();

    // Retrieve area from database
    let row_opt = conn
        .query_opt(&statement, &params)
        .await
        .map_err(|e| Error::from(e).err_map())?;

    // Get retrieved area
    let area = match row_opt {
        Some(row) => AreaModel::from(row),
        None => {
            let json_message = json!({
                "status": "unsuccessful",
                "message": format!("area not found"),
            });

            return Err((StatusCode::NOT_FOUND, Json(json_message)));
        }
    };

    // Return success response
    let json_message = json!({
        "status": "success",
        "data": json!({
            "area": area.to_response(),
        }),
    });

    Ok(Json(json_message))
}

pub async fn update_area_handler(
    State(data): State<Arc<AppState>>,
    jar: CookieJar,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateAreaSchema>,
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
    query_builder.add_column(AreaModel::UPDATED, &timestamp);
    body.add_to_query(&mut query_builder);
    query_builder.add_condition(AreaModel::USER_ID, PostgresCmp::Equal, &user_id);
    query_builder.add_condition(AreaModel::ID, PostgresCmp::Equal, &id);
    query_builder.set_return_all();

    let (statement, params) = query_builder.build_update();

    // Update area in database
    let row_opt = transaction
        .query_opt(&statement, &params)
        .await
        .map_err(|e| Error::from(e).err_map())?;

    // Commit transaction
    transaction
        .commit()
        .await
        .map_err(|e| Error::from(e).err_map())?;

    // Get updated area
    let area = match row_opt {
        Some(row) => AreaModel::from(row),
        None => {
            let json_message = json!({
                "status": "unsuccessful",
                "message": format!("area not found"),
            });

            return Err((StatusCode::NOT_FOUND, Json(json_message)));
        }
    };

    // Return success response
    let json_message = json!({
        "status": "success",
        "data": json!({
            "area": area.to_response(),
        }),
    });

    Ok(Json(json_message))
}

pub async fn delete_area_handler(
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
    query_builder.set_table(AreaModel::TABLE);
    query_builder.add_condition(AreaModel::USER_ID, PostgresCmp::Equal, &user_id);
    query_builder.add_condition(AreaModel::ID, PostgresCmp::Equal, &id);
    query_builder.set_return(vec![AreaModel::ID]);

    let (statement, params) = query_builder.build_delete();

    // Delete area in database
    let row_opt = transaction
        .query_opt(&statement, &params)
        .await
        .map_err(|e| Error::from(e).err_map())?;

    // Commit transaction
    transaction
        .commit()
        .await
        .map_err(|e| Error::from(e).err_map())?;

    // Get deleted area id
    let area_id: Uuid = match row_opt {
        Some(row) => row.get(AreaModel::ID),
        None => {
            let json_message = json!({
                "status": "unsuccessful",
                "message": format!("area not found"),
            });

            return Err((StatusCode::NOT_FOUND, Json(json_message)));
        }
    };

    // Return success message
    let json_message = json!({
        "status": "successful",
        "data": json!({
            "area_id": area_id,
        }),
    });

    Ok(Json(json_message))
}

pub async fn query_area_handler(
    State(data): State<Arc<AppState>>,
    jar: CookieJar,
    Query(opts): Query<FilterOptions>,
    Json(body): Json<QueryAreaSchema>,
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
    query_builder.add_condition(AreaModel::USER_ID, PostgresCmp::Equal, &user_id);
    query_builder.set_limit(limit);
    query_builder.set_offset(offset);

    let (statement, params) = query_builder.build_select();

    // Query areas in database
    let rows = conn
        .query(&statement, &params)
        .await
        .map_err(|e| Error::from(e).err_map())?;

    // Get queried areas
    let areas: Vec<AreaModel> = rows.iter().map(|r| AreaModel::from(r.to_owned())).collect();

    // Return success response
    let area_responses: Vec<AreaResponseModel> = areas.iter().map(|a| a.to_response()).collect();
    let json_message = json!({
        "status": "ok",
        "data": json!({
            "count": area_responses.len(),
            "areas": area_responses,
        }),
    });

    Ok(Json(json_message))
}

// TODO: handler tests?
