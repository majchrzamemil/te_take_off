use rocket::response::Responder;
use thiserror::Error;
#[derive(Debug, Error)]
pub enum Error {
    #[error("Database error:{0}")]
    Sqlx(#[from] sqlx::Error),
}

impl<'r> Responder<'r, 'r> for Error {
    fn respond_to(
        self,
        _: &'r rocket::Request<'_>,
    ) -> std::result::Result<rocket::Response<'r>, rocket::http::Status> {
        match self {
            Self::Sqlx(err) => match err {
                sqlx::Error::RowNotFound => {
                    eprintln!("Resource not found: {}", err);
                    Err(rocket::http::Status::NotFound)
                }
                _ => Err(rocket::http::Status::InternalServerError),
            },
        }
    }
}
