// use chrono::prelude::*;
use db::{DB};
// use mongodb::bson::Bson;
use std::convert::Infallible;
use warp::{Filter, Rejection};

type Result<T> = std::result::Result<T, error::Error>;
type WebResult<T> = std::result::Result<T, Rejection>;

mod db;
mod error;
mod handler;
mod structs;

#[tokio::main]
async fn main() -> Result<()> {
    let db = DB::init().await?;
    let quiz = warp::path("quiz");
    let path = warp::path::param();
    let json = warp::body::json();
    let cors = warp::cors()
        .allow_origin("http://localhost:5000")
        .allow_headers(vec![
            "User-Agent",
            "Sec-Fetch-Mode",
            "Referer",
            "Origin",
            "Access-Control-Request-Method",
            "Access-Control-Request-Headers",
            "Content-Type",
        ])
        .allow_methods(vec!["POST", "GET"]);

    let create_quiz = quiz
        .and(warp::post())
        .and(json)
        .and(with_db(db.clone()))
        .and_then(handler::create_quiz_handler);

    let fetch_quiz = quiz
        .and(warp::get())
        .and(path)
        .and(with_db(db.clone()))
        .and_then(handler::fetch_quiz_handler);

    let edit_quiz = quiz
        .and(warp::put())
        .and(path)
        .and(json)
        .and(with_db(db.clone()))
        .and_then(handler::edit_quiz_handler);

    let delete_quiz = quiz
        .and(warp::delete())
        .and(path)
        .and(with_db(db.clone()))
        .and_then(handler::delete_quiz_handler);

    let search_quizzes = quiz
        .and(warp::get())
        .and(warp::path("search"))
        .and(path)
        .and(with_db(db.clone()))
        .and_then(handler::search_quizzes_handler);

    let fetch_quizzes = quiz
        .and(warp::get())
        .and(with_db(db.clone()))
        .and_then(handler::quizzes_list_handler);

    let fetch_recent_quizzes = quiz
        .and(warp::get())
        .and(warp::path("recent"))
        .and(with_db(db.clone()))
        .and_then(handler::fetch_recent_quizzes_handler);

    let delete_quizzes = quiz
        .and(warp::delete())
        .and(warp::path("delete"))
        .and(with_db(db.clone()))
        .and_then(handler::delete_quizzes_handler);

    let quiz_routes = create_quiz
        .or(fetch_quiz)
        .or(edit_quiz)
        .or(delete_quiz)
        .or(search_quizzes)
        .or(delete_quizzes)
        .or(fetch_recent_quizzes)
        .or(fetch_quizzes);

    let routes = quiz_routes.with(cors).recover(error::handle_rejection);

    println!("Started on port 8080");
    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
    Ok(())
}

fn with_db(db: DB) -> impl Filter<Extract = (DB,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}
