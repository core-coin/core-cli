use cli_error::CliError;
use std::collections::HashMap;
use std::process;
use types::Response;

pub type BaseFunctions = HashMap<String, Box<dyn Fn()>>; // type alias

pub fn base_functions() -> BaseFunctions {
    let mut functions = BaseFunctions::new();
    functions.insert("list".to_string(), Box::new(list));
    functions.insert("help".to_string(), Box::new(list));
    functions.insert("exit".to_string(), Box::new(exit));
    functions
}

fn list() {
    println!("Available base commands:");
    println!("'list' or 'help' - display this help message");
    println!("'exit' - exit the console");
    println!("Available modules:");
    println!("'xcb' - XCB module commands:");
    println!("  'get_block_height()' - get the current block height");
    println!("  'get_block(<hash>|<number>|'latest')' - get block information by hash or number. Use 'latest' to get the latest block");
    println!("  'get_energy_price()' - get the current energy price to allow a timely execution of a transaction");

    println!("'xcbkey' - XCB Key module commands:");
    println!("  'get_key()' - get the current key");
    println!("Example usage:");
    println!("  xcb.get_block_height()");
    println!("  xcb.block('latest')");
    println!("  xcb.block('0x1234')");
    println!("For every command, tou can last argument as 'json' to get the response in JSON format, e.g. xcb.get_block_height('json')");
    println!("For more information, please refer to the documentation.");
}

fn exit() {
    println!("Exiting...");
    process::exit(0);
}
