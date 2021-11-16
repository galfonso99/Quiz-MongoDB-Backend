use chrono::prelude::*;
use db::{DB, doc};
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use warp::{Filter, Rejection};
use mongodb::bson::Bson;

type Result<T> = std::result::Result<T, error::Error>;
type WebResult<T> = std::result::Result<T, Rejection>;

mod db;
mod error;
mod handler;


// pub struct Book {
//     pub id: String,
//     pub name: String,
//     pub author: String,
//     pub num_pages: usize,
//     pub added_at: DateTime<Utc>,
//     pub tags: Vec<String>,
// }

#[derive(Serialize, Deserialize, Debug, Clone,)]
pub struct Question {
    question: String,
    correct_answer: String,
    incorrect_answers: Vec<String>
  }
  
  #[derive(Serialize, Deserialize, Debug)]
  pub struct Quiz {
    pub id: String,
    pub title: String,
    pub author: String,
    pub questions: Vec<Question>,
    pub added_at: DateTime<Utc>,
    pub tags: Vec<String>,
  }

  impl Into<Bson> for Question { 
    fn into(self) -> Bson {
        Bson::Document(doc! {
            "question": self.question,
            "correct_answer": self.correct_answer,
            "incorrect_answers": self.incorrect_answers
        })
    }
  }


#[tokio::main]
async fn main() -> Result<()> {
    let db = DB::init().await?;

    let quiz = warp::path("quiz");

    let quiz_routes = quiz
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(handler::create_quiz_handler)
        .or(quiz
            .and(warp::get())
            .and(warp::path::param())
            .and(with_db(db.clone()))
            .and_then(handler::fetch_quiz_handler))
        .or(quiz
            .and(warp::put())
            .and(warp::path::param())
            .and(warp::body::json())
            .and(with_db(db.clone()))
            .and_then(handler::edit_quiz_handler))
        .or(quiz
            .and(warp::delete())
            .and(warp::path::param())
            .and(with_db(db.clone()))
            .and_then(handler::delete_quiz_handler))
        .or(quiz
            .and(warp::get())
            .and(with_db(db.clone()))
            .and_then(handler::quizzes_list_handler));

    let routes = quiz_routes.recover(error::handle_rejection);

    println!("Started on port 8080");
    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
    Ok(())
}

fn with_db(db: DB) -> impl Filter<Extract = (DB,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}
