use std::fs::OpenOptions;
use std::io::prelude::*;
use std::net::SocketAddr;

pub fn report_server(addr: SocketAddr, anonymous_enabled: bool, unauthenticated_enabled: bool) {
    println!(
        "Found LDAP server with anonymous access: {:?} (Anonymous: {}, Unauthenticated: {})",
        addr, anonymous_enabled, unauthenticated_enabled
    );

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("servers.txt")
        .expect("Unable to open servers file");

    let server_info = format!(
        "Server: {:?} (Anonymous: {}, Unauthenticated: {})\n",
        addr, anonymous_enabled, unauthenticated_enabled
    );

    file.write_all(server_info.as_bytes())
        .expect("Unable to write to servers file");
}
