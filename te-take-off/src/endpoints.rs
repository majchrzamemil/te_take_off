use crate::repository::Opinion as RepoOpinion;
use crate::repository::OpinionType as RepoOpinionType;
use crate::repository::Repository;
use rocket::serde::json::Json;
use rocket::serde::Deserialize;
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

impl From<Opinion> for RepoOpinion {
    fn from(opinion: Opinion) -> Self {
        Self {
            nr_tel: opinion.nr_tel,
            opinion_category: opinion.opinion_type.into(),
            custom_opinion: opinion.custom_opinion,
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde", rename_all = "lowercase")]
pub enum OpinionType {
    #[serde(rename = "drunk")]
    Drunk,
    Late,
    Abusive,
}

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Opinion {
    pub nr_tel: i32,
    pub opinion_type: OpinionType,
    pub custom_opinion: String,
}

#[get("/")]
pub async fn index() -> &'static str {
    "Hello world"
}

#[rocket::post("/add_opinion", format = "json", data = "<req>")]
pub async fn add_opinion(repo: &State<Repository>, req: Json<Opinion>) -> &'static str {
    repo.create_opinion(req.0.into()).await.unwrap(); //TODO: error handling not unwrap
    "ok"
}
