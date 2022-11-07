use std::any::{Any, TypeId};
use std::collections::{HashMap, VecDeque};

pub trait Actor {
    fn register(system: &mut System);
}

pub trait Handler<M: 'static> {
    fn handle(&mut self, message: M, system: &mut System);
}

pub struct System {
    messages: VecDeque<Box<dyn Any>>,
    actors: HashMap<TypeId, Box<dyn Any>>,
    handlers: HashMap<TypeId, (TypeId, fn(&mut dyn Any, Box<dyn Any>, &mut System))>,
}

impl System {
    pub fn new() -> Self {
        Self {
            messages: VecDeque::new(),
            actors: HashMap::new(),
            handlers: HashMap::new(),
        }
    }

    pub fn add_actor<A: Actor + 'static>(&mut self, actor: A) {
        self.actors.insert(TypeId::of::<A>(), Box::new(actor));
        A::register(self);
    }

    pub fn run(mut self) {
        let mut actors = std::mem::take(&mut self.actors);
        let handlers = std::mem::take(&mut self.handlers);

        while let Some(message) = self.messages.pop_front() {
            let (actor_type, handle) = handlers.get(&message.as_ref().type_id()).unwrap();
            let actor = actors.get_mut(actor_type).unwrap();
            handle(actor.as_mut(), message, &mut self);
        }
    }

    pub fn subscribe<A, M>(&mut self)
    where
        A: Actor + Handler<M> + 'static,
        M: 'static,
    {
        self.handlers
            .insert(TypeId::of::<M>(), (TypeId::of::<A>(), handle_any::<A, M>));
    }

    pub fn publish<M: 'static>(&mut self, message: M) {
        self.messages.push_back(Box::new(message));
    }
}

fn handle_any<A, M>(actor: &mut dyn Any, message: Box<dyn Any>, system: &mut System)
where
    A: Handler<M> + 'static,
    M: 'static,
{
    let actor = actor.downcast_mut::<A>().unwrap();
    let message = *message.downcast::<M>().unwrap();
    actor.handle(message, system);
}
