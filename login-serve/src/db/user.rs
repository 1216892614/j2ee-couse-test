use futures::StreamExt;
use mongod::{bson::oid::ObjectId, Bson, Client, ClientBuilder, Mongo};
use mongod::{AsFilter, Comparator};

use crate::error::Error;
use crate::{error::Result, static_setting::CONNECT_WORD};

/// encapsulated user database client.
///
/// # Example
/// ```
/// # use super::*;
/// # use futures::stream::StreamExt;
/// # use mongod::{AsFilter, Comparator};
/// # #[tokio::test]
/// # async fn is_install_user_work() {
///     let user = UserCli::new().unwrap();
/// #     let mut user_filter = User::filter();
/// #     user_filter.username = Some(Comparator::Eq("username".to_owned()));
/// #
/// #     //Del all user with "username" as username
/// #     let _ = user.0.delete::<User, _>(Some(user_filter)).await.unwrap();
///
///     //Insert test data
///     user.insert_new_user(
///         "username".to_owned(),
///         "KICK BACK".to_owned(),
///         1665983027940.0,
///     )
///     .await
///     .unwrap();
///
///     //all user
///     let mut cursor = user.0.find::<User, _>(None).await.unwrap();
///     while let Some(res) = cursor.next().await {
///         if let Ok((_, user)) = res {
///             assert_eq!(user.username, "username");
///             assert_eq!(user.password, "KICK BACK");
///         }
///     }
/// # }
/// ```
pub struct UserCli(Client);

#[derive(Debug, Bson, Mongo)]
#[mongo(collection = "users", field, filter, update)]
struct User {
    username: String,
    password: String,
    registration_time: f64,
}

impl UserCli {
    pub fn new() -> Result<Self> {
        let cli = ClientBuilder::new()
            .uri(CONNECT_WORD)
            .database("user")
            .build()?;

        Ok(Self(cli))
    }

    /// Insert a user
    /// 
    /// # Error
    /// Insert same user will return Error::UserNameExists.
    /// 
    /// ```should_panic
    /// # use super::*;
    /// # use futures::stream::StreamExt;
    /// # use mongod::{AsFilter, Comparator};
    /// # #[tokio::test]
    /// # async fn is_install_user_work() {
    ///     let user = UserCli::new().unwrap();
    /// #     let mut user_filter = User::filter();
    /// #     user_filter.username = Some(Comparator::Eq("username".to_owned()));
    /// #
    /// #     //Del all user with "username" as username
    /// #     let _ = user.0.delete::<User, _>(Some(user_filter)).await.unwrap();
    ///
    ///     //Insert test data
    ///     user.insert_new_user(
    ///         "username".to_owned(),
    ///         "KICK BACK".to_owned(),
    ///         1665983027940.0,
    ///     )
    ///     .await
    ///     .unwrap();
    ///
    ///     //Insert same data
    ///     user.insert_new_user(
    ///         "username".to_owned(),
    ///         "KICK BACK".to_owned(),
    ///         1665983027940.0,
    ///     )
    ///     .await
    ///     .unwrap();//Panic here!!
    /// # }
    /// ```
    /// # Example
    /// ```
    /// # use super::*;
    /// # use futures::stream::StreamExt;
    /// # use mongod::{AsFilter, Comparator};
    /// # #[tokio::test]
    /// # async fn is_install_user_work() {
    ///     let user = UserCli::new().unwrap();
    /// #     let mut user_filter = User::filter();
    /// #     user_filter.username = Some(Comparator::Eq("username".to_owned()));
    /// #
    /// #     //Del all user with "username" as username
    /// #     let _ = user.0.delete::<User, _>(Some(user_filter)).await.unwrap();
    ///
    ///     //Insert test data
    ///     user.insert_new_user(
    ///         "username".to_owned(),
    ///         "Doki Doki".to_owned(),
    ///         2695383726941.0,
    ///     )
    ///     .await
    ///     .unwrap();
    ///
    ///     //all user
    ///     let mut cursor = user.0.find::<User, _>(None).await.unwrap();
    ///     while let Some(res) = cursor.next().await {
    ///         if let Ok((_, user)) = res {
    ///             assert_eq!(user.username, "username");
    ///             assert_eq!(user.password, "KICK BACK");
    ///         }
    ///     }
    /// # }
    /// ```
    pub async fn insert_new_user(
        &self,
        username: String,
        password: String,
        timestamp: f64,
    ) -> Result<ObjectId> {
        let mut user_filter = User::filter();
        user_filter.username = Some(Comparator::Eq(username.clone()));

        let mut cursor = self.0.find::<User, _>(Some(user_filter)).await.unwrap();
        while let Some(res) = cursor.next().await {
            if let Ok((id, user)) = res {
                Err(Error::UserNameExists(format!(
                    "oid: {}, user: {:?}",
                    id, user
                )))?
            }
        }

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
