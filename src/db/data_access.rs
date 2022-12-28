use crate::schema;
use crate::types::question::{CreateQuestionDTO, QuestionDTO, UpdateQuestionDTO};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};

type PooledPg = PooledConnection<ConnectionManager<PgConnection>>;

use handle_errors::AppError;
pub struct DBAccessManager {
    connection: PooledPg,
}

impl DBAccessManager {
    pub fn new(connection: PooledPg) -> DBAccessManager {
        DBAccessManager { connection }
    }
    
    pub fn get_question(&mut self) -> Result<Vec<QuestionDTO>, AppError> {
        use schema::questions;

        questions::dsl::questions
            .load::<QuestionDTO>(&mut self.connection)
            .map_err(|err| AppError::from_diesel_err(err, "Error retrieving questions."))
    }
     
    pub fn create_question(
        &mut self, 
        dto: CreateQuestionDTO
    ) -> Result<QuestionDTO, AppError> {
        use schema::questions;

        diesel::insert_into(questions::table) // insert into books table
            .values(&dto) // use values from CreateBookDTO
            .get_result(&mut self.connection) // execute query
            .map_err(|err| AppError::from_diesel_err(err, "while creating question."))
        // if error occurred map it to AppError
    }

    pub fn update_question(
        &mut self,
        id: i32,
        updated_question: UpdateQuestionDTO,
    ) -> Result<QuestionDTO, AppError> {
        use schema::questions;

        diesel::update(questions::dsl::questions)
            .filter(questions::dsl::id.eq(id))
            .set(updated_question)
            .get_result(&mut self.connection)
            .map_err(|err| AppError::from_diesel_err(err, "Error updating question."))
    }

    pub fn delete_question(
        &mut self,
        id: i32,
    ) -> Result<usize, AppError>{
        use schema::questions;

        diesel::delete(questions::dsl::questions)
            .filter(questions::dsl::id.eq(id))
            .execute(&mut self.connection)
            .map_err(|err| AppError::from_diesel_err(err, "Could not delete question."))
    }
}
