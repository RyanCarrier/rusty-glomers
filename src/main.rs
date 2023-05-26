mod message;

fn main() {
   loop {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        eprintln!("{}", line);
        let msg: message::MaelstromMessage = serde_json::from_str(&line).unwrap();

        match msg.handle() {
             Ok(r) => println!("{}", serde_json::to_string(&r).unwrap()),
            Err(e) => eprintln!("{}", e),
        }
    }
}
