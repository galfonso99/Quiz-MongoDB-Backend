use crate::{db::DB, WebResult, Question};
use serde::{Deserialize, Serialize};
use warp::{http::StatusCode, reject, reply::json, Reply};

#[derive(Serialize, Deserialize, Debug)]
pub struct QuizRequest {
    pub title: String,
    pub author: String,
    pub questions: Vec<Question>,
    pub tags: Vec<String>,
}

pub async fn fetch_quiz_handler(id: String, db: DB) -> WebResult<impl Reply> {
    let quiz = db.fetch_quiz(&id).await.map_err(|e| reject::custom(e))?;
    Ok(json(&quiz))
}

pub async fn create_quiz_handler(body: QuizRequest, db: DB) -> WebResult<impl Reply> {
    db.create_quiz(&body).await.map_err(|e| reject::custom(e))?;
    Ok(StatusCode::CREATED)
}

pub async fn edit_quiz_handler(id: String, body: QuizRequest, db: DB) -> WebResult<impl Reply> {
    db.edit_quiz(&id, &body)
        .await
        .map_err(|e| reject::custom(e))?;
    Ok(StatusCode::OK)
}

pub async fn delete_quiz_handler(id: String, db: DB) -> WebResult<impl Reply> {
    db.delete_quiz(&id).await.map_err(|e| reject::custom(e))?;
    Ok(StatusCode::OK)
}

pub async fn quizzes_list_handler(db: DB) -> WebResult<impl Reply> {
    let quizzes = db.fetch_quizzes().await.map_err(|e| reject::custom(e))?;
    Ok(json(&quizzes))
}
