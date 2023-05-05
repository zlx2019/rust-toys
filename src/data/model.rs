use std::fs::{File, OpenOptions};
use std::io::{ Read, Write};
use serde::{Deserialize, Serialize};

/// 性别枚举
#[derive(Serialize,Deserialize,Debug,PartialEq)]
pub(crate) enum Gender{
    Boy = 0,
    Girl = 1
}

/// 定义一个数据结构体,用于测试
#[derive(Serialize,Deserialize,Debug,PartialEq)]
pub(crate) struct User{
    pub name: String,
    pub age: u8,
    pub gender: Gender
}
// User方法块
impl User {
    // 构造方法
    pub fn new(name: String,age: u8,gender: Gender)-> Self{
        Self{ name, age, gender, }
    }

    /// 以json格式,将实例数据持久化到本地文件。
    pub fn write_file_json(&self,file_name: &str) -> Result<usize,std::io::Error>{
        // 打开文件,如果文件不存在则创建,并且为追加写入模式.如果失败则直接返回Result的Err
        let mut file = OpenOptions::new().append(true).create(true).open(file_name)?;
        // 将结构体序列化为Json字符串,如果失败则直接返回Result的Err
        let json_str = serde_json::to_string(self)?;
        // 将Json字符串写入文件,如果失败则直接返回Result的Err
        file.write_all(format!("{}\n",json_str).as_bytes())?;
        // 返回写入的内容字节长度
        Ok(json_str.len())
        // Err(std::io::Error::new(ErrorKind::Other,"模拟一个错误~"))
    }

    /// 读取本地json文件,反序列化为一个对象
    pub fn load_json_file(file_name: &str) -> Result<Self,std::io::Error>{
        // 打开文件
        let mut file = File::open(file_name)?;
        // 创建一个String对象
        let mut buf = String::new();
        // 将文件内容读取到buf中
        file.read_to_string(&mut buf)?;
        // 将json字符串,反序列化为User对象
        let user: User = serde_json::from_str(&buf)?;
        Ok(user)
    }
}

// 实现User Default默认实现块
impl Default for User {
    // 提供default()默认实例关联函数
    fn default() -> Self {
        User::new("Unknown User".into(),0,Gender::Boy)
    }
}


#[cfg(test)]
mod tests{
    use crate::data::model::User;

    /// 单元测试: 将User数据持久化到本地文件
    /// 再读取本地文件,反序列化为User
    #[test]
    fn test_write_file_json(){
        let user = User::default();
        _ = user.write_file_json("user.json").unwrap();
        println!("{:?}",user);

        let load_user = User::load_json_file("user.json").unwrap();
        assert_eq!(user,load_user);
        println!("{:?}",load_user);
    }
}


