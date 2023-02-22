use crate::errors::Error;
use crate::repository::Opinion as RepoOpinion;
use crate::repository::OpinionType as RepoOpinionType;
use crate::repository::Repository;
use rocket::get;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::State;

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

#[get("/")]
pub async fn index() -> &'static str {
    "Hello world"
}

#[rocket::post("/add-opinion", format = "json", data = "<req>")]
pub async fn add_opinion(repo: &State<Repository>, req: Json<Opinion>) -> Result<(), Error> {
    repo.create_opinion(req.0.into()).await?;
    Ok(())
}

#[rocket::get("/get-opinions/<nr_tel>")]
pub async fn get_opinions(
    repo: &State<Repository>,
    nr_tel: i32,
) -> Result<Json<Vec<Opinion>>, Error> {
    let opinions: Vec<Opinion> = repo
        .list_opinions(nr_tel)
        .await?
        .iter()
        .map(|opinion| opinion.into())
        .collect();
    Ok(Json(opinions))
}
