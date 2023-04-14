use std::net::SocketAddr;
use ldap3::{LdapConnAsync, LdapError};

async fn check_anonymous_bind(url: &str) -> Result<bool, LdapError> {
    let (conn, mut ldap) = LdapConnAsync::new(url).await?;
    ldap3::drive!(conn);
    let bind_result = ldap.simple_bind("", "").await?;
    let is_enabled = bind_result.success().is_ok();
    ldap.unbind().await?;
    Ok(is_enabled)
}

pub async fn check_ldap_anonymous(addr: SocketAddr) -> Result<(bool, bool), LdapError> {
    let scheme = if addr.port() == 636 { "ldaps" } else { "ldap" };
    let ldap_url = format!("{}://{}", scheme, addr);
    let is_anonymous_enabled = check_anonymous_bind(&ldap_url).await?;
    
    let dn = "cn=unauthenticated";
    let (conn, mut ldap) = LdapConnAsync::new(&ldap_url).await?;
    ldap3::drive!(conn);
    let bind_result = ldap.simple_bind(dn, "").await?;
    let is_unauthenticated_enabled = bind_result.success().is_ok();
    ldap.unbind().await?;
    Ok((is_anonymous_enabled, is_unauthenticated_enabled))
}
