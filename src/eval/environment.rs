use crate::object::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub type Env = Rc<RefCell<Environment>>;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Environment {
    store: HashMap<String, Rc<Object>>,
    outer: Option<Env>,
}

impl Environment {
    pub fn new_enclosed(outer: &Env) -> Self {
        let mut env: Environment = Default::default();
        env.outer = Some(Rc::clone(outer));
        env
    }

    pub fn get(&self, name: &str) -> Option<Rc<Object>> {
        match self.store.get(name) {
            Some(obj) => Some(Rc::clone(obj)),
            None => match &self.outer {
                Some(outer) => outer.borrow().get(name),
                None => None,
            },
        }
    }

    pub fn set(&mut self, name: &str, obj: Rc<Object>) {
        self.store.insert(name.to_string(), obj);
    }
}
