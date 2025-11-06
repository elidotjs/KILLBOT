use std::path::PathBuf;
use clap::{Parser, Subcommand};
use crate::toki::{tokenize};
use colored::Colorize;
use consts::*;

mod rusty;
mod toki;
mod pillow;
mod consts;

#[derive(Debug)]
#[derive(Parser)]
#[command(disable_help_subcommand = true)]
#[command(disable_help_flag = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>
}

#[derive(Debug)]
#[derive(Subcommand)]
enum Commands {
    #[command(short_flag = 'h')]
    /// Shows how to use KILLBOT.
    Help,
    #[command(short_flag = 'c')]
    /// Compiles a file
    Compile {
        path: PathBuf
    },
    Info {
        category: Option<String>
    }
}

fn print_page(page: &Page) {
    println!("=============================================================
{}
=============================================================
{}
=============================================================
{}", LOGO.green(), page.label.cyan(), page.content)
}


fn compile(path: &PathBuf) {
    let contents = std::fs::read_to_string(path).unwrap();

    pillow::to_instructions(tokenize(contents));
}

fn main() {
    let args = Cli::parse();

    if let Some(command) = &args.command {
        match command {
            Commands::Compile { path } => {
                compile(path)
            }
            Commands::Help => {
                print_page(
                    &PAGES[1]
                )
            }
            Commands::Info {
                category
            } => {
                if category.is_none() {

                }
            }
        }
    } else {
        print_page(
            &PAGES[0]
        )
    }
}