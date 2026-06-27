use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "llm-client")]
#[command(about = "A minimalistic CLI agent for any OpenAI-compatible server", long_about = None)]
pub struct Args {
    /// Server port
    #[arg(short='p', long="port", default_value_t = 8080)]
    port: u16,

    #[arg(short='l', long="log-file", default_value_os_t = String::from("log.log"))]
    pub log_file: String,

    #[arg(long="log-size", default_value_t = 104_857_600)]
    pub log_size: u64,
}
impl Args {
    #[inline]
    pub fn address(&self) -> String {
        format!("http://127.0.0.1:{}/v1/chat/completions", self.port)
    }
}