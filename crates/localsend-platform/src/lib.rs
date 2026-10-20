pub mod single_instance;

pub use single_instance::{InstanceLock, scoped_lock_name};

pub fn local_ipv4_addresses() -> std::io::Result<Vec<std::net::Ipv4Addr>> {
    let mut addresses = if_addrs::get_if_addrs()?
        .into_iter()
        .filter(|interface| !interface.is_loopback())
        .filter_map(|interface| match interface.ip() {
            std::net::IpAddr::V4(address) => Some(address),
            std::net::IpAddr::V6(_) => None,
        })
        .collect::<Vec<_>>();
    addresses.sort();
    addresses.dedup();
    Ok(addresses)
}
