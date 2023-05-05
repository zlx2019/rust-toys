//! # HTTP请求函数模块

use std::collections::HashMap;
use std::time::Duration;
use lazy_static::lazy_static;

// 全局静态属性
lazy_static!{
    // HTTP同步客户端
    static ref CLIENT: reqwest::blocking::Client = reqwest::blocking::Client::builder().timeout(Duration::from_secs(3)).build().unwrap();
    // HTTP异步客户端
    static ref CLIENT_ASYNC: reqwest::Client = reqwest::Client::builder().timeout(Duration::from_secs(3)).build().unwrap();
}


/// 发送GET请求(同步),将响应体转换为String并且返回.
/// query Query请求参数
pub fn get(url: &str, query: &HashMap<String,String>) -> reqwest::Result<String> {
    CLIENT.get(url)
        // 设置超时时间为3s
        // 指定Query参数
        .query(&query)
        // 发送请求
        .send()?
        // 获取响应数据
        .text()
}


/// 发送GET请求(异步),以Text文本格式获取结果
pub async fn get_async(url: &str,query: &HashMap<String,String>) -> reqwest::Result<String> {
    CLIENT_ASYNC.get(url)
        // 设置超时时间为3s
        .query(query)
        // 发送请求
        .send().await?
        // 以text格式获取结果
        .text().await
}

#[cfg(test)]
mod tests{
    use crate::http::*;

    /// 单元测试,同步Get请求
    #[test]
    fn test_get(){
        // 构建Query参数
        let mut query = HashMap::new();
        query.insert("name".to_string(),"张三".to_string());
        // 执行同步GET请求
        let text: reqwest::Result<String> = get("http://127.0.0.1:13001/example/index", &query);
        // 判断结果是否成功
        match text {
            Ok(t) => println!("{}",t),
            Err(e) => println!("发生错误:{}",e.to_string())
        }
    }

    /// 异步函数单元测试,异步Get请求
    #[tokio::test]
    async fn test_get_async() -> Result<(),Box<dyn std::error::Error>>{
        // 构建Query参数
        let mut query = HashMap::new();
        query.insert("name".to_string(),"王五".to_string());
        // 执行异步GET请求
        let response_result = get_async("http://127.0.0.1:13001/example/index",&query).await;
        // 判断结果是否成功
        match response_result {
            Ok(result)=>println!("{}",result),
            Err(err)=> {panic!("{}",err)}
        }
        Ok(())
    }
}
