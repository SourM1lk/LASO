use std::net::IpAddr;
use std::str::FromStr;
use structopt::StructOpt;
use std::num::ParseIntError;

#[derive(StructOpt)]
#[structopt(
    name = "LDAP Anonymous Scanner & Observer",
    about = "A Rust-based security tool that identifies anonymous login vulnerabilities in LDAP servers.",
)]
pub struct CliOptions {
    pub ip_range: String, 
    #[structopt(long = "connection_limit", short = "c", default_value = "1000")]
    pub connection_limit: usize,
    #[structopt(long = "port", short = "p", default_value = "389", use_delimiter = true)]
    pub ports: Vec<Port>,
    #[structopt(long = "timeout", short = "t", default_value = "2")]
    pub timeout: u64,
}

pub struct ScannerConfig {
    pub ip_range: (IpAddr, IpAddr),
    pub connection_limit: usize,
    pub ports: Vec<Port>,
    pub timeout: u64,
}

impl ScannerConfig {
    pub fn from_options(options: CliOptions) -> Result<ScannerConfig, &'static str> {
        // Parse the IP range
        let ip_range = parse_ip_range(&options.ip_range)?;

        Ok(ScannerConfig {
            ip_range,
            connection_limit: options.connection_limit,
            ports: options.ports,
            timeout: options.timeout
        })
    }
}

pub enum Port {
    Single(u16),
}

impl Port {
    pub fn to_u16(&self) -> u16 {
        match self {
            Port::Single(port) => *port,
        }
    }
}


impl FromStr for Port {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s.parse::<u16>()?;
        Ok(Port::Single(value))
    }
}


fn parse_ip_range(ip_range_str: &str) -> Result<(IpAddr, IpAddr), &'static str> {
    let ips: Vec<&str> = ip_range_str.split('-').collect();
    if ips.len() != 2 {
        return Err("Invalid IP range format. Please use the format: START_IP-END_IP.");
    }

    let start_ip = IpAddr::from_str(ips[0]).map_err(|_| "Invalid start IP address.")?;
    let end_ip = IpAddr::from_str(ips[1]).map_err(|_| "Invalid end IP address.")?;

    Ok((start_ip, end_ip))
}