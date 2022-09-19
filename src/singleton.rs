#![allow(dead_code)]

use once_cell::sync::OnceCell;
use std::sync::Mutex;

#[derive(Debug, Clone)]
struct Config {
    data: String,
}

impl Config {
    pub fn get_instance() -> &'static Mutex<Config> {
        static INSTANCE: OnceCell<Mutex<Config>> = OnceCell::new();
        INSTANCE.get_or_init(|| {
            println!("Initialize new data!");
            Mutex::new(Config {
                data: "hello world from config!".to_string(),
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn singleton() {
        let f1 = Config::get_instance();
        println!("f1: {:?}", f1);

        let f2 = Config::get_instance();
        println!("f2: {:?}", f2);

        let conf2 = f2.lock().unwrap();
        assert_eq!(conf2.data, "hello world from config!".to_string())
    }
}
