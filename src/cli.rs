use clap::Parser;

#[derive(Parser)]
pub struct Opts {
    /// 入口目录
    #[arg(long, value_name = "entry_path")]
    pub entry_path: String,

    /// 跳出目录 exclude-dirs = "path1, path2"
    #[arg(long, value_name = "exclude_dirs")]
    pub exclude_dirs: Option<Vec<String>>,
}
