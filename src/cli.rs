pub mod user;

use std::str::SplitWhitespace;

use showman_cli::interface::{Interface, CommandResult};

pub fn help(mut command_list: SplitWhitespace) -> CommandResult {
    match command_list.next() {
        Some(command) => {
            match command {
                "help" => help(command_list),
                "reload" => {
                    println!();
                    println!("ShowMan server");
                    println!("reload");
                    println!();
                    println!("Reload the server and all its configuration and templates.");
                    println!();
                    CommandResult::Ok
                },
                "update" => {
                    println!();
                    println!("ShowMan server");
                    println!("update");
                    println!();
                    println!("Reload the server's configuration and templates, without stopping and restarting the server.");
                    println!();
                    CommandResult::Ok
                },
                "user" => user::help(command_list),
                "stop" => {
                    println!();
                    println!("ShowMan server");
                    println!("update");
                    println!();
                    println!("Halt the server and exit from the application.");
                    println!();
                    CommandResult::Ok
                },
                _ => {
                    println!();
                    println!("Invalid command.");
                    println!("For a list of available commands, type 'help'.");
                    println!();
                    CommandResult::Ok
                }
            }
        },
        None => {
            println!();
            println!("ShowMan server");
            println!();
            println!("Here is a list of possible showman commands:");
            println!("    help      Display this text");
            println!("    reload    Reload the server and all its configuration and templates");
            println!("    update    Reload the server's configuration and templates");
            println!("    user      Manage users");
            println!("    stop      Shut down the server");
            println!();
            println!("See 'help <command>' for specific help.");
            println!();
            CommandResult::Ok
        }
    }
}

pub fn configure(interface: &mut Interface) {
    let interface_user = Interface::with_configuration(user::configure);

    interface
        .subcommand("user", interface_user)
        .handle("help", help);
}