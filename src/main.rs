mod client;
mod commands;
mod error;
mod handlers;
mod models;
mod opt;
mod parser;
mod path_finder;
mod result;
mod server;

use crate::opt::Opt;
use result::Result;
use structopt::StructOpt;

#[actix_web::main]
async fn main() -> Result<()> {
    match Opt::from_args() {
        Opt::Serve {} => commands::serve().await,
        Opt::Build {} => commands::build().await,
    }
}
