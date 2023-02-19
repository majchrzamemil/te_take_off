use crate::errors::Error;
use rocket::serde::Serialize;
use sqlx::PgPool;

#[derive(Clone, Serialize)]
#[serde(crate = "rocket::serde")]
#[derive(sqlx::Type)]
#[sqlx(type_name = "opinion_type")]
#[sqlx(rename_all = "lowercase")]
pub enum OpinionType {
    Drunk,
    Late,
    Abusive,
}

#[derive(Clone, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Opinion {
    pub nr_tel: i32,
    pub opinion_category: OpinionType,
    pub custom_opinion: Option<String>,
}

pub struct Repository(PgPool);

impl Repository {
    pub fn new(pool: &PgPool) -> Self {
        Self(pool.clone())
    }

    pub async fn create_opinion(&self, opinion: Opinion) -> Result<(), Error> {
        sqlx::query!(
            "INSERT INTO te_take_off.opinions (nr_tel, opinion_category, custom_opinion) VALUES ($1, $2, $3)",
            opinion.nr_tel,
            opinion.opinion_category as OpinionType,
            opinion.custom_opinion
        )
        .execute(&self.0)
        .await.map(|_| ())?;

        Ok(())
    }

    pub async fn list_opinions(&self, nr_tel: i32) -> Result<Vec<Opinion>, Error> {
        let opinions = sqlx::query_as!(
            Opinion,
            r#"SELECT nr_tel, opinion_category as "opinion_category:OpinionType", custom_opinion FROM te_take_off.opinions where opinions.nr_tel = $1"#,
            nr_tel
        )
        .fetch_all(&self.0)
        .await?;
        Ok(opinions)
    }
}
