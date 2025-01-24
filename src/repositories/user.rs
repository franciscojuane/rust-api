use std::sync::Arc;
use crate::entities::prelude::User;
use crate::entities::user;
use crate::errors::errors::CustomError;
use log::{debug, error, info};
use sea_orm::{ActiveValue, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, IntoActiveModel, QueryFilter, QueryOrder, QuerySelect};
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use crate::auth;

pub struct UserRepository {
    database_connection: Arc<RwLock<DatabaseConnection>>
}

impl UserRepository {
    pub fn new(database_connection: Arc<RwLock<DatabaseConnection>>) -> Self{
        Self {
            database_connection
        }
    }
}

impl UserRepository{
    pub async fn create(&mut self, item: &user::Model) -> Result<i32, CustomError> {
        let user_active_model = user::ActiveModel {
            id: ActiveValue::Set(item.id),
            first_name: ActiveValue::Set(item.first_name.clone()),
            last_name: ActiveValue::Set(item.last_name.clone()),
            password: ActiveValue::Set(auth::auth::generate_hash(&item.password.clone())),
            telephone: ActiveValue::Set(item.telephone.clone()),
            email: ActiveValue::Set(item.email.clone()),
        };
        let db = self.database_connection.write().await;
        let result = User::insert(user_active_model).exec(&*db).await;
        match result {
            Ok(insert_result) => {
                info!("User entity created with id {}", &insert_result.last_insert_id);
                Ok(insert_result.last_insert_id)
            }
            Err(_) => {
                error!("Error when creating user: {:?} ", item);
                Err(CustomError::CreationError)
            }
        }
    }


    pub async fn read(&self, id: u64) -> Result<user::Model, CustomError>{
        let db = self.database_connection.read().await;
        let result = User::find_by_id(id as i32).one(&*db).await;
        match result {
            Ok(item) => {
                match item{
                    None => {Err(CustomError::ElementNotFound)}
                    Some(user) => {
                        debug!("Read user with id {}", id);
                        Ok(user)

                    }
                }
            }
            Err(error) => {
                match error {
                    DbErr::RecordNotFound(_) => {Err(CustomError::ElementNotFound)},
                    _ => {
                        error!("Error reading user with id {} and error: {} ", id, error.to_string());
                        Err(CustomError::DatabaseError)
                    }
                }
            }
        }
    }

    pub async fn find_by_email(&self, email: &str) -> Result<user::Model, CustomError>{
        let db = self.database_connection.read().await;
        let result = User::find()
            .filter(user::Column::Email.eq(email))
            .one(&*db).await;
        match result {
            Ok(item) => {
                match item{
                    None => {Err(CustomError::ElementNotFound)}
                    Some(user) => {
                        debug!("Read user with id {}", email);
                        Ok(user)

                    }
                }
            }
            Err(error) => {
                match error {
                    DbErr::RecordNotFound(_) => {Err(CustomError::ElementNotFound)},
                    _ => {
                        error!("Error reading user with email {} and error: {} ", email, error.to_string());
                        Err(CustomError::DatabaseError)
                    }
                }
            }
        }
    }

    pub async fn update(&mut self, id: i32, user_update_dto: UserUpdateDTO) -> Result<user::Model, CustomError> {
        let result = self.read(id as u64).await;
        let logging_dto = user_update_dto.clone();
        match result {
            Ok(value) => {
                let mut active_model = value.into_active_model();
                if let Some(first_name) = user_update_dto.first_name {
                    active_model.first_name = Set(first_name);
                }
                if let Some(last_name) = user_update_dto.last_name {
                    active_model.last_name = Set(last_name);
                }
                if let Some(password) = user_update_dto.password {
                    active_model.password = Set(auth::auth::generate_hash(&password));
                }
                if let Some(telephone) = user_update_dto.telephone {
                    active_model.telephone = Set(telephone);
                }
                if let Some(email) = user_update_dto.email {
                    active_model.email = Set(email);
                }

                let db = self.database_connection.write().await;
                let result = User::update(active_model).exec(&*db).await;
                match result {
                    Ok(model) => {
                        info!("User entity updated with id {}", id);
                        Ok(model)
                    }
                    Err(_) => {
                        error!("Error updating user entity with id {} with DTO {:?}", id, logging_dto);
                        Err(CustomError::UpdateError)
                    }
                }
            },
            Err(error) => {
                info!("Error when updating user entity with id {} with values {:?} and error {}", id, logging_dto, error);
                Err(error)
            }
        }
    }


    pub async fn delete(&mut self,  id: u64) -> Result<(), CustomError> {
        let db = self.database_connection.write().await;
        let result = User::delete_by_id(id as i32).exec(&*db).await;
        match result {
            Ok(delete_result) => {
                if delete_result.rows_affected == 0 {
                    info!("User entity with id {} not found for update.", id);
                    Result::Err(CustomError::ElementNotFound)
                } else {
                    info!("User entity with id {} updated.", id);
                    Result::Ok(())
                }
            }
            Err(_) => {
                info!("User entity with id {} couldn't be deleted", id);
                Err(CustomError::DeletionError)
            }
        }
    }


    pub async fn list(&self, page: u64, page_size:u64) -> Result<Vec<user::Model>, CustomError> {
        let db = self.database_connection.read().await;
        let limit = page_size;
        let offset = limit * (page - 1);
        let results = User::find()
            .limit(limit)
            .offset(offset)
            .order_by_asc(user::Column::Id)
            .all(&*db)
            .await;

        match results {
            Ok(users) => {Ok(users)},
            Err(_) => {
                error!("Couldn't list users");
                Err(CustomError::ReadError)
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserUpdateDTO {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub password: Option<String>,
    pub telephone: Option<String>,
    pub email: Option<String>,
}




