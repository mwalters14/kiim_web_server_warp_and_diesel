use crate::schema::questions;
use diesel::prelude::*;
use serde_derive::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Queryable, Deserialize, Serialize)]
pub struct QuestionDTO {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
    pub created_on: SystemTime,
}

#[derive(Debug, Clone, Insertable, Deserialize, Serialize)]
#[diesel(table_name = questions)]
pub struct CreateQuestionDTO {
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
}

impl CreateQuestionDTO {
    pub fn to_dto(&self) -> Self {
        CreateQuestionDTO {
            title: self.title.clone(),
            content: self.content.clone(),
            tags: self.tags.clone(),
        }
    }
}

#[derive(Debug, Clone, Insertable, Deserialize, Serialize, AsChangeset)]
#[diesel(table_name = questions)]
pub struct UpdateQuestionDTO {
    pub title: Option<String>,
    pub content: Option<String>,
    pub tags: Option<Vec<String>>,
}

impl UpdateQuestionDTO {
    pub fn to_dto(&self) -> Self {
        UpdateQuestionDTO {
            title: self.title.clone(),
            content: self.content.clone(),
            tags: self.tags.clone(),
        }
    }
}

/*
    PartialEq, Eq, Hash -> Are required if being used as a HashMap key
    These traits are already implemented on type String so we just need to derive them on our QuestionId type
*/
// #[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
// pub struct QuestionId(pub i32);

// impl QuestionId {
//     pub fn new(id: i32) -> Self {
//         QuestionId(id)
//     }
// }
