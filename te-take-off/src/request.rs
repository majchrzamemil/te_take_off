use itertools::Itertools;
use rocket::request::{FromRequest, Outcome, Request};
use std::fmt;

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub struct InvalidRequestError;

impl fmt::Display for InvalidRequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Invalid Request")
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Basic {
    pub nr_tel: String,
    pub password: String,
}

impl Basic {
    /// Helper function to take the text after 'Basic ' in a header and return a Basic object
    fn from_header_data_helper(header_data: &str) -> Option<Basic> {
        if let Ok(bytes) = base64::decode(header_data.as_bytes()) {
            if let Some((username_bytes, password_bytes)) =
                bytes.split(|c| *c == b':').collect_tuple()
            {
                if let (Ok(nr_tel), Ok(password)) = (
                    std::str::from_utf8(username_bytes),
                    std::str::from_utf8(password_bytes),
                ) {
                    return Some(Basic {
                        nr_tel: nr_tel.into(),
                        password: password.into(),
                    });
                }
            }
        }

        None
    }

    pub(super) fn from_header(header: &str) -> Outcome<Basic, InvalidRequestError> {
        if let Some(s) = header.strip_prefix("Basic ") {
            if let Some(result) = Basic::from_header_data_helper(s.trim_start()) {
                Outcome::Success(result)
            } else {
                Outcome::Failure((
                    rocket::http::Status::UnprocessableEntity,
                    InvalidRequestError,
                ))
            }
        } else {
            Outcome::Forward(())
        }
    }
}

#[rocket::async_trait]
impl<'a> FromRequest<'a> for Basic {
    type Error = InvalidRequestError;

    async fn from_request(req: &'a Request<'_>) -> Outcome<Self, Self::Error> {
        let mut auth_iter = req.headers().get("Authorization");
        let outcome = match auth_iter.next() {
            None => Outcome::Forward(()),
            Some(x) => Basic::from_header(x),
        };

        if auth_iter.next().is_some() {
            return Outcome::Failure((
                rocket::http::Status::UnprocessableEntity,
                InvalidRequestError,
            ));
        }

        outcome
    }
}
