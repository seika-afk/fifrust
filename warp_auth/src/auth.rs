use std::convert::Infallible;
use warp::{Filter, Rejection};

#[derive(Debug, Clone)]
pub enum Role {
    User,
    Admin,
}
pub fn with_auth(role: Role) -> impl Filter<Extract = (Role,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || role.clone())
}
