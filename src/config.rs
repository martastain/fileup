use clap::Parser;
use dotenv::dotenv;
use lazy_static::lazy_static;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[clap(
    name = "fileup",
    about = "Simple file upload server",
    version = env!("CARGO_PKG_VERSION"),
)]
pub struct Config {
    #[clap(long, env = "FILEUP_PORT", default_value = "6547")]
    pub server_port: u16,

    #[clap(long, env = "FILEUP_DIR", default_value = "uploads")]
    pub target_dir: String,

    #[clap(long, env = "HTTP2AMCP_LOG_LEVEL", default_value = "info")]
    pub log_level: String,
}

lazy_static! {
    pub static ref CONFIG: Config = {
        dotenv().ok(); // Load .env file if it exists
        Config::parse()
    };
}
