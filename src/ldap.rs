use std::net::SocketAddr;
use ldap3::{LdapConnAsync, LdapError, LdapConnSettings};

pub async fn check_ldap_anonymous(addr: SocketAddr) -> Result<(bool, bool), LdapError> {
        let scheme = if addr.port() == 636 { "ldaps" } else { "ldap" };
        let ldap_url = format!("{}://{}", scheme, addr);
    
        let settings = LdapConnSettings::new()
            .set_no_tls_verify(true); // Disable certificate validation
    
        let (_, mut ldap) = LdapConnAsync::with_settings(settings, &ldap_url).await?;
    

    println!("ldap trying {}", ldap_url);
    // Check for anonymous bind
    let bind_result_anonymous = ldap.simple_bind("", "").await?;
    let anonymous_enabled = bind_result_anonymous.success().is_ok();
    println!("Trying Anonymous Enabled");
    println!("Anoynmous Enabled {}", anonymous_enabled);

    // Check for unauthenticated authentication
    let dn = "cn=unauthenticated";
    let bind_result_unauthenticated = ldap.simple_bind(dn, "").await?;
    let unauthenticated_enabled = bind_result_unauthenticated.success().is_ok();
    println!("Trying Unauthenticated Enabled");
    println!("Unauthenticated Enabled {}", unauthenticated_enabled);
    ldap.unbind().await?;

    Ok((anonymous_enabled, unauthenticated_enabled))
}

