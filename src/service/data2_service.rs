use std::{sync::Arc, time::Duration};

use chrono::Utc;

use crate::{db::Db, async_synchronized};

pub struct Data2Service {
    db: Arc<Db>,
}

impl Data2Service {
    pub fn new(db: Arc<Db>) -> Self {
        Self { db }
    }

    pub async fn generate(&self) {
        // async_synchronized!();
        println!("Data2Service::generate, {:?}", Utc::now());
        // tokio::time::sleep(Duration::from_secs(5)).await;
        let value = self.db.count_data1().await.unwrap_or_default() + 1;
        self.db.create_data2(value as i32).await;
    }
}
