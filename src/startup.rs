use std::sync::Arc;

use dotenv::dotenv;

use crate::{app::App, scheduler::run_scheduler, server::run_server};

pub async fn run() {
    // std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").unwrap();

    let app = App::new(database_url).await;
    let app = Arc::new(app);

    run_scheduler(Arc::clone(&app));
    run_server(Arc::clone(&app)).await.unwrap();
}
