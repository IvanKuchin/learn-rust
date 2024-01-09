#[derive(Debug)]
struct Data {
    id: String,
    opt: Option<String>,
    checks: Vec<String>,
}

fn reference(data: &mut Data) {
    let _a = data.opt.take();
}


fn main() {
    let mut data = Data {
        id: "123".to_string(),
        opt: Some("123".to_string()),
        checks: vec![
            "a".to_string(),
            "b".to_string(),
        ],
    };

    println!("before: {:?}", data);
    
    reference(&mut data);

    println!("after: {:?}", data);

}
