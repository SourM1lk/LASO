# LASO - LDAP Anonymous Scanner & Observer

LASO is a fast and efficient command-line tool for scanning and identifying LDAP servers with anonymous and unauthenticated access. 

## TODO
```
Add ldaps check without cert
```

## Features
``
- Scans IP ranges for LDAP servers with anonymous and unauthenticated access
- Supports multiple ports for each IP
- Uses asynchronous programming for fast scanning
- Configurable connection limits and timeouts
- Generates a report with found servers
``

## Installation
``
1. Install Rust: https://rustup.rs/

2. Clone this repository: 
git clone https://github.com/SourM1lk/LASO.git

3. Build the project:
cd laso
cargo build --release

The compiled binary will be located in the `target/release` folder.
``

## Usage
```
./LASO --help
     ___       ___           ___           ___     
    /\__\     /\  \         /\  \         /\  \    
   /:/  /    /::\  \       /::\  \       /::\  \   
  /:/  /    /:/\:\  \     /:/\ \  \     /:/\:\  \  
 /:/  /    /::\~\:\  \   _\:\~\ \  \   /:/  \:\  \ 
/:/__/    /:/\:\ \:\__\ /\ \:\ \ \__\ /:/__/ \:\__\
\:\  \    \/__\:\/:/  / \:\ \:\ \/__/ \:\  \ /:/  /
 \:\  \        \::/  /   \:\ \:\__\    \:\  /:/  / 
  \:\  \       /:/  /     \:\/:/  /     \:\/:/  /  
   \:\__\     /:/  /       \::/  /       \::/  /   
    \/__/     \/__/         \/__/         \/__/    
                                                 
LDAP Anonymous Scanner & Observer 0.1.0
A Rust-based security tool that identifies anonymous login vulnerabilities in LDAP servers.

USAGE:
    LASO [OPTIONS] <ip-range>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --connection_limit <connection-limit>     [default: 1000]
    -p, --port <ports>...                         [default: 389]
    -t, --timeout <timeout>                       [default: 2]

ARGS:
    <ip-range>  
```

## Disclaimer
```
This software is provided "as is" without warranty of any kind, either expressed or implied, including, but not limited to, the implied warranties of merchantability and fitness for a particular purpose. The entire risk as to the quality and performance of the software is with the user.

In no event will the authors or copyright holders be liable for any damages, including lost profits, lost savings, or other incidental or consequential damages arising out of the use or inability to use the software, even if the authors or copyright holders have been advised of the possibility of such damages.

This software is intended for educational and research purposes only. The authors do not encourage or condone the use of this software for malicious activities or any actions that violate the law. It is the responsibility of the user to ensure that their use of this software complies with all applicable laws and regulations. The authors will not be held responsible for any misuse of the software.
```