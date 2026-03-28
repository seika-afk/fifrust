use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;
use warp::{Filter, Rejection, Reply};

mod auth;
mod error;

use auth::{with_auth, Role};

type WebResult<T> = Result<T, Rejection>;
type Users = Arc<HashMap<String, User>>;

#[derive(Clone, Debug)]
pub struct User {
    pub uid: String,
    pub email: String,
    pub pw: String,
    pub role: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub pw: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

#[tokio::main]
async fn main() {
    let users = Arc::new(init_users());

    let login_route = warp::path!("login")
        .and(warp::post())
        .and(with_users(users.clone()))
        .and(warp::body::json())
        .and_then(login_handler);

    let user_route = warp::path!("user")
        .and(with_auth(Role::User))
        .and_then(user_handler);

    let admin_route = warp::path!("admin")
        .and(with_auth(Role::Admin))
        .and_then(admin_handler);

    let routes = login_route
        .or(user_route)
        .or(admin_route)
        .recover(error::handle_rejection);

    warp::serve(routes).run(([127, 0, 0, 1], 3000)).await;
}

fn with_users(users: Users) -> impl Filter<Extract = (Users,), Error = Infallible> + Clone {
    warp::any().map(move || users.clone())
}

async fn login_handler(users: Users, body: LoginRequest) -> WebResult<impl Reply> {
    let user = users
        .values()
        .find(|u| u.email == body.email && u.pw == body.pw);

    match user {
        Some(u) => {
            let token = format!("fake-jwt-token-for-{}", u.uid);
            let reply = LoginResponse { token };
            Ok(warp::reply::json(&reply))
        }
        None => Err(warp::reject::custom(error::Error::AuthError)),
    }
}

async fn user_handler(role: Role) -> WebResult<impl Reply> {
    Ok(warp::reply::json(&format!("Hello User: {:?}", role)))
}

async fn admin_handler(role: Role) -> WebResult<impl Reply> {
    Ok(warp::reply::json(&format!("Hello Admin: {:?}", role)))
}

fn init_users() -> HashMap<String, User> {
    let mut users = HashMap::new();

    users.insert(
        "1".into(),
        User {
            uid: "1".into(),
            email: "user@test.com".into(),
            pw: "1234".into(),
            role: "USER".into(),
        },
    );

    users.insert(
        "2".into(),
        User {
            uid: "2".into(),
            email: "admin@test.com".into(),
            pw: "admin".into(),
            role: "ADMIN".into(),
        },
    );

    users
}
