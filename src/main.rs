pub mod closures;
pub mod helpers;
fn main() {
    // println!("Hello, world!");
    // test_func()
    // let full_name = helpers::name_helpers::get_full_name("tian", "xq");
    // println!("full_name is {}", full_name);

    // let new_age = helpers::private_module::get_age_plus_5(25);
    // println!("new_age is {}", new_age)

    closures::test_closures();
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
