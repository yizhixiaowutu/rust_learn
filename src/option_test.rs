use std::fmt::{Display, Formatter, Result};

pub fn test_option_type() -> Option<u8> {
    // let opt1: Option<u8> = None;
    let mut opt = None;
    println!("opt is {:?}", opt);
    opt = Some(5); // Option 类型赋值，需要使用 Some() 函数
    println!("after change, opt is {:?}", opt);
    opt
}

pub fn test_option_string() -> Option<String> {
    let mut opt = None;
    println!("opt is {:?}", opt);
    opt = Some("hello".to_string());
    println!("after change, opt is {:?}", opt);
    opt
}

pub fn test_option_character(c: CharacterType) -> Option<CharacterType> {
    let mut character = None;
    // character = Some(CharacterType::Mega);
    character = Some(c);
    character
}

#[derive(Debug)]
pub enum CharacterType {
    Mega,
    Warrior,
    Archer,
}
