use crate::{error::Error::*, handler::QuizRequest, Question, Quiz, Result};
use chrono::prelude::*;
use futures::StreamExt;
pub use mongodb::bson::{doc, document::Document, oid::ObjectId, Bson};
use mongodb::{options::ClientOptions, Client, Collection};

const DB_NAME: &str = "quizzbuzz";
const COLL: &str = "quizzes";

const ID: &str = "_id";
const TITLE: &str = "title";
const AUTHOR: &str = "author";
const QUESTIONS: &str = "questions";
const ADDED_AT: &str = "added_at";
const TAGS: &str = "tags";

#[derive(Clone, Debug)]
pub struct DB {
    pub client: Client,
}

impl DB {
    pub async fn init() -> Result<Self> {
        let mut client_options = ClientOptions::parse("mongodb://127.0.0.1:27017").await?;
        client_options.app_name = Some("quizzbuzz".to_string());

        Ok(Self {
            client: Client::with_options(client_options)?,
        })
    }

    pub async fn fetch_quiz(&self, id: &str) -> Result<Quiz> {
        let oid = ObjectId::with_string(id).map_err(|_| InvalidIDError(id.to_owned()))?;
        let query = doc! {
            "_id": oid,
        };
        let quiz_document = self
            .get_collection()
            .find_one(query, None)
            .await
            .map_err(MongoQueryError)?
            .expect("Could not fetch the given Quiz");
        let quiz = self
            .doc_to_quiz(&quiz_document)
            .expect("Could not fetch the given Quiz");
        Ok(quiz)
    }

    pub async fn create_quiz(&self, entry: &QuizRequest) -> Result<()> {
        let doc = self.quiz_to_doc(entry);

        self.get_collection()
            .insert_one(doc, None)
            .await
            .map_err(MongoQueryError)?;
        Ok(())
    }

    pub async fn edit_quiz(&self, id: &str, entry: &QuizRequest) -> Result<()> {
        let oid = ObjectId::with_string(id).map_err(|_| InvalidIDError(id.to_owned()))?;
        let query = doc! { "_id": oid,};
        let quiz_doc = self.quiz_to_doc(entry);

        self.get_collection()
            .update_one(query, quiz_doc, None)
            .await
            .map_err(MongoQueryError)?;
        Ok(())
    }

    pub async fn delete_quiz(&self, id: &str) -> Result<()> {
        let oid = ObjectId::with_string(id).map_err(|_| InvalidIDError(id.to_owned()))?;
        let filter = doc! {
            "_id": oid,
        };

        self.get_collection()
            .delete_one(filter, None)
            .await
            .map_err(MongoQueryError)?;
        Ok(())
    }

    pub async fn fetch_quizzes(&self) -> Result<Vec<Quiz>> {
        let mut cursor = self
            .get_collection()
            .find(None, None)
            .await
            .map_err(MongoQueryError)?;

        let mut result: Vec<Quiz> = Vec::new();
        while let Some(doc) = cursor.next().await {
            result.push(self.doc_to_quiz(&doc?)?);
        }
        Ok(result)
    }

    fn get_collection(&self) -> Collection {
        self.client.database(DB_NAME).collection(COLL)
    }

    fn doc_to_quiz(&self, doc: &Document) -> Result<Quiz> {
        let id = doc.get_object_id(ID)?;
        let title = doc.get_str(TITLE)?;
        let author = doc.get_str(AUTHOR)?;
        let questions = doc.get_array(QUESTIONS)?;
        let added_at = doc.get_datetime(ADDED_AT)?;
        let tags = doc.get_array(TAGS)?;

        let quiz = Quiz {
            id: id.to_hex(),
            title: title.to_owned(),
            author: author.to_owned(),
            questions: questions
                .iter()
                .map(|entry| {
                    entry
                        .as_document()
                        .and_then(|doc| self.doc_to_question(doc).ok())
                        .expect("Could not fetch Question object from database")
                })
                .collect(),
            added_at: *added_at,
            tags: tags
                .iter()
                .filter_map(|entry| match entry {
                    Bson::String(v) => Some(v.to_owned()),
                    _ => None,
                })
                .collect(),
        };
        Ok(quiz)
    }
    fn doc_to_question(&self, doc: &Document) -> Result<Question> {
        let question = doc.get_str("question")?;
        let correct_answer = doc.get_str("correct_answer")?;
        let incorrect_answers = doc.get_array("incorrect_answers")?;

        let question = Question {
            question: question.to_owned(),
            correct_answer: correct_answer.to_owned(),
            incorrect_answers: incorrect_answers
                .iter()
                .filter_map(|entry| match entry {
                    Bson::String(v) => Some(v.to_owned()),
                    _ => None,
                })
                .collect(),
        };
        Ok(question)
    }

    fn quiz_to_doc(&self, quiz: &QuizRequest) -> Document {
        doc! {
            TITLE: quiz.title.clone(),
            AUTHOR: quiz.author.clone(),
            QUESTIONS: quiz.questions.clone(),
            ADDED_AT: Utc::now(),
            TAGS: quiz.tags.clone(),

        }
    }
}
