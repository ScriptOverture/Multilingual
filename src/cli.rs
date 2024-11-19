use clap::Parser;

#[derive(Parser)]
pub struct Opts {
    /// 入口目录
    #[arg(long, value_name = "entry_path")]
    pub entry_path: String,
}
