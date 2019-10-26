use std::str::SplitWhitespace;

use showman_cli::interface::{CommandResult, Interface};
use showman_db::{establish_connection, schema};

use diesel::*;

pub fn help(mut command_list: SplitWhitespace) -> CommandResult {
    match command_list.next() {
        Some(command) => {
            match command {
                "create" => {
                    println!();
                    println!("ShowMan server");
                    println!("user create <parameters>");
                    println!();
                    println!("Create a new user.");
                    println!("This option is not implemented yet.");
                    println!();
                    CommandResult::Ok
                },
                "delete" => {
                    println!();
                    println!("ShowMan server");
                    println!("user delete <identifier>");
                    println!();
                    println!("Delete an existing user.");
                    println!("This option is not implemented yet.");
                    println!();
                    CommandResult::Ok
                },
                "role" => {
                    println!();
                    println!("ShowMan server");
                    println!("user role <role> <identifier>");
                    println!();
                    println!("Set a role for a specific user.");
                    println!("This option is not implemented yet.");
                    println!();
                    CommandResult::Ok
                },
                "set-name" => {
                    println!();
                    println!("ShowMan server");
                    println!("user set-name <identifier> <name> <surname>");
                    println!();
                    println!("Sets the name and surname of a specific user.");
                    println!();
                    println!("PARAMETERS:");
                    println!("    identifier    Unique identifier for the user, i.e. user id or login email.");
                    println!("    name          New name for the user.");
                    println!("    surname       New surname for the user.");
                    println!();
                    CommandResult::Ok
                },
                "show" => {
                    println!();
                    println!("ShowMan server");
                    println!("user show <identifier>");
                    println!();
                    println!("Show data about a specific user.");
                    println!("This option is not implemented yet.");
                    println!();
                    CommandResult::Ok
                }
                _ => {
                    println!();
                    println!("Invalid subcommand for 'user'.");
                    println!("For a list of available subcommands, type 'help user'.");
                    println!();
                    CommandResult::Ok
                }
            }
        },
        None => {
            println!();
            println!("ShowMan server.");
            println!("user");
            println!();
            println!("Manage users; for details, see the list of available 'user' commands.");
            println!();
            println!("Here is a list of possible showman 'user' commands:");
            println!("    create    Create a new user");
            println!("    delete    Delete an existing user");
            println!("    set-name  Set a name for a specific user");
            println!("    set-role  Set a role for a specific user");
            println!("    show      Show data about a specific user");
            println!();
            println!("See 'help user <command>' for specific help.");
            println!();
            CommandResult::Ok
        }
    }
}

pub fn configure(interface: &mut Interface) {
    interface
        .handle("create", |_| CommandResult::Unimplemented)
        .handle("delete", |_| CommandResult::Unimplemented)
        .handle("set-name", set_name)
        .handle("set-role", |_| CommandResult::Unimplemented)
        .handle("show", |_| CommandResult::Unimplemented);
}

pub fn set_name(mut cmd_list: SplitWhitespace) -> CommandResult {
    let identifier = cli_next!(cmd_list, "identifier");
    let name = cli_next!(cmd_list, "name");
    let surname = cli_next!(cmd_list, "surname");
    cli_end!(cmd_list);

    if let Ok(user_id) = identifier.parse::<u32>() {
        match establish_connection() {
            Err(_) => CommandResult::CustomMessage("Error while establishing a connection to the database.".to_owned()),
            Ok(connection) => {
                match connection.transaction(|| {
                    use schema::user::dsl;
                    diesel::update(
                        dsl::user.filter(dsl::user_id.eq(user_id))
                    ).set(
                        (dsl::name.eq(name), dsl::surname.eq(surname))
                    ).execute(&connection)?;

                    Ok(())
                }) {
                    Ok(_) => {
                        println!("\nUser data updated successfully!\n");

                        CommandResult::Ok
                    },
                    Err(diesel::result::Error::NotFound) => CommandResult::CustomMessage("The specified user does not exist.".to_owned()),
                    Err(_) => CommandResult::CustomMessage("Error while updating the user.".to_owned())
                }
            }
        }
    } else {
        CommandResult::Unimplemented
    }
}

pub fn get() {
    /*let usr = cli_next!(command_list, "id");
    cli_end!(command_list);

    let usr = if let Ok(usr) = u32::from_str(usr) {
        usr
    } else {
        println!("Error: '{}' is not a number.", usr);
        return;
    };

    let connection = cli_try!(establish_connection());
    let result: Vec<User> = cli_try!(
        user.filter(user_id.eq(usr))
            .load::<User>(&connection)
    );
    if result.len() == 0 {
        println!("Error: '{}' not found.", usr);
        return;
    }
    let result = &result[0];

    println!("User #{}", result.user_id);
    println!("  Name: {}", &result.name);
    println!("  Surname: {}", &result.surname);
    println!("  Role: {}", &result.role.to_string());

    println!("Done!");*/
    println!("Not implemented yet!");
}