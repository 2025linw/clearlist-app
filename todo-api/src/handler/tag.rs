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
    model::{
        ToResponse,
        tag::{TagModel, TagResponseModel},
    },
    schema::{
        FilterOptions,
        tag::{CreateTagSchema, QueryTagSchema, UpdateTagSchema},
    },
    util::{AddToQuery, PostgresCmp, SQLQueryBuilder, extract_user_id},
};

// TODO: convert everything to use `map_err` to allow for `?` operator to auto return

pub async fn create_tag_handler(
    State(data): State<Arc<AppState>>,
    jar: CookieJar,
    Json(body): Json<CreateTagSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Get cookie for user id
    let user_id = extract_user_id(&jar).map_err(|e| {
        let json_message = json!({
            "status": "error",
            "message": format!("Error getting user id: {:?}", e),
        });

        (StatusCode::INTERNAL_SERVER_ERROR, Json(json_message))
    })?;

    // Get connection from pool and then start transaction
    let mut conn = data.get_conn().await.map_err(|e| {
        let json_message = json!({
            "status": "error",
            "message": format!("Error retrieving connection from pool: {:?}", e),
        });

        (StatusCode::INTERNAL_SERVER_ERROR, Json(json_message))
    })?;
    let transaction = conn.transaction().await.map_err(|e| {
        let json_message = json!({
            "status": "error",
            "message": format!("Error starting transaction from connection: {:?}", e),
        });

        (StatusCode::INTERNAL_SERVER_ERROR, Json(json_message))
    })?;

    // Build SQL query
    let mut query_builder = SQLQueryBuilder::new();
    query_builder.add_column(TagModel::USER_ID, &user_id);
    body.add_to_query(&mut query_builder);
    query_builder.set_return_all();

    let (statement, params) = query_builder.build_insert();

    // Insert tag into database
    let row = transaction
        .query_one(&statement, &params)
        .await
        .map_err(|e| {
            let json_message = json!({
                "status": "error",
                "message": format!("Error inserting tag to database: {:#}", e),
            });

            (StatusCode::INTERNAL_SERVER_ERROR, Json(json_message))
        })?;

    // Commit transaction
    if let Err(e) = transaction.commit().await {
        let json_message = json!({
            "status": "error",
            "message": format!("Error commiting transaction to database: {:#?}", e),
        });

        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json_message)));
    }

    // Get created tag
    let tag = TagModel::from(row);

    // Return success response
    let json_message = json!({
        "status": "ok",
        "data": {
            "tag": tag.to_response(),
        },
    });

    Ok(Json(json_message))
}

pub async fn retrieve_tag_handler(
    State(data): State<Arc<AppState>>,
    jar: CookieJar,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Get cookie for user id
    let user_id = extract_user_id(&jar).map_err(|e| {
        let json_message = json!({
            "status": "error",
            "message": format!("Error getting user id: {:?}", e),
        });

        (StatusCode::INTERNAL_SERVER_ERROR, Json(json_message))
    })?;

    // Get connection from pool
    let conn = data.get_conn().await.map_err(|e| {
        let json_message = json!({
            "status": "error",
            "message": format!("Error retrieving connection from pool: {:?}", e),
        });

        (StatusCode::INTERNAL_SERVER_ERROR, Json(json_message))
    })?;

    // Build SQL query
    let mut query_builder = SQLQueryBuilder::new();
    query_builder.set_table(TagModel::TABLE);
    query_builder.add_condition(TagModel::USER_ID, PostgresCmp::Equal, &user_id);
    query_builder.add_condition(TagModel::ID, PostgresCmp::Equal, &id);
    query_builder.set_return_all();

    let (statement, params) = query_builder.build_select();

    // Retrieve tag from database
    let row_opt = conn.query_opt(&statement, &params).await.map_err(|e| {
        let json_message = json!({
            "status": "error",
            "message": format!("Error retrieving tag from database: {:#}", e),
        });

        (StatusCode::INTERNAL_SERVER_ERROR, Json(json_message))
    })?;

    // Get retrieved tag
    let tag = match row_opt {
        Some(row) => TagModel::from(row),
        None => {
            let json_message = json!({
                "status": "unsuccessful",
                "message": format!("tag not found"),
            });

            return Err((StatusCode::NOT_FOUND, Json(json_message)));
        }
    };

    // Return success response
    let json_message = json!({
        "status": "success",
        "data": json!({
            "tag": tag.to_response(),
        }),
    });

    Ok(Json(json_message))
}

pub async fn update_tag_handler(
    State(data): State<Arc<AppState>>,
    jar: CookieJar,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateTagSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Get cookie for user id
    let user_id = extract_user_id(&jar).map_err(|e| {
        let json_message = json!({
            "status": "error",
            "message": format!("Error getting user id: {:?}", e),
        });

        (StatusCode::INTERNAL_SERVER_ERROR, Json(json_message))
    })?;

    // Get connection from pool and then start transaction
    let mut conn = data.get_conn().await.map_err(|e| {
        let json_message = json!({
            "status": "error",
            "message": format!("Error retrieving connection from pool: {:?}", e),
        });

        (StatusCode::INTERNAL_SERVER_ERROR, Json(json_message))
    })?;
    let transaction = conn.transaction().await.map_err(|e| {
        let json_message = json!({
            "status": "error",
            "message": format!("Error starting transaction from connection: {:?}", e),
        });

        (StatusCode::INTERNAL_SERVER_ERROR, Json(json_message))
    })?;

    // Build SQL query
    let timestamp = Local::now();
    let mut query_builder = SQLQueryBuilder::new();
    query_builder.add_column(TagModel::UPDATED, &timestamp);
    body.add_to_query(&mut query_builder);
    query_builder.add_condition(TagModel::USER_ID, PostgresCmp::Equal, &user_id);
    query_builder.add_condition(TagModel::ID, PostgresCmp::Equal, &id);
    query_builder.set_return_all();

    let (statement, params) = query_builder.build_update();

    // Update tag in database
    let row_opt = transaction
        .query_opt(&statement, &params)
        .await
        .map_err(|e| {
            let json_message = json!({
                "status": "error",
                "message": format!("error updating tag in database: {:#}", e),
            });

            (StatusCode::INTERNAL_SERVER_ERROR, Json(json_message))
        })?;

    // Commit transaction
    if let Err(e) = transaction.commit().await {
        let json_message = json!({
            "status": "error",
            "message": format!("error commiting transaction to database: {:#}", e),
        });

        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json_message)));
    }

    // Get updated tag
    let tag = match row_opt {
        Some(row) => TagModel::from(row),
        None => {
            let json_message = json!({
                "status": "unsuccessful",
                "message": format!("tag not found"),
            });

            return Err((StatusCode::NOT_FOUND, Json(json_message)));
        }
    };

    // Return success response
    let json_message = json!({
        "status": "success",
        "data": json!({
            "tag": tag.to_response(),
        }),
    });

    Ok(Json(json_message))
}

