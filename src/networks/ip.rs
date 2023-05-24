use std::net::UdpSocket;
use serde::Deserialize;

/// 获取本机局域网IP
///
/// # Examples
/// ```
/// use toys::networks::ip::get_internal_ip;
/// assert_eq!(get_internal_ip().unwrap(),"192.168.0.100".to_string())
/// ```
pub fn get_internal_ip() -> Option<String>{
    // 创建udp连接
    let udp_socket: UdpSocket = UdpSocket::bind("0.0.0.0:0").ok()?;
    // 建立连接
    udp_socket.connect("8.8.8.8:80").ok()?;
    // 获取udp客户端,也就是本机的IP
    udp_socket.local_addr().ok()?.ip().to_string().into()
}

// 公网信息请求响应体
#[derive(Deserialize,Debug)]
#[warn(non_snake_case)]
pub struct IP{
    // 国家
    pub country: String,
    // 地区名
    #[serde(rename = "regionName")]
    pub region_name: String,
    // 公网IP
    pub query: String
}

/// 获取本机公网IP等信息
///
/// # Examples
/// ```
/// use toys::networks::ip::get_public_ip;
/// assert_eq!(get_public_ip().unwrap(),"168.138.213.6".to_string());
/// ```
pub fn get_public_ip() -> Option<String>{
    // 请求获取公网信息
    let response = reqwest::blocking::get("http://ip-api.com/json/").ok()?;
    // 映射为IP结构体,并且获取query-IP
    response.json::<IP>().ok()?.query.into()
}

#[cfg(test)]
mod tests{
    use crate::networks::ip::{get_internal_ip, get_public_ip};

    #[test]
    pub fn test_get_internal_ip(){
        let ip = get_internal_ip().unwrap();
        assert_eq!(ip,"192.168.0.100".to_string());
    }

    #[test]
    pub fn test_get_public_ip(){
        let ip = get_public_ip().unwrap();
        assert_eq!(ip,"168.138.213.6".to_string());
    }
}