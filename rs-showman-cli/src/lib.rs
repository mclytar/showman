pub mod interface;

#[macro_export]
macro_rules! cli_next {
    ($cmd_list:expr, $expect:expr) => {{
        let cmd = $cmd_list.next();
        if cmd.is_none() {
            println!("Expected <{}>, found end of input.", $expect);
            return CommandResult::Incomplete;
        }
        cmd.unwrap()
    }}
}

#[macro_export]
macro_rules! cli_end {
    ($cmd_list:expr) => {{
        let cmd = $cmd_list.next();
        if cmd.is_some() {
            println!("Unexpected extra token: '{}'", cmd.unwrap());
            return CommandResult::TooManyArguments;
        }
    }}
}

