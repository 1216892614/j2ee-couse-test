use crate::db::user;
use crate::static_setting::SERVER_SIDE_ERROR;

pub(super) async fn login_up(username: String, password: String, timestamp: f64) -> String {
    match user::UserCli::new() {
        Ok(user) => {
            if let Err(_) = user.insert_new_user(username, password, timestamp).await {
                log::debug!("The username already exists!");
                "USERNAME_ALREADY_EXISTS".to_owned()
            } else {
                "SCSS".to_owned()
            }
        }
        Err(e) => {
            log::error!("{}", e);
            SERVER_SIDE_ERROR.to_owned()
        }
    }
}
