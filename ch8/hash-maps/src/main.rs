use std::collections::HashMap;

fn main() {
    let mut hm = HashMap::new();

    hm.insert(String::from("Red"), 15);
    hm.insert(String::from("Blue"), 20);

    let val = hm.get("Red");

    match val {
        Some(v) => {
            // *v += 1;
            println!("Value: {}", v)
        },
        None => println!("No value"),
    }

    println!("HashMap: {:?}", hm);


    let mut my_num = 10;
    let _my_num_ref = &mut my_num;
    *_my_num_ref += 1;

    let _my_num_ref = &mut &mut my_num;
    **_my_num_ref += 1;


    let _my_num_ref2 = &mut & my_num;
    // **_my_num_ref2 += 1;

    println!("my_num: {}", my_num);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map
            .entry(String::from(word))
            .or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
