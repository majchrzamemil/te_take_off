use rocket::http::Cookie;
use rocket::http::CookieJar;
use rocket::request::FromRequest;
use rocket::request::Outcome;
use rocket::request::Request;
use rocket::State;

use crate::repository::Repository;
use crate::repository::User;

pub struct Session<'r> {
    user: Option<User>,
    cookie_jar: &'r CookieJar<'r>,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Session<'r> {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let cookies = <&CookieJar>::from_request(req).await.unwrap();
        let repo: &State<Repository> = <&State<Repository>>::from_request(req).await.unwrap();

        let user = if let Some(nr_tel) = req.cookies().get_private("nr_tel") {
            repo.find_user(nr_tel.value()).await.ok()
        } else {
            None
        };

        Outcome::Success(Self {
            user,
            cookie_jar: cookies,
        })
    }
}

impl Session<'_> {
    pub fn user(&self) -> &Option<User> {
        &self.user
    }

    #[allow(unused)]
    pub fn logout(&mut self) {
        self.user = None;
        if let Some(cookie) = self.cookie_jar.get_private("nr_tel") {
            self.cookie_jar.remove_private(cookie);
        }
    }

    pub fn set_user(&mut self, user: User) {
        self.cookie_jar
            .add_private(Cookie::new("nr_tel", user.nr_tel.clone()));
        self.user = Some(user);
    }
}
