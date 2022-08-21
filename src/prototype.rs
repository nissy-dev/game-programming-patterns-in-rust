trait IPrototype: Clone {
    fn new(value: i32) -> Self;
    fn print_value(&self);
}

#[derive(Debug, Clone)]
struct ConcretePrototype {
    value: i32,
}

impl IPrototype for ConcretePrototype {
    fn new(value: i32) -> Self {
        ConcretePrototype { value: value }
    }

    fn print_value(&self) {
        println!("Print value: {}", self.value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prototype_test() {
        let prototype = ConcretePrototype::new(10);
        println!("original: {:?}", prototype);
        let _cloned = prototype.clone();
        println!("cloned: {:?}", prototype);
    }
}
