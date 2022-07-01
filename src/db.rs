use serde::{Deserialize, Serialize};
use sqlx::PgPool;

pub struct Db {
    pool: PgPool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data1 {
    pub id: i32,
    pub value: i32,
}

impl Db {
    pub async fn new(url: String) -> Self {
        let pool = PgPool::connect(&url).await.unwrap();
        Self { pool }
    }

    pub async fn create_data1(&self, value: i32) -> i32 {
        sqlx::query_scalar!("insert into data1 (value) values ($1) returning id", value)
            .fetch_one(&self.pool)
            .await
            .unwrap()
    }

    pub async fn find_all_data1(&self) -> Vec<Data1> {
        sqlx::query_as!(Data1, "select * from data1 order by id")
            .fetch_all(&self.pool)
            .await
            .unwrap()
    }


    pub async fn create_data2(&self, value: i32) -> i32 {
        sqlx::query_scalar!("insert into data2 (value) values ($1) returning id", value)
            .fetch_one(&self.pool)
            .await
            .unwrap()
    }

    pub async fn find_all_data2(&self) -> Vec<Data1> {
        sqlx::query_as!(Data1, "select * from data2 order by id")
            .fetch_all(&self.pool)
            .await
            .unwrap()
    }

    pub async fn count_data1(&self) -> Option<i64> {
        sqlx::query_scalar!("select count(*) from data1")
            .fetch_one(&self.pool)
            .await
            .unwrap()
    }

    pub async fn count_data2(&self) -> Option<i64> {
        sqlx::query_scalar!("select count(*) from data2")
            .fetch_one(&self.pool)
            .await
            .unwrap()
    }
}
