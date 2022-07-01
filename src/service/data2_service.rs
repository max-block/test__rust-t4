use std::sync::Arc;

use crate::db::Db;

pub struct Data2Service {
    db: Arc<Db>,
}

impl Data2Service {
    pub fn new(db: Arc<Db>) -> Self {
        Self { db }
    }

    pub async fn generate(&self) {
        println!("Data2Service::generate");
        let value = self.db.count_data1().await.unwrap_or_default() + 1;
        self.db.create_data2(value as i32).await;
    }
}
