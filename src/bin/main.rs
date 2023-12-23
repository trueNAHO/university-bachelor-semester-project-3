use anyhow::Result;

use bachelor_semester_project_3_lib::server::Server;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().init();
    Server::new("127.0.0.1:0".into()).run().await
}
