use std::net::SocketAddr;
use std::fs::read_to_string;
use std::error::Error;
use std::sync::RwLockReadGuard;
use std::collections::HashMap;
use uuid::Uuid;

use crate::db::Order;

pub fn parse_env_sockargs(addr: &str, port:&str) -> SocketAddr {
    let mut pair = String::from(addr);
    pair.push(':');
    pair.push_str(port);
    let sock: SocketAddr = pair.parse().unwrap();
    sock
}

pub type Kr = KlugeRender;

pub struct KlugeRender {
    header: String,
    footer: String,
}

impl KlugeRender {
    // We should only be "init-ing" this hack once or so who cares about performant file
    // ops.
    pub fn new<P: AsRef<std::path::Path>>(head_path: P, foot_path: P) -> Result<Self, Box<dyn Error>> {
        let header = read_to_string(head_path).unwrap_or_else(|e| return e.to_string());
        let footer = read_to_string(foot_path).unwrap_or_else(|e| return e.to_string());

        Ok( KlugeRender { header, footer } ) 
    }

    pub fn index_html(&self, data: RwLockReadGuard<'_, HashMap<u128, Order>>) -> Result<String, Box<dyn Error>> {
        let mut html = String::from(&self.header);
        //let data = data.read().unwrap();
        for (_, order) in &*data {
            html.push_str(
                &format!(
                "
                <tr>
                <td>{}</td>
                <td>{}</td>
                <td>{}</td>
                <td>{}</td>
                <td>{}</td>
                </tr>
                ",
                order.status(),
                order.name(),
                order.server(),
                order.details(),
                order.submitted()))
        }
        html.push_str(&self.footer);
        Ok(html)
    }

    pub fn finish_html(&self, data: RwLockReadGuard<'_, HashMap<u128, Order>>) -> Result<String, Box<dyn Error>> {
        let mut html = String::from(&self.header);
        for (id, order) in &*data {
            let (shard0,shard1) = Uuid::from_u128(*id).as_u64_pair();
            html.push_str(
                &format!(
                "
                <tr>
                <td>{}</td>
                <td>{}</td>
                <td>{}</td>
                <td>{}</td>
                <td>{}</td>
                <td><a href='/finish/{}/{}'>Finish Order</a></td> 
                </tr>
                ",
                order.status(),
                order.name(),
                order.server(),
                order.details(),
                order.submitted(),
                shard0,shard1
                ))
        }
        html.push_str(&self.footer);
        Ok(html)
    }

}
