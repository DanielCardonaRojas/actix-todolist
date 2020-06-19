use crate::db_connection::{PgPool, PgPooledConnection};
use crate::diesel::RunQueryDsl;
use crate::models::{AuthUser, RegisterUser, User, UserClaims};
use crate::schema::users;
use crate::schema::users::dsl::*;
use diesel::prelude::*;
use diesel::result::{Error, QueryResult};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

pub struct UserService<'a> {
    connection: &'a PgPooledConnection,
}

impl<'a> UserService<'a> {
    pub fn new(connection: &'a PgPooledConnection) -> UserService {
        Self {
            connection: connection,
        }
    }

    pub fn register(&self, registration: RegisterUser) -> QueryResult<User> {
        diesel::insert_into(users::table)
            .values(User::from(registration))
            .get_result(self.connection)
    }

    pub fn login(&self, authentication: AuthUser) -> QueryResult<User> {
        users::table
            .filter(email.eq(authentication.email))
            .get_result::<User>(self.connection)
    }

    pub fn create_token(claims: UserClaims) -> Result<String, jsonwebtoken::errors::Error> {
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret("secret".as_ref()),
        );
        token
    }
}
