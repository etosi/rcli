use clap::Parser;
use rcli::cli::SubCommand;

fn main() -> anyhow::Result<()> {
    let opts = rcli::cli::Options::parse();
    match opts.cmd {
        // rcli csv -i input.csv -o output.json --header -d ','
        SubCommand::Csv(opts) => rcli::process_csv(&opts.input, &opts.output)?,
    }

    Ok(())
}
