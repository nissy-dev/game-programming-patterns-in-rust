// see: https://gist.github.com/matthewjberger/082ec2caccc67d68d63794949e350d9b
// see: https://gist.github.com/RyuuGan/44c565cb59cd984d3fcde70010ed9591
// The difference is Box/RC/RefCell: https://doc.rust-jp.rs/book-ja/ch15-05-interior-mutability.html
// this implementation is not thread safe
#![allow(dead_code)]

use std::rc::Rc;

enum Event {
    A,
    B,
}

trait IObserver {
    fn on_notify(&self, event: &Event);
}

struct Observer {}

impl IObserver for Observer {
    fn on_notify(&self, event: &Event) {
        match event {
            Event::A => println!("A!"),
            Event::B => println!("B!"),
        }
    }
}

struct Subject {
    observers: Vec<Rc<dyn IObserver>>,
}

trait ISubject {
    fn add_observer(&mut self, observer: Rc<dyn IObserver>);
    fn remove_observer(&mut self, observer: Rc<dyn IObserver>);
    fn notify(&self, event: &Event);
}

impl Subject {
    fn new() -> Self {
        Subject {
            observers: Vec::new(),
        }
    }
}

impl ISubject for Subject {
    fn add_observer(&mut self, observer: Rc<dyn IObserver>) {
        self.observers.push(observer);
    }

    fn remove_observer(&mut self, observer: Rc<dyn IObserver>) {
        self.observers.retain(|o| !Rc::ptr_eq(o, &observer));
    }

    fn notify(&self, event: &Event) {
        for observer in self.observers.iter() {
            observer.on_notify(event);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn observer_test() {
        let observer = Rc::new(Observer {});
        let mut subject = Subject::new();
        subject.add_observer(observer.clone());
        assert!(subject.observers.len() == 1);
        subject.notify(&Event::A);
        subject.remove_observer(observer.clone());
        assert!(subject.observers.len() == 0);
    }
}
