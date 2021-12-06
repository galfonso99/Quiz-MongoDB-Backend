use crate::{db::DB, WebResult};
use crate::structs::{Question};
use serde::{Deserialize, Serialize};
use warp::{http::StatusCode, reject, reply::json, Reply};
pub use mongodb::bson::{doc, document::Document};
use chrono::prelude::{Utc};

#[derive(Serialize, Deserialize, Debug)]
pub struct QuizRequest {
    pub title: String,
    pub author: String,
    pub questions: Vec<Question>,
    pub tags: Vec<String>,
}

impl Into<Document> for QuizRequest {
    fn into(self) -> Document {
        doc! {
            "title": self.title,
            "author": self.author,
            "questions": self.questions,
            "added_at": Utc::now(),
            "tags": self.tags,
        }
    }
}

pub async fn fetch_quiz_handler(id: String, db: DB) -> WebResult<impl Reply> {
    let quiz = db.fetch_quiz(&id).await.map_err(|e| reject::custom(e))?;
    Ok(json(&quiz))
}

pub async fn create_quiz_handler(body: QuizRequest, db: DB) -> WebResult<impl Reply> {
    let document_id = db.create_quiz(body).await.map_err(|e| reject::custom(e))?;
    Ok(json(&document_id))
}

pub async fn edit_quiz_handler(id: String, body: QuizRequest, db: DB) -> WebResult<impl Reply> {
    db.edit_quiz(&id, body)
        .await
        .map_err(|e| reject::custom(e))?;
    Ok(StatusCode::OK)
}

pub async fn delete_quiz_handler(id: String, db: DB) -> WebResult<impl Reply> {
    db.delete_quiz(&id).await.map_err(|e| reject::custom(e))?;
    Ok(StatusCode::OK)
}

pub async fn fetch_recent_quizzes_handler(db: DB) -> WebResult<impl Reply> {
    let quizzes = db.fetch_recent_quizzes().await.map_err(|e| reject::custom(e))?;
    Ok(json(&quizzes))
}

pub async fn search_quizzes_handler(title: String, db: DB) -> WebResult<impl Reply> {
    let quizzes = db.search_quizzes(&title).await.map_err(|e| reject::custom(e))?;
    Ok(json(&quizzes))
}

pub async fn quizzes_list_handler(db: DB) -> WebResult<impl Reply> {
    let quizzes = db.fetch_quizzes().await.map_err(|e| reject::custom(e))?;
    Ok(json(&quizzes))
}

pub async fn delete_quizzes_handler(db: DB) -> WebResult<impl Reply> {
    db.delete_quizzes().await.map_err(|e| reject::custom(e))?;
    Ok(StatusCode::OK)
}

