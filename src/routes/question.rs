use std::collections::HashMap;
use warp::hyper::StatusCode;
use tracing::{instrument, Level};

use crate::store::Store;

use crate::types::{
    pagination::{Pagination, extract_pagination},
    question::{Question, NewQuestion},
};

#[instrument]
pub async fn get_questions(
    params: HashMap<String, String>,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    tracing::event!(target: "minimal_warp", tracing::Level::INFO, "querying questions");
    let mut pagination = Pagination::default();

    if !params.is_empty() {
        tracing::event!(Level::INFO, pagination = true);
        pagination = extract_pagination(params)?;
    }
        let res: Vec<Question> = match store.get_questions(pagination.limit, pagination.offset).await {
            Ok(res) => res,
            Err(e) => return Err(warp::reject::custom(e)),
        };

        Ok(warp::reply::json(&res))
}

pub async fn add_question(
    store: Store,
    new_question: NewQuestion,
) -> Result<impl warp::Reply, warp::Rejection> {
    if let Err(e) = store.add_question(new_question).await {
        return Err(warp::reject::custom(e));
    }

    Ok(warp::reply::with_status("Question added", StatusCode::OK))
}

pub async fn update_question(
    id: i32,
    store: Store,
    question: Question,
) -> Result<impl warp::Reply, warp::Rejection> {
    let res = match store.update_question(question, id).await {
        Ok(res) => res,
        Err(e) => return Err(warp::reject::custom(e)),
    };

    Ok(warp::reply::json(&res))
}

pub async fn delete_question(
    id: i32,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    if let Err(e) = store.delete_question(id).await {
        return Err(warp::reject::custom(e));
    }

    Ok(warp::reply::with_status(format!("Question {} deleted", id), StatusCode::OK))
}
