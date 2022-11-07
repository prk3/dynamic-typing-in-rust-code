use dynamic_programing_in_rust_code::typed::*;

struct InitMessage;

struct PingMessage {
    ping_number: u64,
}

struct PongMessage {
    pong_number: u64,
}

struct PingActor {
    pings_to_send: u64,
}

impl Actor for PingActor {
    fn register(system: &mut System) {
        system.subscribe::<Self, InitMessage>();
        system.subscribe::<Self, PongMessage>();
        system.publish(InitMessage);
    }
}

impl Handler<InitMessage> for PingActor {
    fn handle(&mut self, _message: InitMessage, system: &mut System) {
        println!("Ping actor received init message, sending ping 1");
        system.publish(PingMessage { ping_number: 1 });
        self.pings_to_send -= 1;
    }
}

impl Handler<PongMessage> for PingActor {
    fn handle(&mut self, message: PongMessage, system: &mut System) {
        if self.pings_to_send > 0 {
            println!(
                "Ping actor received pong {}, sending ping {}",
                message.pong_number,
                message.pong_number + 1,
            );
            system.publish(PingMessage {
                ping_number: message.pong_number + 1,
            });
            self.pings_to_send -= 1;
        } else {
            println!("Ping actor has no more pings to send, exiting");
        }
    }
}

struct PongActor {}

impl Actor for PongActor {
    fn register(system: &mut System) {
        system.subscribe::<Self, PingMessage>();
    }
}

impl Handler<PingMessage> for PongActor {
    fn handle(&mut self, message: PingMessage, system: &mut System) {
        system.publish(PongMessage {
            pong_number: message.ping_number,
        });
    }
}

fn main() {
    let mut system = System::new();
    system.add_actor(PingActor { pings_to_send: 10 });
    system.add_actor(PongActor {});
    system.run();
}
