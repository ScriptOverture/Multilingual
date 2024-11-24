use clap::Parser;

#[derive(Parser)]
pub struct Opts {
    /// 入口目录
    #[arg(long, value_name = "entry_path")]
    pub entry_path: String,

    /// 远端匹配文件路径
    #[arg(long, value_name = "origin_language_path")]
    pub origin_language_path: String,
}
