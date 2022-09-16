# ENS-Resolver-Rust-CLI

A CLI app that takes space separated .txt file of ENS names and pushes resolved addresses to an output file.

## Details

The app outputs a txt file that adds resolves addresses into an array format. 
If an ENS name does not resolve the error message is printed to the terminal informing you which name has no address.
Finally the program tells you the number of names in the original input file

## Usage

```
$ cargo run {ensnames.txt} {rpc_url} > {outputfile.txt}
```

