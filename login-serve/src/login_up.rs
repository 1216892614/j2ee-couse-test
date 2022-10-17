pub(super) fn login_up(username: String, password: String, timestamp: f64) -> String {
    format!("\"success, {}, {}, {}\"", username, password, timestamp)
}
