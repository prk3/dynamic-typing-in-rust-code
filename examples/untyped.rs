use dynamic_programing_in_rust_code::untyped::*;

use serde_json::json;

struct PingActor {
    pings_to_send: u64,
}

impl Actor for PingActor {
    fn register(actor: &ActorBox<Self>, system: &mut System) {
        system.subscribe(actor, "init", Self::handle_init);
        system.subscribe(actor, "pong", Self::handle_pong);
        system.publish("init", json!({}));
    }
}

impl PingActor {
    fn handle_init(&mut self, _message: Message, system: &mut System) {
        println!("Ping actor received init message, sending ping 1");
        system.publish("ping", json!({ "ping_number": 1 }));
        self.pings_to_send -= 1;
    }
}

impl PingActor {
    fn handle_pong(&mut self, message: Message, system: &mut System) {
        let pong_number = message.get("pong_number").unwrap().as_u64().unwrap();

        if self.pings_to_send > 0 {
            println!(
                "Ping actor received pong {}, sending ping {}",
                pong_number,
                pong_number + 1,
            );
            system.publish("ping", json!({ "ping_number": pong_number + 1 }));
            self.pings_to_send -= 1;
        } else {
            println!("Ping actor has no more pings to send, exiting");
        }
    }
}

struct PongActor {}

impl Actor for PongActor {
    fn register(actor: &ActorBox<Self>, system: &mut System) {
        system.subscribe(actor, "ping", Self::handle_ping);
    }
}

impl PongActor {
    fn handle_ping(&mut self, message: Message, system: &mut System) {
        let ping_number = message.get("ping_number").unwrap().as_u64().unwrap();
        system.publish("pong", json!({ "pong_number": ping_number }));
    }
}

fn main() {
    let mut system = System::new();
    system.add_actor(PingActor { pings_to_send: 10 });
    system.add_actor(PongActor {});
    system.run();
}
