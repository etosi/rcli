use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Options {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV or convert CSV to other formats")]
    Csv(CsvOptions),
}

#[derive(Debug, Parser)]
pub struct CsvOptions {
    #[arg(short, long, value_parser = verify_input_file)]
    pub input: String,
    #[arg(short, long, default_value = "output.json")]
    pub output: String,
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
    #[arg(long, default_value_t = false)]
    pub header: bool,
}

fn verify_input_file(filename: &str) -> anyhow::Result<String, String> {
    if std::path::Path::new(filename).exists() {
        Ok(String::from(filename))
    } else {
        Err(format!("File {} does not exist", filename))
    }
}
