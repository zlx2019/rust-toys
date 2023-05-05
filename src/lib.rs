//! # toys Crate
//! `toys`  These things work as well as toys  <br>
//! 提供了一些常用的函数库.


// 将string.rs文件所有资源作为一个mod,并且导出
#[cfg(feature = "strings")]
pub mod strings;
#[cfg(feature = "http")]
pub mod http;
pub mod data;



// 单元测试
#[cfg(test)]
mod tests {
    use super::strings::*;

    #[test]
    fn test_is_blank() {
        assert_eq!(is_blank("".to_string()),true);
        assert_eq!(is_blank(" ".to_string()),false);
    }


}
