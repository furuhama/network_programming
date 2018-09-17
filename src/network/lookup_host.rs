use std::env;
use std::process;
use std::net::ToSocketAddrs;

pub fn run() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Error:\n    please give me just one hostname and port\nUSAGE:\n    lookup_host localhost:8080");
        process::exit(1);
    }

    let addresses = args[1].to_socket_addrs();

    match addresses {
        Err(e) => {
            eprintln!("Error:\n    {}\nUSAGE:\n    lookup_host localhost:8080", e);
            process::exit(1);
        },
        Ok(addresses) => {
            for address in addresses {
                println!("{}", address.ip());
            }
        }
    }
}
