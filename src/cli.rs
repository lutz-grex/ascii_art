use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(name = "llm-cli")]
pub struct Cli {
    #[arg(long, short = 'i')]
    pub image: String,

    #[arg(long, short = 'o')]
    pub output: String,

    #[arg(long, short = 't')]
    pub target_width: Option<f32>,

    #[arg(long, short = 'p', default_value = "false")]
    pub preview: bool,
}
