extern crate showman;

use std::io::{stdin, stdout, Write};

use actix_rt::System;
use dotenv::dotenv;

use showman_cli::interface::{Interface, CommandResult, InterfaceNode};
use showman_gui::preprocessor;

fn main() {
    let mut command_line_interface = Interface::new();
    let _system = System::new("showman");

    dotenv().expect("Cannot load environment variables.");
    preprocessor::update("./www/template/");

    println!("Starting server...");
    showman::start();
    println!("Server is running!");
    println!("Write a command or type 'help' for a list of commands.");

    command_line_interface
        .handle("update", |_| {
            print!("Updating definitions... ");
            preprocessor::update("./www/template/");
            dotenv().expect("Cannot load environment variables");
            println!("OK!");

            CommandResult::Ok
        })
        .handle("reload", |_| {
            print!("Stopping server... ");
            showman::stop(true);
            println!("OK!");
            print!("Reloading definitions... ");
            preprocessor::update("./www/template/");
            dotenv().expect("Cannot load environment variables");
            println!("OK!");
            print!("Restarting server... ");
            showman::start();
            println!("OK!");

            CommandResult::Ok
        })
        .handle("stop", |_| {

            print!("Stopping server... ");
            showman::stop(true);
            println!("OK!");

            CommandResult::Stop
        });

    command_line_interface.configure(showman::cli::configure);

    loop {
        print!("> ");

        stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let command = input.trim();
        if command == "" { continue; }
        let tokens = command.split_whitespace();

        match command_line_interface.parse(tokens) {
            CommandResult::Ok => { /* Do nothing... */},
            CommandResult::Stop => { break; },
            CommandResult::NotFound => { println!("\nCommand '{}' does not exist. To see a list of available commands, type 'help'.\n", command); },
            CommandResult::Incomplete => { println!("\nSome parameter is missing. To see a list of available parameters, type 'help {}'.\n", command); }
            CommandResult::CustomMessage(msg) => { println!("\n{}\n", &msg); },
            CommandResult::TooManyArguments => { println!("\nToo many arguments."); },
            CommandResult::Unimplemented => { println!("\nThis functionality is currently not implemented.\n"); }
        }
    }

    println!("Goodbye!");
}
