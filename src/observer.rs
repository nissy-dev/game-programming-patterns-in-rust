// see: https://gist.github.com/matthewjberger/082ec2caccc67d68d63794949e350d9b
#![allow(dead_code)]

enum Event {
    A,
    B,
}

trait IObserver: PartialEq {
    fn on_notify(&self, event: &Event);
}

#[derive(PartialEq)]
struct Observer {}

impl IObserver for Observer {
    fn on_notify(&self, event: &Event) {
        match event {
            Event::A => println!("A!"),
            Event::B => println!("B!"),
        }
    }
}

struct Subject<'a, T: IObserver> {
    observers: Vec<&'a T>,
}

trait ISubject<'a, T: IObserver> {
    fn add_observer(&mut self, observer: &'a T);
    fn remove_observer(&mut self, observer: &'a T);
    fn notify(&self, event: &Event);
}

impl<'a, T: IObserver> Subject<'a, T> {
    fn new() -> Self {
        Subject {
            observers: Vec::new(),
        }
    }
}

impl<'a, T: IObserver> ISubject<'a, T> for Subject<'a, T> {
    fn add_observer(&mut self, observer: &'a T) {
        self.observers.push(observer);
    }

    fn remove_observer(&mut self, observer: &'a T) {
        self.observers.retain(|o| *o != observer);
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
        let observer = Observer {};
        let mut subject = Subject::new();
        subject.add_observer(&observer);
        subject.notify(&Event::A);
    }
}
