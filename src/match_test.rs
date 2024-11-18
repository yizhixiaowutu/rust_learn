pub fn test_match_init() {
    let my_age: u8 = 25;

    let y = 5;

    match my_age {
        0 | 5 => {
            println!("my_age is 0 or 5, my_age is {}", my_age);
        }

        1..=35 if y == 5 => {
            println!(
                "my_age is between 1 and 35 and y is 5, my_age is {}",
                my_age
            );
        }

        1..=35 if y != 5 => {
            println!(
                "my_age is between 1 and 35 and y is not 5, my_age is {}",
                my_age
            );
        }

        // 1..=35 => {
        //     println!("my_age is between 1 and 35, my_age is {}", my_age);
        // }
        36..=50 => {
            println!("my_age is between 36 and 50, my_age is {}", my_age);
        }

        _ => {
            println!("my_age is something else");
        }
    }
}

pub fn test_match_string() {
    let people_sex = "unknown";

    match people_sex {
        "male" => println!("people sex is male"),

        "female" => println!("people sex is female"),

        _ => println!("may be an alien ?!"),
    }
}

pub fn test_match_array() {
    // let arr = [1, 2, 3, 4, 5];
    let arr = [1, 1, 4, 5];

    match arr[0..=2] {
        [1, 2] => println!("1 & 2"),
        [1, 2, ..] => println!("1 & 2 & _"),
        _ => println!("not 1 & 2"),
    }
}
