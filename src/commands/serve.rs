use crate::result::Result;
use crate::server;

pub async fn serve() -> Result<()> {
    let _ = server::run()?.await;
    Ok(())
}
