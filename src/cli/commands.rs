use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    author = "Eric Zorn", 
    version = "1.0.0", 
    about, 
    long_about = None
)]
pub struct Args {
    /// CSV file path for LinkedIn Sales Navigator Export
    /// coming from Eva Boot
    #[arg(short, long)]
    pub file_path: String,
}
