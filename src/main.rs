use crate::message::{MaelstromMessage, MessageType};

mod message;
mod state;
fn main() {
    let mut state = state::State::new();
    loop {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let msg: MaelstromMessage = serde_json::from_str(&line).unwrap();
        if msg.body.msg_type == MessageType::Broadcast
            && !state.seen_messages.contains(&msg.body.message.unwrap())
        {
            MaelstromMessage::broadcast(&state, msg.body.message.clone().unwrap())
                .into_iter()
                .for_each(post);
        }
        match msg.handle(&mut state) {
            Ok(r) => post(r),
            Err(_) => {}
        }
    }
}

fn post(msg: MaelstromMessage) {
    println!("{}", serde_json::to_string(&msg).unwrap());
}
