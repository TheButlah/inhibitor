use std::{
    io::{Read, Write},
    num::ParseIntError,
    path::{Path, PathBuf},
    process::ExitCode,
};

use clap::{
    Parser, Subcommand,
    builder::{Styles, styling::AnsiColor},
};
use owo_colors::{OwoColorize, Stream::Stdout};

fn clap_v3_styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::Yellow.on_default())
        .usage(AnsiColor::Green.on_default())
        .literal(AnsiColor::Green.on_default())
        .placeholder(AnsiColor::Green.on_default())
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, styles=clap_v3_styles())]
struct Args {
    #[command(subcommand)]
    cmds: Commands,
}

#[derive(Parser, Debug)]
struct FilterArgs {
    /// The vendor ID of the device
    #[arg(short, long, value_parser = parse_hex_u16)]
    vid: Option<u16>,
    /// The product ID of the device
    #[arg(short, long, value_parser = parse_hex_u16)]
    pid: Option<u16>,
    /// The name of the device
    #[arg(short, long)]
    name: Option<String>,
}

#[derive(Parser, Debug)]
struct ListCommand {
    #[command(flatten)]
    filter: FilterArgs,
}

fn enumerate_with_filter(filter: &FilterArgs) -> impl Iterator<Item = (PathBuf, evdev::Device)> {
    evdev::enumerate()
        .filter(|(_, d)| {
            filter
                .name
                .as_ref()
                .is_none_or(|name| name == d.name().unwrap_or_default())
        })
        .filter(|(_, d)| {
            filter
                .vid
                .as_ref()
                .is_none_or(|vid| *vid == d.input_id().vendor())
        })
        .filter(|(_, d)| {
            filter
                .pid
                .as_ref()
                .is_none_or(|pid| *pid == d.input_id().product())
        })
}

impl ListCommand {
    fn run(self) -> Result<(), ExitCode> {
        for (path, device) in enumerate_with_filter(&self.filter) {
            let prefix = Path::new("/sys/class/input");
            let device_path = path
                .file_name()
                .and_then(|n| prefix.join(n).join("device").canonicalize().ok());
            println!(
                "{} {}, {} {:04x}, {} {:04x}, {} {}",
                "Name:".if_supports_color(Stdout, |text| text.green()),
                device.name().unwrap_or("??"),
                "Vendor ID:".if_supports_color(Stdout, |text| text.green()),
                device.input_id().vendor(),
                "Product ID:".if_supports_color(Stdout, |text| text.green()),
                device.input_id().product(),
                "Device Path:".if_supports_color(Stdout, |text| text.green()),
                device_path.unwrap_or_else(|| PathBuf::from("??")).display()
            );
        }

        Ok(())
    }
}

#[derive(Debug)]
struct InhibitCommand {
    action: InhibitAction,
    filter: FilterArgs,
}

impl InhibitCommand {
    fn run(self) -> Result<(), ExitCode> {
        let mut iter = enumerate_with_filter(&self.filter);
        let Some((path, _device)) = iter.next() else {
            eprintln!("could not find any devices matching the filter!");
            return Err(ExitCode::FAILURE);
        };
        if iter.next().is_some() {
            eprintln!("refusing to manipulate multiple devices: narrow your filter");
            return Err(ExitCode::FAILURE);
        }
        println!("path: {path:?}");
        let prefix = Path::new("/sys/class/input");
        let event_suffix = path.file_name().expect("should not be empty path");
        let inhibit_path = prefix.join(event_suffix).join("device").join("inhibited");

        let Ok(mut file) = std::fs::File::options()
            .read(self.action == InhibitAction::Toggle)
            .write(true)
            .open(&inhibit_path)
        else {
            eprintln!("failed to open {inhibit_path:?}, perhaps you need to run as root?");
            return Err(ExitCode::FAILURE);
        };

        let result = match self.action {
            InhibitAction::Enable => file.write_all(b"0"),
            InhibitAction::Disable => file.write_all(b"1"),
            InhibitAction::Toggle => {
                let mut current = [69];
                file.read_exact(&mut current).and_then(|()| {
                    assert!(current[0] != 69);
                    let inhibited = &current == b"1";
                    // Toggles the state
                    let payload = if inhibited { b"0" } else { b"1" };
                    file.write_all(payload)
                })
            }
        };

        if result.is_err() {
            eprintln!("failed to write to {inhibit_path:?}, perhaps you need to run as root?");
            return Err(ExitCode::FAILURE);
        }

        Ok(())
    }
}

#[derive(Debug, Eq, PartialEq)]
enum InhibitAction {
    Enable,
    Disable,
    Toggle,
}

#[derive(Subcommand, Debug)]
enum Commands {
    List(ListCommand),
    Enable(FilterArgs),
    Disable(FilterArgs),
    Toggle(FilterArgs),
}

impl Commands {
    fn run(self) -> Result<(), ExitCode> {
        match self {
            Commands::List(list_command) => list_command.run(),
            Commands::Enable(filter) => InhibitCommand {
                action: InhibitAction::Enable,
                filter,
            }
            .run(),
            Commands::Disable(filter) => InhibitCommand {
                action: InhibitAction::Disable,
                filter,
            }
            .run(),
            Commands::Toggle(filter) => InhibitCommand {
                action: InhibitAction::Toggle,
                filter,
            }
            .run(),
        }
    }
}

fn parse_hex_u16(s: &str) -> Result<u16, ParseIntError> {
    u16::from_str_radix(s, 16)
}

// TODO: Better error handling than ExitCode
fn main() -> Result<(), ExitCode> {
    let args = Args::parse();
    args.cmds.run()
}
