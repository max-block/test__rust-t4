use std::sync::Arc;

use crate::db::Db;

pub struct Data1Service {
    db: Arc<Db>,
}

impl Data1Service {
    pub fn new(db: Arc<Db>) -> Self {
        Self { db }
    }

    pub async fn generate(&self) {
        println!("Data1Service::generate");
        let value = self.db.count_data2().await.unwrap_or_default() + 1;
        self.db.create_data1(value as i32).await;
    }
}
