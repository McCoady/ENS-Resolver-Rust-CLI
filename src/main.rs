use ethers::prelude::*;
use futures::executor::block_on;
use std::env;
use std::fs;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let rpc_url = &args[2];

    let mut addresses: Vec<ethers::prelude::H160> = Vec::new();

    let ens_names = fs::read_to_string(file_path).unwrap();
    for ens_name in ens_names.split_ascii_whitespace() {
        let res = get_address(ens_name, &rpc_url);
        let _result = match block_on(res) {
            Ok(res) => addresses.push(res),
            Err(_) => eprintln!("{ens_name} : UNRESOLVED"),
        };
    }

    if env::var("REMOVE_DUP").is_ok() {
        addresses.sort();
        addresses.dedup();
    }
    eprintln!("{:?} addresses", addresses.len());
    for address in addresses {
        println!("{:?}", address);
    }
}

async fn get_address(
    name: &str,
    rpc: &String,
) -> Result<ethers::prelude::H160, ethers::providers::ProviderError> {
    let provider = Provider::<Http>::try_from(rpc).expect("Problem connecting to provider");
    let address_result = provider.resolve_name(name).await;

    address_result
}
