//! # Json数据格式处理函数库

use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;

/// 将一个结构体对象序列化为Json字符串
/// T 必须实现了Serialize Deserialize 两个特征
/// # Examples
/// ```
/// use toys::data::json::{Student, to_json_str};
/// let mut stu = Student::default();
/// let json: String = to_json_str(&stu).unwrap();
/// println!("{}",json)
/// ```
pub fn to_json_str<'a,T: Serialize + Deserialize<'a>>(value: &T) -> Result<String,std::io::Error>
{
    Ok(serde_json::to_string(value)?)
}

/// 将一个Json字符串反序列化为一个结构体对象
/// # Examples
/// ```
/// use toys::data::json::{from_json_str, Student};
/// let stu: Student = from_json_str(r#"{"name":"满城雪","age":23,"address":"广州","locked":true,"sex":"男"}"#).unwrap();
/// println!("{:?}",stu)
/// ```
pub fn from_json_str<T: DeserializeOwned>(json: &str) -> Result<T, serde_json::Error> {
    let value: T = serde_json::from_str(json)?;
    Ok(value)
}

/// 将一个结构体对象序列化为Json字节集合
/// # Examples
/// ```
/// use toys::data::json::{from_json_bytes, Student, to_json_bytes};
/// let stu = Student::default();
/// let bytes = to_json_bytes(&stu).unwrap();
/// let ass_val = vec![123, 34, 110, 97, 109, 101, 34, 58, 34, 230, 187, 161, 229, 159, 142, 233, 155, 170, 34, 44, 34, 97, 103, 101, 34, 58, 50, 51, 44, 34, 97, 100, 100, 114, 101, 115, 115, 34, 58, 34, 229, 185, 191, 229, 183, 158, 34, 44, 34, 108, 111, 99, 107, 101, 100, 34, 58, 116, 114, 117, 101, 44, 34, 115, 101, 120, 34, 58, 34, 231, 148, 183, 34, 125];
/// assert_eq!(bytes,ass_val)
/// ```
pub fn to_json_bytes<'a,T>(value: &T) -> Result<Vec<u8>,std::io::Error>
    where T: Serialize + Deserialize<'a>
{
    Ok(serde_json::to_vec(value)?)
}

/// 将一个Json字节集合,反序列化为一个结构体对象
/// # Examples
/// ```
/// use toys::data::json::{from_json_bytes, Student};
/// let json_bytes = vec![123, 34, 110, 97, 109, 101, 34, 58, 34, 230, 187, 161, 229, 159, 142, 233, 155, 170, 34, 44, 34, 97, 103, 101, 34, 58, 50, 51, 44, 34, 97, 100, 100, 114, 101, 115, 115, 34, 58, 34, 229, 185, 191, 229, 183, 158, 34, 44, 34, 108, 111, 99, 107, 101, 100, 34, 58, 116, 114, 117, 101, 44, 34, 115, 101, 120, 34, 58, 34, 231, 148, 183, 34, 125];
/// let student: Student = from_json_bytes(&json_bytes).unwrap();
/// assert_eq!(student,Student::default())
/// ```
pub fn from_json_bytes<T: DeserializeOwned>(bytes: &Vec<u8>) -> Result<T,serde_json::Error>{
    Ok(serde_json::from_slice(bytes)?)
}



/// 测试数据实体
#[derive(Deserialize,Serialize,Debug,PartialEq)]
pub struct Student{
    name: String,
    age: u8,
    address: String,
    locked: bool,
    sex: char,
}

impl Student {
    // Student 构造方法
    fn new(name: String,age: u8,address: String,locked: bool,sex: char)-> Self{
        Student{
            name,
            age,
            address,
            locked,
            sex,
        }
    }
}
// Default默认实现块
impl Default for Student {
    fn default() -> Self {
        Student::new("满城雪".into(),23,"广州".into(),true,'男')
    }
}


/// 单元测试
#[cfg(test)]
mod tests{
    use crate::data::json::{from_json_bytes, from_json_str, Student, to_json_bytes, to_json_str};

    /// 测试:
    /// struct Convert Json_string
    /// Json_string Convert struct
    #[test]
    fn test_json_str_convert(){
        // 将对象 To Json
        let student = Student::default();
        let json_result = to_json_str(&student);
        // 正确的Json字符串值
        let json_val = r#"{"name":"满城雪","age":23,"address":"广州","locked":true,"sex":"男"}"#;

        // Assert
        assert_eq!(json_result.unwrap(),json_val);

        // 将 Json To 对象
        let new_student: Student = from_json_str(json_val).unwrap();

        // Assert
        assert_eq!(student,new_student);
    }

    /// 测试:
    /// struct to json bytes
    /// json_bytes to struct
    #[test]
    fn test_json_bytes_convert(){
        let  stu = Student::default();
        let bytes = to_json_bytes(&stu).unwrap();
        println!("{:?}",bytes);
        let new_stu: Student = from_json_bytes(&bytes).unwrap();
        assert_eq!(stu,new_stu);
    }
}