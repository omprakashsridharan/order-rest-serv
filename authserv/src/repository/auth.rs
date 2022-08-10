use crate::entity::user::{
    ActiveModel as UserModel, Column as UserColumn, Entity as UserEntity, Model,
};

use common::db::connection::{DatabaseConnection, DbErr};
use common::enums::ROLES;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter};
use tracing::{error, info};

#[derive(Clone)]
pub struct AuthRepository {
    pub db_pool: DatabaseConnection,
}

impl AuthRepository {
    pub fn new(db_pool: DatabaseConnection) -> Self {
        AuthRepository { db_pool }
    }

    pub async fn signup(
        &self,
        email: String,
        password: String,
        address: String,
        phone: String,
    ) -> Result<UserModel, DbErr> {
        let user_model = UserModel {
            email: Set(email),
            password: Set(password),
            address: Set(address),
            phone: Set(phone),
            role: Set(ROLES::USER.to_string()),
            ..Default::default()
        }
        .save(&self.db_pool)
        .await?;
        info!("User created successfully");

        Ok(user_model)
    }

    pub async fn signin(&self, email: String, password: String) -> Result<Model, DbErr> {
        let user_model_option = UserEntity::find()
            .filter(UserColumn::Email.contains(&email))
            .filter(UserColumn::Password.eq(&*password))
            .one(&self.db_pool)
            .await?;
        if let Some(user_model) = user_model_option {
            Ok(user_model)
        } else {
            error!("Cannot find User");
            return Err(DbErr::Custom("Cannot fund User".to_string()));
        }
    }
}
