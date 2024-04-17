use blogear_server::config::Config;
use clap::Parser;

mod cli;

#[tokio::main]
async fn main() {
    let opts = cli::Opts::parse();
    let config = Config::new(&opts.config).expect("Config should be correct");

    blogear_server::logger::init(&config.log).expect("Logger should be configured");
    blogear_server::run(config).await.expect("Blogear running error")
}
