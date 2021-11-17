use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Powers r7kamura.com.")]
pub enum Opt {
    #[structopt(about = "Build static files.")]
    Build {},

    #[structopt(about = "Run HTTP server.")]
    Serve {},
}
