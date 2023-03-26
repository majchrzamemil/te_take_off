use crate::errors::Error;
use crate::repository::Opinion as RepoOpinion;
use crate::repository::OpinionType as RepoOpinionType;
use crate::repository::Repository;
use crate::repository::User;
use crate::session::Session;
use itertools::Either;
use rocket::form::Form;
use rocket::form::FromForm;
use rocket::http::ContentType;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::State;
use rocket::{get, post};

impl From<OpinionType> for RepoOpinionType {
    fn from(opinion_type: OpinionType) -> Self {
        match opinion_type {
            OpinionType::Drunk => RepoOpinionType::Drunk,
            OpinionType::Late => RepoOpinionType::Late,
            OpinionType::Abusive => RepoOpinionType::Abusive,
        }
    }
}

impl From<RepoOpinionType> for OpinionType {
    fn from(opinion_type: RepoOpinionType) -> Self {
        match opinion_type {
            RepoOpinionType::Drunk => OpinionType::Drunk,
            RepoOpinionType::Late => OpinionType::Late,
            RepoOpinionType::Abusive => OpinionType::Abusive,
        }
    }
}

impl From<Opinion> for RepoOpinion {
    fn from(opinion: Opinion) -> Self {
        Self {
            nr_tel: opinion.nr_tel,
            opinion_category: opinion.opinion_type.into(),
            custom_opinion: Some(opinion.custom_opinion),
        }
    }
}

impl From<&RepoOpinion> for Opinion {
    fn from(opinion: &RepoOpinion) -> Self {
        Self {
            nr_tel: opinion.nr_tel,
            opinion_type: opinion.opinion_category.clone().into(),
            custom_opinion: opinion.custom_opinion.clone().unwrap_or_default(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde", rename_all = "lowercase")]
pub enum OpinionType {
    Drunk,
    Late,
    Abusive,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Opinion {
    pub nr_tel: i32,
    pub opinion_type: OpinionType,
    pub custom_opinion: String,
}
macro_rules! html {
    ($html:expr) => {
        (ContentType::HTML, $html)
    };
}
#[get("/")]
pub fn index(session: Session<'_>) -> (ContentType, String) {
    if let Some(user) = session.user() {
        html!(format!("Welcome back, {}", user.nr_tel))
    } else {
        html!(
            r#"Hello, world! If you want to login, click here: <a href="/username-form">Login</a><br>
    If you want to register, click here: <a href="/registration-form">Register</a>
    "#.into()
        )
    }
}

#[get("/registration-form")]
pub fn registration_form() -> (ContentType, &'static str) {
    html!(
        r#"
    <form method="POST" action="/register">
        <label for="nr_tel">Numer telefonu/label><input type="text" name="nr_tel" />
        <label for="password">Haslo</label><input type="password" name="password" />
        <label for="email">email</label><input type="text" name="email" />
        <input type="submit" value="Register" />
    </form>
    "#
    )
}

#[get("/username-form")]
pub fn username_form() -> (ContentType, &'static str) {
    html!(
        r#"
    <form method="POST" action="/login-chooser">
        Numer telefonu: <input type="text" name="nr_tel" />
        <input type="submit" value="Next" />
    </form>
    "#
    )
}

#[post("/users/<username>/login", data = "<password>")]
pub async fn login_password(
    username: &str,
    password: Form<Password>,
    repository: &State<Repository>,
    mut session: Session<'_>,
) -> Result<String, Error> {
    // TODO: return some message on incorrect login not 404 xD
    let user = repository
        .password_login(username, &password.password)
        .await?;
    session.set_user(user);

    Ok(format!("Success login for user: {}.", username))
}

#[derive(FromForm)]
pub struct RegistrationForm {
    nr_tel: String,
    password: String,
    email: String,
}

#[derive(FromForm)]
pub struct Password {
    password: String,
}

#[post("/register", data = "<data>")]
pub async fn register(
    data: Form<RegistrationForm>,
    repository: &State<Repository>,
) -> Result<(ContentType, String), Error> {
    repository
        .create(User {
            nr_tel: data.nr_tel.clone(),
            password: data.password.clone(),
            email: data.email.clone(),
            verified: false,
        })
        .await?;
    Ok(html!(format!(
        r#"
    You successfully registered as <i>{}</i> using password <i>{}</i>
    "#,
        data.nr_tel, data.password
    )))
}

#[derive(FromForm)]
pub struct Username {
    nr_tel: String,
}

#[post("/login-chooser", data = "<data>")]
pub async fn login_chooser(data: Form<Username>) -> (ContentType, String) {
    html!(format!(
        r#"
             <form method="POST" action="/users/{}/login">
                 Password: <input type="password" name="password" />
                 <input type="submit" value="Next" />
             </form>
             "#,
        &data.nr_tel
    ))
}

#[rocket::post("/add-opinion", format = "json", data = "<req>")]
pub async fn add_opinion(
    repo: &State<Repository>,
    req: Json<Opinion>,
    session: Session<'_>,
) -> Either<Result<(), Error>, (ContentType, String)> {
    if session.user().is_some() {
        Either::Left(repo.create_opinion(req.0.into()).await)
    } else {
        Either::Right(html!(
                    r#"Hello, world! If you want to login, click here: <a href="/username-form">Login</a><br>
            If you want to register, click here: <a href="/registration-form">Register</a>
            "#.into()
    ))
    }
}

#[rocket::get("/get-opinions/<nr_tel>")]
pub async fn get_opinions(
    repo: &State<Repository>,
    nr_tel: i32,
    session: Session<'_>,
) -> Result<Either<Json<Vec<Opinion>>, (ContentType, String)>, Error> {
    if session.user().is_some() {
        let opinions: Vec<Opinion> = repo
            .list_opinions(nr_tel)
            .await?
            .iter()
            .map(|opinion| opinion.into())
            .collect();
        Ok(Either::Left(Json(opinions)))
    } else {
        Ok(Either::Right(html!(
                    r#"Hello, world! If you want to login, click here: <a href="/username-form">Login</a><br>
            If you want to register, click here: <a href="/registration-form">Register</a>
            "#.into()
        )))
    }
}
