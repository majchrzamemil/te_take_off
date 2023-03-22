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

pub struct User {
    pub nr_tel: String,
    pub password: String,
    pub email: String,
    pub verified: bool,
}

pub struct Repository(PgPool);

impl Repository {
    pub fn new(pool: &PgPool) -> Self {
        Self(pool.clone())
    }

    pub async fn create(&self, user: User) -> Result<(), Error> {
        sqlx::query!(
            "INSERT INTO te_take_off.users (nr_tel, password, email) VALUES ($1, crypt($2, gen_salt('bf')), $3)",
            user.nr_tel,
            user.password,
            user.email
        )
        .execute(&self.0)
        .await?;
        Ok(())
    }

    pub async fn password_login(&self, email: &str, password: &str) -> Result<User, Error> {
        Ok(sqlx::query_as!(
            User,
            "SELECT nr_tel, password, email, verified FROM te_take_off.users WHERE nr_tel = $1 AND password = crypt($2, password)",
            email,
            password
        )
        .fetch_one(&self.0).await?)
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
    pub async fn find_user(&self, nr_tel: &str) -> Result<User, Error> {
        Ok(sqlx::query_as!(
            User,
            "SELECT nr_tel, password, email, verified FROM te_take_off.users WHERE nr_tel = $1",
            nr_tel,
        )
        .fetch_one(&self.0)
        .await?)
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
