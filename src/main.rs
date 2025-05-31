use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The vendor ID of the device
    #[arg(short, long)]
    vid: String,
    /// The product ID of the device
    #[arg(short, long)]
    pid: String,
    /// The name of the device
    #[arg(short, long)]
    name: Option<String>,
}

fn main() {
    let _args = Args::parse();
}
