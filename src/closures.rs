struct Person {
    first_name: String,
    last_name: String,
}
pub fn test_closures() {
    let add = |x, y| println!("Add result is {}", x + y);
    add(3, -8);

    let mut person = Person {
        first_name: String::from("tian"),
        last_name: String::from("xq"),
    };
    /*
     闭包捕获的变量是通过引用捕获的，而引用的可变性需要通过闭包的可变性来传递
     所以当闭包需要修改捕获的变量时，闭包本身也需要是可变的
    */
    let mut change_last_name = |new_last_name| person.last_name = new_last_name;
    change_last_name(String::from("xq2"));
    change_last_name(String::from("xq3"));
    println!(
        "after change_last_name, last_name is {:?}",
        person.last_name
    );
}
