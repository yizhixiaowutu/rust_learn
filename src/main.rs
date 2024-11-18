pub mod closures;
pub mod helpers;
pub mod match_test;
pub mod option_test;
fn main() {
    // println!("Hello, world!");
    // test_func()
    // let full_name = helpers::name_helpers::get_full_name("tian", "xq");
    // println!("full_name is {}", full_name);

    // let new_age = helpers::private_module::get_age_plus_5(25);
    // println!("new_age is {}", new_age)

    // closures::test_closures();

    // match_test::test_match_init();
    // match_test::test_match_string();
    // match_test::test_match_array();

    // let r1 = option_test::test_option_type();
    // println!("r1 is {:?}", r1.unwrap());
    let r2 = option_test::test_option_string();
    println!("r2 is {:?}", r2.unwrap());
    let r3 = option_test::test_option_character(option_test::CharacterType::Warrior);
    println!("r3 is {:?}", r3.unwrap());
}

#[allow(dead_code)]
fn test_func() {
    let x: i32 = 5;
    // let u: () = ();
    println!("x is {}", x);
    let my_str = '🔥';
    println!("my_str is {}", my_str);
    // println!("u is {}", u);
}
