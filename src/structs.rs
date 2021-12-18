use crate::db::{doc};
use chrono::prelude::*;
use mongodb::bson::Bson;
use serde::{Deserialize, Serialize};
pub use mongodb::bson::document::Document;
pub use mongodb::bson::from_bson;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Question {
    pub question: String,
    pub correct_answer: String,
    pub incorrect_answers: Vec<String>,
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

impl From<Bson> for Question {
    fn from(bson: Bson) -> Self {
        bson.as_document().unwrap().clone().into()
    }
}

impl From<Document> for Question {
    fn from(doc: Document) -> Self {
        Question {
            question: doc.get_str("question").unwrap().to_string(),
            correct_answer: doc.get_str("correct_answer").unwrap().to_string(),
            incorrect_answers: doc.get_array("incorrect_answers").unwrap().to_vec().iter().map(|x| x.to_string()).collect(),
        }
    }

}

impl From<Document> for Quiz {
    fn from(doc: Document) -> Self {
        Quiz { 
            id: doc.get_object_id("_id").unwrap().to_hex(),
            title: doc.get_str("title").unwrap().to_string(),
            author: doc.get_str("author").unwrap().to_string(),
            questions: doc.get_array("questions").unwrap().to_vec().iter().map(|x| x.clone().into()).collect(),
            added_at: doc.get_datetime("added_at").unwrap().to_owned().to_chrono(),
            tags: doc.get_array("tags").unwrap().to_vec().iter().map(|x| x.to_string()).collect(),
        }
    }

}
