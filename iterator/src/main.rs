struct Servers(String);

struct Rack {
    servers: <Vec<Servers> as IntoIterator>::IntoIter,
    hight_in_ru: u32,
}

impl Rack {
    fn new(servers: Vec<Servers>, hight_in_ru: u32) -> Self {
        Self {
            servers: servers.into_iter(),
            hight_in_ru,
        }
    }

    fn try_next(&mut self) -> Result<Servers, ()> {
        // if let Some(s) = self.servers.next() {
        //     return Ok(s)
        // }
        // Err(())
        self.servers.next().ok_or(())
    }
}

impl Iterator for Rack {
    type Item = Servers;

    fn next(&mut self) -> Option<Self::Item> {
        match self.try_next() {
            Ok(s) => Some(s),
            Err(_) => None,
        }
    }
}

fn main() {
    let rack = Rack::new(
        vec![Servers("infra".to_string()), Servers("fillmore".to_string()), Servers("M3".to_string()), Servers("captain".to_string()) ], 
        42
    );

    for server in rack {
        println!("server {}", server.0);
    }

    let mut s = String::from("hello world");
    let word = String::from(first_word(&s));
    s.clear(); // error!
    println!("first word is {}", word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, item) in bytes.iter().enumerate() { // iter() returns each element in a collection
        if *item == b' ' {
            return &s[0..i]; // return a slice
        }
    }
    &s[..]
}
