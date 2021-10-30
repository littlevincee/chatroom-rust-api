use chrono::{DateTime, Utc};
// use diesel::sql_types::Timestamp;
// use async_graphql::guard::Guard;
use std::str::FromStr;
use async_graphql::*;
use async_graphql_actix_web::*;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

use common_utils::Role as AuthRole;

use crate::get_conn_from_ctx;
use crate::persistence::user::{NewUserEntity, UserEntity};
use crate::persistence::repository;
use crate::utils::{hash_password, verify_password};

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

pub struct Query;

#[Object]
impl Query {
    async fn get_users(&self, ctx: &Context<'_>) -> Vec<User> {
        repository::get_all(&get_conn_from_ctx(ctx))
            .expect("Can't get planets")
            .iter()
            .map(User::from)
            .collect()
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    async fn create_user(&self, ctx: &Context<'_>, user: UserInput) -> User {
        let new_user = NewUserEntity {
            username: user.username,
            first_name: user.first_name,
            last_name: user.last_name,
            is_active: user.is_active,
            is_super: user.is_super,
            //created_at: user.created_at,
            //updated_at: user.updated_at
        };

        let created_user_entity =
            repository::create(new_user, &get_conn_from_ctx(ctx)).expect("Can't create user");

        User::from(&created_user_entity)
    }
}


#[derive(SimpleObject)]
struct User {
    username: String,
    first_name: String,
    last_name: String,
    is_active: bool,
    is_super: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(InputObject)]
struct UserInput {
    username: String,
    first_name: String,
    last_name: String,
    is_active: bool,
    is_super: bool,
    // created_at: DateTime<Utc>,
    // updated_at: DateTime<Utc>
}

impl From<&UserEntity> for User {
    fn from(entity: &UserEntity) -> Self {
        User {
            username: entity.username.clone(),
            first_name: entity.first_name.clone(),
            last_name: entity.last_name.clone(),
            is_active: entity.is_active.clone(),
            is_super: entity.is_super.clone(),
            created_at: entity.created_at.clone(),
            updated_at: entity.updated_at.clone(),
        }
    }
}
