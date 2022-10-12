use clap::{command, Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
#[command(author, version, about, long_about = None)]
#[command(name = "ezp2010")]
#[command(author = "Håvard Sørbø <havard@hsorbo.no>")]
#[command(version = "0.1")]
#[command(about = "Read and write flash-roms using ezp2010", long_about = None)]
pub struct EzpArgs {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Read from rom
    Read(ReadWriteCommand),
    /// Write to rom
    Write(ReadWriteCommand),
    /// Shows information about connected programmer
    Info,
    /// Erase (on supported chips)
    Erase,
    /// Detect rom
    Detect,
    /// Shows available flash rom type
    List,
}

#[derive(Debug, Args)]
pub struct ReadWriteCommand {
    /// Type of rom. [ezp2000 list] for list
    #[arg(short = 't', long = "type", value_name = "type")]
    pub chip_type: String,
    pub file: String
}
