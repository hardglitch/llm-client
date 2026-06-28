use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "llm-client")]
#[command(about = "A minimalistic CLI agent for any OpenAI-compatible server", long_about = None)]
pub struct Args {
    #[arg(short='p', long="port", default_value_t = 8080)]
    pub port: u16,

    #[arg(long="show-stat", default_value_t = false)]
    pub show_stat: bool,

    #[arg(short='l', long="log-file", default_value_os_t = String::from("log.log"))]
    pub log_file: String,

    #[arg(long="log-size", default_value_t = 104_857_600)]
    pub log_size: u64,
}