pub async fn delete_tag_handler(
    State(data): State<Arc<AppState>>,
    jar: CookieJar,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Get cookie for user id
    let user_id = extract_user_id(&jar).map_err(|e| {
        let json_message = json!({
            "status": "error",
            "message": format!("Error getting user id: {:?}", e),
        });

        (StatusCode::INTERNAL_SERVER_ERROR, Json(json_message))
    })?;

    // Get connection from pool and then start transaction
    let mut conn = data.get_conn().await.map_err(|e| {
        let json_message = json!({
            "status": "error",
            "message": format!("Error retrieving connection from pool: {:?}", e),
        });

        (StatusCode::INTERNAL_SERVER_ERROR, Json(json_message))
    })?;
    let transaction = conn.transaction().await.map_err(|e| {
        let json_message = json!({
            "status": "error",
            "message": format!("Error starting transaction from connection: {:?}", e),
        });

        (StatusCode::INTERNAL_SERVER_ERROR, Json(json_message))
    })?;

    // Build SQL query
    let mut query_builder = SQLQueryBuilder::new();
    query_builder.set_table(TagModel::TABLE);
    query_builder.add_condition(TagModel::USER_ID, PostgresCmp::Equal, &user_id);
    query_builder.add_condition(TagModel::ID, PostgresCmp::Equal, &id);
    query_builder.set_return(vec![TagModel::ID]);

    let (statement, params) = query_builder.build_delete();

    // Delete tag in database
    let row_opt = transaction
        .query_opt(&statement, &params)
        .await
        .map_err(|e| {
            let json_message = json!({
                "status": "error",
                "message": format!("error deleting tag from database: {:#}", e),
            });

            (StatusCode::INTERNAL_SERVER_ERROR, Json(json_message))
        })?;

    // Commit transaction
    if let Err(e) = transaction.commit().await {
        let json_message = json!({
            "status": "error",
            "message": format!("error commiting transaction to database: {:#}", e),
        });

        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json_message)));
    }

    // Get deleted tag id
    let tag_id: Uuid = match row_opt {
        Some(row) => row.get(TagModel::ID),
        None => {
            let json_message = json!({
                "status": "unsuccessful",
                "message": format!("tag not found"),
            });

            return Err((StatusCode::NOT_FOUND, Json(json_message)));
        }
    };

    // Return success message
    let json_message = json!({
        "status": "successful",
        "data": json!({
            "tag_id": tag_id,
        }),
    });

    Ok(Json(json_message))
}

pub async fn query_tag_handler(
    State(data): State<Arc<AppState>>,
    jar: CookieJar,
    Query(opts): Query<FilterOptions>,
    Json(body): Json<QueryTagSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Get pagination info
    let page = opts.page.unwrap_or(1);
    let limit = opts.limit.unwrap_or(25);
    let offset = (page - 1) * limit;

    // Get cookie for user id
    let user_id = extract_user_id(&jar).map_err(|e| {
        let json_message = json!({
            "status": "error",
            "message": format!("Error getting user id: {:?}", e),
        });

        (StatusCode::INTERNAL_SERVER_ERROR, Json(json_message))
    })?;

    // Get connection from pool
    let conn = data.get_conn().await.map_err(|e| {
        let json_message = json!({
            "status": "error",
            "message": format!("Error retrieving connection from pool: {:?}", e),
        });

        (StatusCode::INTERNAL_SERVER_ERROR, Json(json_message))
    })?;

    // Build SQL query
    let mut query_builder = SQLQueryBuilder::new();
    body.add_to_query(&mut query_builder);
    query_builder.add_condition(TagModel::USER_ID, PostgresCmp::Equal, &user_id);
    query_builder.set_limit(limit);
    query_builder.set_offset(offset);

    let (statement, params) = query_builder.build_select();

    // Query tags in database
    let rows = conn.query(&statement, &params).await.map_err(|e| {
        let json_message = json!({
            "status": "error",
            "message": format!("error querying tags in database: {:#}", e),
        });

        (StatusCode::INTERNAL_SERVER_ERROR, Json(json_message))
    })?;

    // Get queried tags
    let tags: Vec<TagModel> = rows.iter().map(|r| TagModel::from(r.to_owned())).collect();

    // Return success response
    let tag_responses: Vec<TagResponseModel> = tags.iter().map(|a| a.to_response()).collect();
    let json_message = json!({
        "status": "ok",
        "data": json!({
            "count": tag_responses.len(),
            "tags": tag_responses,
        }),
    });

    Ok(Json(json_message))
}

// TODO: handler tests?
