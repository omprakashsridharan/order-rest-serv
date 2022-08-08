use crate::entity::user::{ActiveModel as UserModel, Column as UserColumn, Entity as UserEntity};

use common::db::connection::{DatabaseConnection, DbErr};
use common::enums::ROLES;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter};

use crate::utils::jwt::{generate_jwt, TokenData};
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
    ) -> Result<(), DbErr> {
        UserModel {
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
        Ok(())
    }

    pub async fn signin(&self, email: String, password: String) -> Result<String, DbErr> {
        let user_model_option = UserEntity::find()
            .filter(UserColumn::Email.contains(&email))
            .filter(UserColumn::Password.contains(&password))
            .one(&self.db_pool)
            .await?;
        if let Some(user_model) = user_model_option {
            let email = user_model.email;
            let token = generate_jwt(TokenData {
                email: email.clone(),
                user_id: user_model.id.to_string(),
                role: user_model.role,
                token: None,
            });
            info!("{} Token generated successfully", email.clone());
            Ok(token)
        } else {
            error!("Cannot find User");
            return Err(DbErr::Custom("Cannot fund User".to_string()));
        }
    }
}
