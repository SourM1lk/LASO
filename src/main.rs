#![allow(non_snake_case)]
use colored::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use config::{CliOptions, ScannerConfig};
use scanner::scan;
use structopt::StructOpt;
use tokio::runtime::Runtime;

mod config;
mod ldap;
mod scanner;

fn main() {
    display_welcome_message();

    let options = CliOptions::from_args();
    let config = ScannerConfig::from_options(options).expect("Failed to parse command-line options");

    // Create the Tokio runtime and run the scanner
    let rt = Runtime::new().expect("Failed to create Tokio runtime");
    let valid_servers = rt.block_on(scan(config));
}

fn display_welcome_message() {
    let logo = r#"
     ___       ___           ___           ___     
    /\__\     /\  \         /\  \         /\  \    
   /:/  /    /::\  \       /::\  \       /::\  \   
  /:/  /    /:/\:\  \     /:/\ \  \     /:/\:\  \  
 /:/  /    /::\~\:\  \   _\:\~\ \  \   /:/  \:\  \ 
/:/__/    /:/\:\ \:\__\ /\ \:\ \ \__\ /:/__/ \:\__\
\:\  \    \/__\:\/:/  / \:\ \:\ \/__/ \:\  \ /:/  /
 \:\  \        \::/  /   \:\ \:\__\    \:\  /:/  / 
  \:\  \       /:/  /     \:\/:/  /     \:\/:/  /  
   \:\__\     /:/  /       \::/  /       \::/  /   
    \/__/     \/__/         \/__/         \/__/    
                                                 "#;

    let mut colors = vec![
        "red",
        "green",
        "yellow",
        "blue",
        "magenta",
        "cyan",
        "white",
    ];
    let mut rng = thread_rng();
    colors.shuffle(&mut rng);
    let mut index = 0;
    for line in logo.lines() {
        println!("{}", line.color(colors[index % colors.len()]).bold());
        index += 1;
    }
}