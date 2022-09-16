# ENS-Resolver-Rust-CLI

A CLI app that takes space separated .txt file of ENS names and pushes resolved addresses to an output file.

## Details

The app outputs a txt file of the resolved addresses. 
If an ENS name does not resolve the error message is printed to the terminal informing you which name has no address.
Finally the program tells you the number of addresses in the final list

## Usage

```
$ cargo run {ensnames.txt} {rpc_url} > {outputfile.txt}
```

Remove duplicate addresses from the output file by adding `REMOVE_DUP=1` to the terminal command:

```
$ REMOVE_DUP=1 cargo run {ensnames.txt} {rpc_url} > {nodupesoutput.txt}
```
