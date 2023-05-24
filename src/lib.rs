//! # toys Crate
//! `toys`  These things work as well as toys  <br>
//! 提供了一些常用的函数库.


// 将string.rs文件所有资源作为一个mod,并且导出
#[cfg(feature = "strings")]
pub mod strings;
pub mod data;
pub mod networks;


// 单元测试
#[cfg(test)]
mod tests {
    use std::any::Any;
    use std::collections::HashMap;
    use super::strings::*;

    #[test]
    fn test_is_blank() {
        assert_eq!(is_blank("".to_string()),true);
        assert_eq!(is_blank(" ".to_string()),false);
    }

    #[test]
    fn test_map(){
        let mut map: HashMap<&str,Box<dyn Any>> = HashMap::new();
        map.insert("name",Box::new("张三".to_string()));
        map.insert("age",Box::new(18));
        let name = map.get("name").unwrap().downcast_ref::<String>().unwrap();
        println!("{:?}",name)
    }

}
