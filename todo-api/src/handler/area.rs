use std::sync::Arc;

use axum::{
    Json,
    extract::{Extension, Path, Query},
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

pub async fn create_area_handler(
    Extension(data): Extension<Arc<AppState>>,
    jar: CookieJar,
    Json(body): Json<CreateAreaSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Get user id
    let user_id = extract_user_id(&jar).map_err(|e| e.to_axum_response())?;

    // Get database connection and start transaction
    let mut conn = data.get_conn().await.map_err(|e| e.to_axum_response())?;
    let transaction = conn
        .transaction()
        .await
        .map_err(|e| Error::from(e).to_axum_response())?;

    // Create area
    let mut query_builder = SQLQueryBuilder::new(AreaModel::TABLE);
    query_builder.add_column(AreaModel::USER_ID, &user_id);
    body.add_to_query(&mut query_builder);
    query_builder.set_return_all();

    let (statement, params) = query_builder.build_insert();

    let row = transaction
        .query_one(&statement, &params)
        .await
        .map_err(|e| Error::from(e).to_axum_response())?;

    // Commit transaction
    transaction
        .commit()
        .await
        .map_err(|e| Error::from(e).to_axum_response())?;

    let area = AreaModel::from(row);

    Ok((
        StatusCode::CREATED,
        Json(json!({
            "status": "ok",
            "data": {
                "area": area.to_response(),
            },
        })),
    ))
}

pub async fn retrieve_area_handler(
    Extension(data): Extension<Arc<AppState>>,
    jar: CookieJar,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Get user id
    let user_id = extract_user_id(&jar).map_err(|e| e.to_axum_response())?;

    // Get database connection
    let conn = data.get_conn().await.map_err(|e| e.to_axum_response())?;

    // Retrieve area
    let mut query_builder = SQLQueryBuilder::new(AreaModel::TABLE);
    query_builder.add_condition(AreaModel::USER_ID, PostgresCmp::Equal, &user_id);
    query_builder.add_condition(AreaModel::ID, PostgresCmp::Equal, &id);
    query_builder.set_return_all();

    let (statement, params) = query_builder.build_select();

    let row_opt = conn
        .query_opt(&statement, &params)
        .await
        .map_err(|e| Error::from(e).to_axum_response())?;

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

    Ok(Json(json!({
        "status": "success",
        "data": json!({
            "area": area.to_response(),
        }),
    })))
}

pub async fn update_area_handler(
    Extension(data): Extension<Arc<AppState>>,
    jar: CookieJar,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateAreaSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Get user id
    let user_id = extract_user_id(&jar).map_err(|e| e.to_axum_response())?;

    // Get database connection and start transaction
    let mut conn = data.get_conn().await.map_err(|e| e.to_axum_response())?;
    let transaction = conn
        .transaction()
        .await
        .map_err(|e| Error::from(e).to_axum_response())?;

    // Update area
    let timestamp = Local::now();
    let mut query_builder = SQLQueryBuilder::new(AreaModel::TABLE);
    query_builder.add_column(AreaModel::UPDATED, &timestamp);
    body.add_to_query(&mut query_builder);
    query_builder.add_condition(AreaModel::USER_ID, PostgresCmp::Equal, &user_id);
    query_builder.add_condition(AreaModel::ID, PostgresCmp::Equal, &id);
    query_builder.set_return_all();

    let (statement, params) = query_builder.build_update();

    let row_opt = transaction
        .query_opt(&statement, &params)
        .await
        .map_err(|e| Error::from(e).to_axum_response())?;

    if row_opt.is_none() {
        let json_message = json!({
            "status": "unsuccessful",
            "message": format!("area not found"),
        });

        return Err((StatusCode::NOT_FOUND, Json(json_message)));
    }

    // Commit transaction
    transaction
        .commit()
        .await
        .map_err(|e| Error::from(e).to_axum_response())?;

    // Get updated area
    let area = AreaModel::from(row_opt.unwrap());

    Ok(Json(json!({
        "status": "success",
        "data": json!({
            "area": area.to_response(),
        }),
    })))
}

pub async fn delete_area_handler(
    Extension(data): Extension<Arc<AppState>>,
    jar: CookieJar,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Get user id
    let user_id = extract_user_id(&jar).map_err(|e| e.to_axum_response())?;

    // Get database connection and start transaction
    let mut conn = data.get_conn().await.map_err(|e| e.to_axum_response())?;
    let transaction = conn
        .transaction()
        .await
        .map_err(|e| Error::from(e).to_axum_response())?;

    // Delete area
    let mut query_builder = SQLQueryBuilder::new(AreaModel::TABLE);
    query_builder.add_condition(AreaModel::USER_ID, PostgresCmp::Equal, &user_id);
    query_builder.add_condition(AreaModel::ID, PostgresCmp::Equal, &id);
    query_builder.set_return(vec![AreaModel::ID]);

    let (statement, params) = query_builder.build_delete();

    let row_opt = transaction
        .query_opt(&statement, &params)
        .await
        .map_err(|e| Error::from(e).to_axum_response())?;

    // Commit transaction
    transaction
        .commit()
        .await
        .map_err(|e| Error::from(e).to_axum_response())?;

    if row_opt.is_none() {
        let json_message = json!({
            "status": "unsuccessful",
            "message": format!("area not found"),
        });

        return Err((StatusCode::NOT_FOUND, Json(json_message)));
    }

    Ok(StatusCode::NO_CONTENT)
}

pub async fn query_area_handler(
    Extension(data): Extension<Arc<AppState>>,
    jar: CookieJar,
    Query(opts): Query<FilterOptions>,
    Json(body): Json<QueryAreaSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Get user id
    let user_id = extract_user_id(&jar).map_err(|e| e.to_axum_response())?;

    // Get database connection
    let conn = data.get_conn().await.map_err(|e| e.to_axum_response())?;

    // Get pagination info
    let page = opts.page.unwrap_or(1);
    let limit = opts.limit.unwrap_or(25);
    let offset = (page - 1) * limit;

    // Query areas
    let mut query_builder = SQLQueryBuilder::new(AreaModel::TABLE);
    body.add_to_query(&mut query_builder);
    query_builder.add_condition(AreaModel::USER_ID, PostgresCmp::Equal, &user_id);
    query_builder.set_limit(limit);
    query_builder.set_offset(offset);

    let (statement, params) = query_builder.build_select();

    let rows = conn
        .query(&statement, &params)
        .await
        .map_err(|e| Error::from(e).to_axum_response())?;

    let areas: Vec<AreaModel> = rows.iter().map(|r| AreaModel::from(r.to_owned())).collect();

    let area_responses: Vec<AreaResponseModel> = areas.iter().map(|a| a.to_response()).collect();

    Ok(Json(json!({
        "status": "ok",
        "data": json!({
            "count": area_responses.len(),
            "areas": area_responses,
        }),
    })))
}

// TEST: handler tests?
