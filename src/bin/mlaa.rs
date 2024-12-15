use clap::Parser;
use vertin::mlaa::MLAAPrecompute;
use anyhow::Result;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    output: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let im = MLAAPrecompute::precompute_lut();
    im.save(args.output)?;

    Ok(())
}
