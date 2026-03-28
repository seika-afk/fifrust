use warp::{http::StatusCode, Rejection, Reply};

#[derive(Debug)]
pub enum Error {
    AuthError,
}

impl warp::reject::Reject for Error {}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, std::convert::Infallible> {
    if err.find::<Error>().is_some() {
        Ok(warp::reply::with_status(
            "Unauthorized",
            StatusCode::UNAUTHORIZED,
        ))
    } else {
        Ok(warp::reply::with_status(
            "Internal Server Error",
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    }
}
