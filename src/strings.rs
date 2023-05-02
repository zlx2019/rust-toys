//! # 字符串类型相关处理模块

/// 判断字符串是否为空
/// ## Examples
/// ```
///use toys::strings::is_blank;
///assert_eq!(is_blank("".to_string()),true);
/// ```
pub fn is_blank(s: String) -> bool {
    s.is_empty()
}

/// 判断字符串是否非空
/// ## Examples
///```
///use toys::strings::is_not_blank;
///assert_eq!(is_not_blank(String::from("123")),true)
///```

pub fn is_not_blank(s: String) -> bool{
    !s.is_empty()
}