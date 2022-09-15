use ethers::prelude::*;
use futures::executor::block_on;
use std::env;
use std::fs;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let rpc_url = &args[2];

    let mut count = 0;

    println!("[");

    let addresses = fs::read_to_string(file_path).unwrap();
    for address in addresses.split_ascii_whitespace() {
        let res = get_address(address, &rpc_url);
        block_on(res);
        count += 1;
    }
    println!("]");
    eprintln!("{} addresses", count);
}

async fn get_address(name: &str, rpc: &String) {
    let provider = Provider::<Http>::try_from(rpc).expect("Problem connecting to provider");
    let address_result = provider.resolve_name(name).await;

    let _address = match address_result {
        Ok(res) => println!("{:?},", res),
        Err(_) => eprintln!("{name} : UNRESOLVED"),
    };
}

// allow separate by something other than space
// errors for non addresses
