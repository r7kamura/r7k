mod client;
mod commands;
mod handlers;
mod models;
mod opt;
mod parser;
mod path_finder;

use crate::opt::Opt;
use structopt::StructOpt;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    match Opt::from_args() {
        Opt::Serve {} => commands::run()?.await,
        Opt::Build {} => commands::build().await,
    }
}
