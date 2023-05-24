//! # HTTP请求函数模块

use std::collections::HashMap;
use std::io::ErrorKind;
use std::time::Duration;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;

// 全局静态属性
lazy_static!{
    // HTTP同步客户端
    static ref CLIENT: reqwest::blocking::Client = reqwest::blocking::Client::builder().timeout(Duration::from_secs(3)).build().unwrap();
    // HTTP异步客户端
    static ref CLIENT_ASYNC: reqwest::Client = reqwest::Client::builder().timeout(Duration::from_secs(3)).build().unwrap();
}


/// 发送GET请求(同步)
/// query Query请求参数
/// `R` 表示响应载体类型
/// # Examples
pub fn get<R: DeserializeOwned>(url: &str, query: &HashMap<String,String>) -> reqwest::Result<R> {
    CLIENT.get(url)
        // 设置超时时间为3s
        // 指定Query参数
        .query(&query)
        // 发送请求
        .send()?
        // 获取响应数据
        .json::<R>()
}


/// 发送GET请求(异步)
/// query Query请求参数
/// `R` 表示响应载体类型
pub async fn get_async<R: DeserializeOwned>(url: &str,query: &HashMap<String,String>) -> reqwest::Result<R> {
    CLIENT_ASYNC.get(url)
        // 设置超时时间为3s
        .query(query)
        // 发送请求
        .send().await?
        // 以text格式获取结果
        .json::<R>().await
}

/// 发送Post请求(同步)
/// `T` 表示请求体类型,该类型必须实现`Serialize `trait才能将其序列化为Json
/// `R` 表示响应体载体类型,该类型必须实现了`Deserialize` trait才能将其反序列为结构体
pub fn post<T,R>(url: &str,request_body: &T) -> Result<R,reqwest::Error> where
T: Serialize,
R: DeserializeOwned
{
    Ok(
        CLIENT.post(url)
            .json(request_body)
            .send()?
            .json::<R>()?
    )
}

/// 发送Post请求(异步)
/// `T` 表示请求体类型,该类型必须实现`Serialize `trait才能将其序列化为Json
/// `R` 表示响应体载体类型,该类型必须实现了`Deserialize` trait才能将其反序列为结构体
pub async fn post_async<T,R>(url: &str,request_body: &T) -> Result<R, reqwest::Error> where
    T: Serialize,
    R: DeserializeOwned
{
    let entity = CLIENT_ASYNC.post(url)
        .json(request_body)
        .send().await?
        .json::<R>().await?;
    Ok(entity)
}

/// Http请求体
/// 使用serde的Serialize特征,让其支持结构体序列化为Json
#[derive(Serialize,Debug)]
#[allow(dead_code)]// 避免未使用字段警告
pub struct RequestBody{
    name: String,
    age: u8,
    locked: bool,
    scope: f64
}

/// Http响应体
/// 使用了serde的Deserialize特性，以便将JSON格式的字符串反序列化为结构体对象。
#[derive(Deserialize,Debug)]
#[allow(dead_code)] // 避免未使用字段警告
pub struct ResponseBody{
    name: String,
    age: u8,
    locked: bool,
    scope: f64
}

#[cfg(test)]
mod tests{
    use std::collections::HashMap;
    use crate::networks::http::*;

    /// 单元测试,同步Get请求
    #[test]
    fn test_get(){
        // 构建Query参数
        let mut query = HashMap::new();
        query.insert("name".to_string(),"张三".to_string());
        // 执行同步GET请求
        let result_map :HashMap<String,String> = get("http://127.0.0.1:13001/example/index", &query).unwrap();
        println!("{:?}",result_map)
    }

    /// 异步函数单元测试,异步Get请求
    #[tokio::test]
    async fn test_get_async() -> Result<(),Box<dyn std::error::Error>>{
        // 构建Query参数
        let mut query = HashMap::new();
        query.insert("name".to_string(),"王五".to_string());
        // 执行异步GET请求
        let result_map: HashMap<String,String> = get_async("http://127.0.0.1:13001/example/index",&query).await.unwrap();
        println!("{:?}",result_map);
        Ok(())
    }

    /// 测试同步Post请求
    #[test]
    fn test_post(){
        // 创建请求体
        let request_body = RequestBody{
            name: "满城雪".to_string(),
            age: 23,
            locked: true,
            scope: 188.88,
        };
        // 发起请求,指定返回体为 ResponseBody类型
        let result: ResponseBody = post("http://127.0.0.1:13001/example/index/post",&request_body).unwrap();
        println!("{:?}",result)
    }

    /// 测试异步Post请求
    #[tokio::test]
    async fn test_post_async() -> Result<(),std::io::Error>{
        // 创建Post请求体
        let request_body = RequestBody{
            name: "满城雪".to_string(),
            age: 23,
            locked: true,
            scope: 188.88,
        };

        // 发起请求,指定返回映射体的类型
        let result:Result<ResponseBody,reqwest::Error> = post_async("http://127.0.0.1:13001/example/index/post",&request_body).await;
        // 处理结果
        match result {
            Ok(value)=> {
                println!("{:?}",value);
                Ok(())
            },
            Err(error)=> {
                println!("{}",error.to_string());
                Err(std::io::Error::new(ErrorKind::Other,"发送Post请求失败"))
            }
        }
    }
}
