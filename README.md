# LDAP Anonymous Security Observer

## TODO
```
Add ldaps check without cert
```

## Help
```
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
                                                 
LDAP Anonymous Security Observer 0.1.0
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