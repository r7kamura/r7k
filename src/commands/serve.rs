use crate::result::Result;
use crate::server;

pub async fn serve() -> Result<()> {
    println!("Running a server on http://localhost:8080/");
    let _ = server::run()?.await;
    Ok(())
}
