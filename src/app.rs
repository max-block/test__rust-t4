use std::sync::Arc;

use crate::{
    db::Db,
    service::{Data1Service, Data2Service},
};

pub struct App {
    pub db: Arc<Db>,
    pub data1_service: Arc<Data1Service>,
    pub data2_service: Arc<Data2Service>,
}

impl App {
    pub async fn new(url: String) -> Self {
        let db = Db::new(url).await;
        let db = Arc::new(db);

        let data1_service = Data1Service::new(Arc::clone(&db));
        let data1_service = Arc::new(data1_service);

        let data2_service = Data2Service::new(Arc::clone(&db));
        let data2_service = Arc::new(data2_service);

        Self {
            db,
            data1_service,
            data2_service,
        }
    }
}
