use std::collections::HashMap;
use std::process;

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
    println!("  'get_network_id()' - get the nework ID of the current network");

    println!("'xcbkey' - XCB Key module commands:");
    println!("  'list()' - list all accounts");
    println!("  'new(optional!<password>)' - create a new account. If password is not provided, it will be prompted (it is not recommended to provide the password as an argument)");
    println!("  'new_from_key(optional! <private_key>, optional! <password>)' - create a new account from existing private key. If key or password are not provided, they will be asked during the execution");
    println!("  'unlock(optional! <address>, optional! <password>)' - unlock an account for a signining session");
    println!("  'sign(optional! <address>, optional! <message>)' - sign a message with the unlocked account");
    println!("  'verify(optional! <address>, optional! <signature>, optional! <message>)' - verify that the signature is correct for the message and address");

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
