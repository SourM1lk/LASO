use crate::config::ScannerConfig;
use crate::ldap::check_ldap_anonymous;
use crate::report::report_server;
use std::net::{IpAddr, SocketAddr, Ipv4Addr};
use tokio::sync::Semaphore;
use tokio::time::{timeout, Duration};
use indicatif::{ProgressBar, ProgressStyle};
use std::sync::Arc;

pub async fn scan(config: ScannerConfig) -> Vec<SocketAddr> {
    let semaphore = Arc::new(Semaphore::new(config.connection_limit));
    let (start_ip, end_ip) = config.ip_range;
    let mut valid_servers = Vec::new();
    let chunk_size = config.connection_limit;

    let total_ips = ip_range(start_ip, end_ip).count();

    let pb = ProgressBar::new(total_ips as u64);
    let progress_style = ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos:>7}/{len:7}")
        .expect("Failed to create progress bar template")
        .progress_chars("=#-");
    pb.set_style(progress_style);

    let mut tasks = Vec::new();
    let mut task_count = 0;

    for ip in ip_range(start_ip, end_ip) {
        for port in &config.ports {
            let addr = SocketAddr::new(ip, port.to_u16());
            let pb_clone = pb.clone();
            let semaphore_clone = Arc::clone(&semaphore);

            let task = tokio::spawn(async move {
                let _permit = semaphore_clone.acquire().await.unwrap();

                match timeout(Duration::from_secs(config.timeout), check_ldap_anonymous(addr)).await {
                    Ok(Ok((anonymous_enabled, unauthenticated_enabled))) => {
                        if anonymous_enabled || unauthenticated_enabled {
                            report_server(addr, anonymous_enabled, unauthenticated_enabled);
                            return Some(addr);
                        }
                    }
                    _ => {}
                }
                pb_clone.inc(1);
                None
            });

            tasks.push(task);
            task_count += 1;

            if task_count == chunk_size {
                for task in tasks.drain(..) {
                    if let Ok(Some(server)) = task.await {
                        valid_servers.push(server);
                    }
                }
                task_count = 0;
            }
        }
    }

    // Process the remaining tasks
    for task in tasks {
        if let Ok(Some(server)) = task.await {
            valid_servers.push(server);
        }
    }

    pb.finish();
    valid_servers
}


fn ip_range(start_ip: IpAddr, end_ip: IpAddr) -> impl Iterator<Item = IpAddr> {
    let start = match start_ip {
        IpAddr::V4(v4) => u32::from(v4),
        _ => panic!("Start IP must be IPv4"),
    };
    let end = match end_ip {
        IpAddr::V4(v4) => u32::from(v4),
        _ => panic!("End IP must be IPv4"),
    };

    (start..=end).map(|ip| IpAddr::V4(Ipv4Addr::from(ip)))
}