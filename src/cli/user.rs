use std::str::SplitWhitespace;

use showman_cli::interface::{CommandResult, Interface};
use showman_db::{establish_connection, schema};

use diesel::*;
use showman_db::models::User;
use showman_db::models::role::Role;

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
                "list" => {
                    println!();
                    println!("ShowMan server");
                    println!("user list [<count> [<page>]]");
                    println!();
                    println!("Show a list of users with their names and their roles.");
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
                "set-role" => {
                    println!();
                    println!("ShowMan server");
                    println!("user set-role <identifier> <role>");
                    println!();
                    println!("Set a role for a specific user.");
                    println!();
                    println!("PARAMETERS:");
                    println!("    identifier    Unique identifier for the user, i.e. user id or login email.");
                    println!("    role          New role for the user.");
                    println!();
                    println!("The possible roles are:");
                    println!("    maintainer    A maintainer organizes and manages everything and has access to all areas of the app.");
                    println!("    admin         An admin organizes and manages shows and users and has access to all areas of the app.");
                    println!("    organizer     An organizer can create new shows and accept users into their shows.");
                    println!("    user          An user is a simple user that cannot globally organize anything.");
                    println!("    banned        A banned user cannot do anything.");
                    println!();
                    CommandResult::Ok
                },
                "show" => {
                    println!();
                    println!("ShowMan server");
                    println!("user show <identifier>");
                    println!();
                    println!("Show data about a specific user.");
                    println!();
                    println!("PARAMETERS:");
                    println!("    identifier    Unique identifier for the user, i.e. user id or login email.");
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
            println!("    create      Create a new user");
            println!("    delete      Delete an existing user");
            println!("    list        Show a list of users with their names and their roles");
            println!("    set-name    Set a name for a specific user");
            println!("    set-role    Set a role for a specific user");
            println!("    show        Show data about a specific user");
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
        .handle("list", list)
        .handle("set-name", set_name)
        .handle("set-role", set_role)
        .handle("show", show);
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

pub fn list(mut cmd_list: SplitWhitespace) -> CommandResult {
    let count_str = cmd_list.next();
    let count = count_str.map(|s| s.parse::<i64>());
    let page_str = cmd_list.next();
    let page = page_str.map(|s| s.parse::<i64>());
    cli_end!(cmd_list);

    let count = match count {
        None => None,
        Some(Ok(count)) => Some(count),
        Some(Err(_)) => return CommandResult::CustomMessage(format!("Expected a number, found '{}'.", count_str.unwrap()))
    };

    let page = match page {
        None => None,
        Some(Ok(page)) => Some(page),
        Some(Err(_)) => return CommandResult::CustomMessage(format!("Expected a number, found '{}'.", page_str.unwrap()))
    };

    match establish_connection() {
        Err(_) => CommandResult::CustomMessage("Error while establishing a connection to the database.".to_owned()),
        Ok(connection) => {
            let table: Vec<User> = {
                use schema::user::dsl;

                let result = match (count, page) {
                    (Some(count), Some(page)) => dsl::user.limit(count).offset(count * (page - 1)).load(&connection),
                    (Some(count), None) => dsl::user.limit(count).load(&connection),
                    (None, None) => dsl::user.load(&connection),
                    (None, Some(_)) => return CommandResult::CustomMessage(format!("Reached unreachable condition."))
                };

                match result {
                    Ok(table) => table,
                    Err(_) => return CommandResult::CustomMessage("Error retrieving user table.".to_owned())
                }
            };

            println!();
            println!("Displaying {} items.", table.len());
            println!();
            println!("User ID   Role          Name                  Surname");
            println!("--------------------------------------------------------------------");
            for user in table {
                println!("{:>7}   {:<12}  {:<20}  {:<20}", user.user_id, user.role.to_string(), &user.name, &user.surname);
            }
            println!();

            CommandResult::Ok
        }
    }
}

pub fn set_role(mut cmd_list: SplitWhitespace) -> CommandResult {
    let identifier = cli_next!(cmd_list, "identifier");
    let role_str = cli_next!(cmd_list, "role");
    cli_end!(cmd_list);

    let role = Role::from(role_str);

    if role == Role::Invalid {
        return CommandResult::CustomMessage(format!("Role '{}' does not exist.", role_str))
    }

    if let Ok(user_id) = identifier.parse::<u32>() {
        match establish_connection() {
            Err(_) => CommandResult::CustomMessage("Error while establishing a connection to the database.".to_owned()),
            Ok(connection) => {
                match connection.transaction(|| {
                    use schema::user::dsl;
                    diesel::update(
                        dsl::user.filter(dsl::user_id.eq(user_id))
                    ).set(
                        dsl::role.eq(role)
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

pub fn show(mut cmd_list: SplitWhitespace) -> CommandResult {
    let identifier = cli_next!(cmd_list, "identifier");
    cli_end!(cmd_list);

    if let Ok(user_id) = identifier.parse::<u32>() {
        match establish_connection() {
            Err(_) => CommandResult::CustomMessage("Error while establishing a connection to the database.".to_owned()),
            Ok(connection) => {
                let user: User = {
                    use schema::user::dsl;

                    match dsl::user.filter(dsl::user_id.eq(user_id))
                        .first(&connection) {
                        Ok(user) => user,
                        Err(diesel::result::Error::NotFound) => return CommandResult::CustomMessage("The specified user does not exist.".to_owned()),
                        Err(_) => return CommandResult::CustomMessage("Error while updating the user.".to_owned())
                    }
                };

                println!();
                println!("Found!");
                println!("user id: {}", user.user_id);
                println!("name: {}", &user.name);
                println!("surname: {}", &user.surname);
                println!("role: {}", user.role.to_string());
                println!();

                CommandResult::Ok
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