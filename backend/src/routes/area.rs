use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_jwt_auth::Claims;
use serde_json::json;
use uuid::Uuid;

use crate::{
    AppState,
    error::ErrorResponse,
    models::{
        FilterOptions, ToResponse,
        area::{CreateSchema, DatabaseModel, QuerySchema, ResponseModel, UpdateSchema},
        auth::token::Claim,
    },
    util::{PostgresCmp, SQLQueryBuilder, ToSQLQueryBuilder},
};

const NOT_FOUND: &str = "area not found";
const NO_UPDATE: &str = "no area updates were requested";

pub async fn create_handler(
    Claims(claim): Claims<Claim>,
    State(data): State<AppState>,
    Json(body): Json<CreateSchema>,
) -> Result<impl IntoResponse, ErrorResponse> {
    // Get user id
    let user_id = claim.sub;

    // Create area
    let mut query_builder = body.to_sql_builder();
    query_builder.add_column(DatabaseModel::USER_ID, &user_id);

    let (statement, params) = query_builder.build_insert();

    // TODO: should the row response be used?
    data.db_conn.query_insert(statement, params).await?;

    // Return
    Ok(StatusCode::CREATED)
}

pub async fn retrieve_handler(
    Claims(claim): Claims<Claim>,
    State(data): State<AppState>,
    Path(area_id): Path<Uuid>,
) -> Result<impl IntoResponse, ErrorResponse> {
    // Get user id
    let user_id = claim.sub;

    // Retrieve area
    let mut query_builder = SQLQueryBuilder::new(DatabaseModel::TABLE);
    query_builder.add_condition(DatabaseModel::USER_ID, PostgresCmp::Equal, &user_id);
    query_builder.add_condition(DatabaseModel::ID, PostgresCmp::Equal, &area_id);
    query_builder.set_return_all();

    let (statement, params) = query_builder.build_select();

    let row = match data.db_conn.query_select_one(statement, params).await? {
        Some(r) => r,
        None => return Err(ErrorResponse::new(StatusCode::NOT_FOUND, NOT_FOUND)),
    };

    let area = DatabaseModel::from(row);

    // Return
    Ok(Json(json!({
        "status": "success",
        "data": json!({
            "area": area.to_response(),
        }),
    })))
}

pub async fn update_handler(
    Claims(claim): Claims<Claim>,
    State(data): State<AppState>,
    Path(area_id): Path<Uuid>,
    Json(body): Json<UpdateSchema>,
) -> Result<impl IntoResponse, ErrorResponse> {
    // Get user id
    let user_id = claim.sub;

    // If no updates made
    if body.is_empty() {
        return Err(ErrorResponse::new(StatusCode::BAD_REQUEST, NO_UPDATE));
    }

    // Update area
    let mut query_builder = body.to_sql_builder();
    query_builder.add_condition(DatabaseModel::USER_ID, PostgresCmp::Equal, &user_id);
    query_builder.add_condition(DatabaseModel::ID, PostgresCmp::Equal, &area_id);
    query_builder.set_return_all();

    let (statement, params) = query_builder.build_update();

    let row = match data.db_conn.query_update(statement, params).await? {
        Some(r) => r,
        None => return Err(ErrorResponse::new(StatusCode::NOT_FOUND, NOT_FOUND)),
    };

    let area = DatabaseModel::from(row);

    // Return
    Ok(Json(json!({
        "status": "success",
        "data": json!({
            "area": area.to_response(),
        }),
    })))
}

pub async fn delete_handler(
    Claims(claim): Claims<Claim>,
    State(data): State<AppState>,
    Path(area_id): Path<Uuid>,
) -> Result<impl IntoResponse, ErrorResponse> {
    // Get user id
    let user_id = claim.sub;

    // Delete area
    let mut query_builder = SQLQueryBuilder::new(DatabaseModel::TABLE);
    query_builder.add_condition(DatabaseModel::USER_ID, PostgresCmp::Equal, &user_id);
    query_builder.add_condition(DatabaseModel::ID, PostgresCmp::Equal, &area_id);
    query_builder.set_return(&[DatabaseModel::ID]);

    let (statement, params) = query_builder.build_delete();

    if !data.db_conn.query_delete(statement, params).await? {
        return Err(ErrorResponse::new(StatusCode::NOT_FOUND, NOT_FOUND));
    }

    // Return
    Ok(StatusCode::NO_CONTENT)
}

pub async fn query_handler(
    Claims(claim): Claims<Claim>,
    State(data): State<AppState>,
    Query(opts): Query<FilterOptions>,
    Json(body): Json<QuerySchema>,
) -> Result<impl IntoResponse, ErrorResponse> {
    // Get user id
    let user_id = claim.sub;

    // Get pagination info
    let page = opts.page.unwrap_or(1);
    let limit = opts.limit.unwrap_or(25);
    let offset = (page - 1) * limit;

    // Query areas
    let mut query_builder = body.to_sql_builder();
    query_builder.add_condition(DatabaseModel::USER_ID, PostgresCmp::Equal, &user_id);
    query_builder.set_limit(limit);
    query_builder.set_offset(offset);

    let (statement, params) = query_builder.build_select();

    let rows = data.db_conn.query_select_many(statement, params).await?;

    let areas: Vec<ResponseModel> = rows
        .iter()
        .map(|r| DatabaseModel::from(r.to_owned()))
        .map(|a| a.to_response())
        .collect();

    // Return
    Ok(Json(json!({
        "status": "ok",
        "data": json!({
            "count": areas.len(),
            "areas": areas,
        }),
    })))
}

// TEST: area handlers
