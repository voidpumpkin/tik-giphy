use crate::{schema::users, types::DbConnection};
use diesel::{
    insert_into, ExpressionMethods, Insertable, QueryDsl, QueryResult, Queryable, RunQueryDsl,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
}

impl User {
    pub fn insert(
        user: &UserInsertTO,
        connection: &DbConnection,
    ) -> Result<(), diesel::result::Error> {
        use crate::schema::users::dsl::*;

        insert_into(users).values(user).execute(connection)?;
        Ok(())
    }

    pub fn get_by_email(query_email: &str, connection: &DbConnection) -> QueryResult<User> {
        use crate::schema::users::dsl::*;

        let user = users
            .filter(email.eq(query_email))
            .first::<User>(connection)?;
        Ok(user)
    }

    pub fn get_all(connection: &DbConnection) -> QueryResult<Vec<User>> {
        use crate::schema::users::dsl::*;

        let user = users.load::<User>(connection)?;
        Ok(user)
    }
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "users"]
pub struct UserInsertTO {
    pub username: String,
    pub email: String,
    pub password: String,
}
