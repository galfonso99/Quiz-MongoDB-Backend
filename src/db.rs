use crate::{error::Error::*, handler::QuizRequest, Result};
use crate::structs::{Question, Quiz};
use chrono::prelude::*;
use futures::StreamExt;
pub use mongodb::bson::{doc, document::Document, oid::ObjectId, Bson};
use mongodb::{Client, Collection, };
use mongodb::options::{ClientOptions, FindOptions};

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
        let quiz: Quiz = quiz_document.into();
        Ok(quiz)
    }

    //Request made to test the query characteristics of the database
    pub async fn search_quizzes(&self, title: &str) -> Result<Vec<Quiz>> {
        let query = doc! {
            "title": {"$regex" : format!("{}", title), "$options": "i"},
        };
        let mut cursor = self
            .get_collection()
            .find(query, None)
            .await
            .map_err(MongoQueryError)?;

        let mut result: Vec<Quiz> = Vec::new();
        while let Some(doc) = cursor.next().await {
            let quiz: Quiz = doc.map_err(MongoQueryError)?.into();
            result.push(quiz);
        }
        Ok(result)
    }

    pub async fn create_quiz(&self, entry: QuizRequest) -> Result<String> {
        let quiz_doc: Document = entry.into();

        let result = self
            .get_collection()
            .insert_one(quiz_doc, None)
            .await
            .map_err(MongoQueryError)?;
        let inserted_id = result.inserted_id.to_string();
        Ok(inserted_id)
    }

    pub async fn edit_quiz(&self, id: &str, entry: QuizRequest) -> Result<()> {
        let oid = ObjectId::with_string(id).map_err(|_| InvalidIDError(id.to_owned()))?;
        let query = doc! { "_id": oid,};
        let quiz_doc: Document = entry.into();

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
            let quiz: Quiz = doc.map_err(MongoQueryError)?.into();
            result.push(quiz);
        }
        Ok(result)
    }

    pub async fn fetch_recent_quizzes(&self) -> Result<Vec<Quiz>> {
        let options = FindOptions::builder()
            .limit(8)
            .sort(doc! {
                "added_at": -1,
            })
            .build();
        let mut cursor = self
            .get_collection()
            .find(None, options)
            .await
            .map_err(MongoQueryError)?;

        let mut result: Vec<Quiz> = Vec::new();
        while let Some(doc) = cursor.next().await {
            let quiz: Quiz = doc.map_err(MongoQueryError)?.into();
            result.push(quiz);
        }
        Ok(result)
    }

    pub async fn delete_quizzes(&self) -> Result<()> {
        let delete_result = self
            .get_collection()
            .delete_many(
                doc! {
                   "tags": ["funner"]
                },
                None
            ).await?;

        println!("Deleted {} documents", delete_result.deleted_count);
        Ok(())
    }

    fn get_collection(&self) -> Collection {
        self.client.database(DB_NAME).collection(COLL)
    }

}
