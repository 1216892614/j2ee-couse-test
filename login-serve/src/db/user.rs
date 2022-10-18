use mongod::{bson::oid::ObjectId, Bson, Client, ClientBuilder, Collection, Mongo};

use crate::{error::Result, static_setting::CONNECT_WORD};

pub struct UserCli(Client);

#[derive(Debug, Bson, Mongo)]
#[mongo(collection = "users", field, filter, update)]
struct User {
    username: String,
    password: String,
    registration_time: f64,
}

impl UserCli {
    fn new() -> Result<Self> {
        let cli = ClientBuilder::new().uri(CONNECT_WORD).database("user").build()?;

        Ok(Self(cli))
    }

    async fn insert_new_user(
        &self,
        username: String,
        password: String,
        timestamp: f64,
    ) -> Result<ObjectId> {
        Ok(self
            .0
            .insert_one(User {
                username,
                password,
                registration_time: timestamp,
            })
            .await?)
    }
}