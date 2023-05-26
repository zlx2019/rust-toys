use std::net::UdpSocket;
use std::string::ToString;
use serde::Deserialize;
use crate::data::json::from_json_str;

// IP相关信息实体
#[derive(Deserialize,Debug)]
#[warn(non_snake_case)]
pub struct IPInfo {
    // 所属国家
    pub country: String,
    // 地区名
    #[serde(rename = "regionName")]
    pub region_name: String,
    // 公网IP
    pub query: String,
    // 纬度
    pub lat: f64,
    // 经度
    pub lon: f64,
}

#[derive(Deserialize,Debug)]
#[warn(non_snake_case)]
pub struct IPAddress{
    // 省份
    pub pro: String,
    // 城市
    pub city: String,
    // 详细地址,境外地区时使用该字段
    pub addr: String,
    // 错误提示
    // noprovince: 表示无省份名,可能是国外地区
    // nocity: 表示无城市名,可能是一些直辖市
    pub err: String
}

impl IPAddress {
    // 获取地区名
    pub fn get_name(&self) -> String{
        let empty = String::from("");
        let no_province = String::from("noprovince");
        let no_city = String::from("nocity");
        // 国内地区,获取省份和城市
        if self.err == empty {
            format!("{}{}",self.pro,self.city)
        }else if self.err ==  no_province{
            // 海外地区,获取addr
            self.addr.clone()
        }else if self.err == no_city {
            //直辖市,只获取省份
            self.pro.clone()
        }else {
            String::from("Unknown")
        }
    }
}

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


/// 获取本机公网IP
///
/// # Examples
/// ```
/// use toys::networks::ip::get_public_ip;
/// assert_eq!(get_public_ip().unwrap(),"168.138.213.6".to_string());
/// ```
pub fn get_public_ip() -> Option<String>{
    Some(get_ip_info().ok()?.query)
}

/// 获取IP的经度纬度
///
/// # Examples
/// ```
/// use toys::networks::ip::get_ip_lat_lon;
/// assert_eq!(get_ip_lat_lon().unwrap(),(35.798, 140.1803))
/// ```
pub fn get_ip_lat_lon()-> Option<(f64,f64)>{
    get_ip_info().ok().map(|info| (info.lat,info.lon))
}



/// 获取本机IP相关信息
/// # Examples
/// ```rust
/// use toys::networks::ip::get_ip_info;
/// assert_eq!(get_ip_info().unwrap().query,"103.149.249.231".to_string());
/// ```
pub fn get_ip_info() -> Result<IPInfo,Box<dyn std::error::Error>>{
    // 请求获取公网信息
    let response = reqwest::blocking::get("http://ip-api.com/json/")?;
    // 映射为IPInfo结构体
    Ok(response.json::<IPInfo>()?)
}

/// 获取本机IP相关信息(异步)
/// # Examples
/// ```
///use toys::networks::ip::get_ip_info_async;
///#[tokio::test]
///async fn test_get_ip_info_async() -> Result<(),Box<dyn std::error::Error>>{
///  assert_eq!(get_ip_info_async().await.unwrap().query,"103.149.249.231".to_string());
///  Ok(())
/// }
/// ```
pub async fn get_ip_info_async() -> Result<IPInfo,Box<dyn std::error::Error>>{
    Ok(reqwest::get("http://ip-api.com/json/").await?
        .json::<IPInfo>().await?)
}

/// 获取IP地区相关信息
///
/// # Examples
/// ```
/// use toys::networks::ip::{get_ip_address_info, IPAddress};
/// let address: IPAddress = get_ip_address_info("103.149.249.231").unwrap();
/// assert_eq!(address.pro,"香港".to_string());
/// ```
pub fn get_ip_address_info(ip: &str) -> Result<IPAddress,Box<dyn std::error::Error>>{
    // 拼接查询url,根据ip查询
    let url = format!("https://whois.pconline.com.cn/ipJson.jsp?ip={}&json=true",ip);
    // 获取响应
    let response = reqwest::blocking::get(url)?;
    // 将响应映射到实体
    Ok(from_json_str(&(response.text()?))?)
}

/// 获取IP地区相关信息(异步)
/// # Examples
///
/// ```rust
/// use toys::networks::ip::get_ip_address_info_async;
/// #[tokio::test]
/// async fn test_get_ip_address_info_async() -> Result<(),Box<dyn std::error::Error>>{
///     assert_eq!(get_ip_address_info_async("103.149.249.231").await.unwrap().pro,"香港");
///     Ok(())
/// }
/// ```
pub async fn get_ip_address_info_async(ip: &str)-> Result<IPAddress,Box<dyn std::error::Error>>{
    let url =  format!("https://whois.pconline.com.cn/ipJson.jsp?ip={}&json=true",ip);
    Ok(from_json_str(&reqwest::get(url).await?
                                    .text().await?)?)
}

/// 请求天气信息响应体
#[derive(Deserialize,Debug)]
pub struct WeatherInfo{
    pub weather: Vec<Weather>, // 天气
    pub main: Temperature, // 气温
    pub wind: Wind,        // 风速
    pub dt: i64,//数据计算时间戳 单位:unix

}

/// 天气信息
#[derive(Deserialize,Debug)]
pub struct Weather{
    // 天气状况
    pub main: String,
    // 中文天气状况
    #[serde(rename = "description")]
    pub desc: String,
}

///气温相关信息
#[derive(Deserialize,Debug)]
pub struct Temperature{
    // 温度
    pub temp: f64,
    // 最低温度
    pub temp_min: f64,
    // 最高温度
    pub temp_max: f64,
    // 湿度百分比
    pub humidity: f64
}

/// 风速相关信息
#[derive(Deserialize,Debug)]
pub struct Wind{
    // 风速 单位: 米/秒
    pub speed: f64,
    // 风向度数
    pub deg: usize
}


/// 根据一个坐标获取当前的天气信息
/// # Examples
/// ```
///
/// ```
pub async fn get_weather(lat: f64, lon: f64) -> Result<WeatherInfo,Box<dyn std::error::Error>>{
    // 拼接获取天气的API,根据经度纬度查询
    let url = format!("https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&lang=zh_cn&appid=55c1be18ce60cbae87c80507ef061ff6&units=metric",lat,lon);
    // 发起异步请求,将响应体以json格式序列化为WeatherInfo
    Ok(
        reqwest::get(url).await?
            .json::<WeatherInfo>().await?
    )
}


#[cfg(test)]
mod tests{
    use reqwest::Error;
    use crate::networks::ip::{get_internal_ip, get_ip_address_info, get_ip_info_async, get_public_ip};

    #[test]
    pub fn test_get_internal_ip(){
        let ip = get_internal_ip().unwrap();
        assert_eq!(ip,"192.168.0.100".to_string());
    }

    #[test]
    pub fn test_get_public_ip(){
        // let ip = get_public_ip().unwrap();
        // assert_eq!(ip,"45.62.169.92".to_string());
    }

    #[test]
    pub fn test_get_ip_address_name(){
        let address = get_ip_address_info("103.149.249.231").unwrap();
        println!("{:?}",address);
    }

    #[tokio::test]
    pub async fn test_get_ip_info_async()-> Result<(),std::io::Error>{
        assert_eq!(get_ip_info_async().await.unwrap().query,"103.149.249.231".to_string());
        Ok(())
    }
}