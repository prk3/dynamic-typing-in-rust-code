use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

pub type Message = serde_json::Value;

pub trait Actor: Sized {
    fn register(actor: &ActorBox<Self>, system: &mut System);
}

pub struct System {
    messages: VecDeque<(&'static str, Message)>,
    handlers: HashMap<&'static str, Box<dyn Fn(Message, &mut System)>>,
}

pub struct ActorBox<A>(Rc<RefCell<A>>);

impl System {
    pub fn new() -> Self {
        Self {
            messages: VecDeque::new(),
            handlers: HashMap::new(),
        }
    }

    pub fn add_actor<A: Actor>(&mut self, actor: A) {
        let actor = ActorBox(Rc::new(RefCell::new(actor)));
        A::register(&actor, self);
    }

    pub fn run(mut self) {
        let handlers = std::mem::take(&mut self.handlers);

        while let Some((key, message)) = self.messages.pop_front() {
            let handle = handlers.get(&key).unwrap();
            handle(message, &mut self);
        }
    }

    pub fn subscribe<A: Actor + 'static>(
        &mut self,
        actor: &ActorBox<A>,
        key: &'static str,
        handler: fn(&mut A, Message, &mut System),
    ) {
        let actor = actor.0.clone();
        self.handlers.insert(
            key,
            Box::new(move |message, system| {
                handler(&mut actor.borrow_mut(), message, system);
            }),
        );
    }

    pub fn publish(&mut self, key: &'static str, message: Message) {
        self.messages.push_back((key, message));
    }
}
