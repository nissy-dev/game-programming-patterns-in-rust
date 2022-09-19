#![allow(dead_code)]

trait IAudio {
    fn play_sound(&self, sound_id: i32);
    fn stop_sound(&self, sound_id: i32);
}

struct ConsoleAudio {}

impl ConsoleAudio {
    fn new() -> Self {
        Self {}
    }
}

impl IAudio for ConsoleAudio {
    fn play_sound(&self, sound_id: i32) {
        println!("Playing console sound {}", sound_id);
    }

    fn stop_sound(&self, sound_id: i32) {
        println!("Stopping console sound {}", sound_id);
    }
}

struct Locator {
    audio: Option<Box<dyn IAudio>>,
}

impl Locator {
    fn new() -> Self {
        Self { audio: None }
    }

    fn provide(&mut self, audio: Box<dyn IAudio>) {
        self.audio = Some(audio);
    }

    fn get_audio(&self) -> &Box<dyn IAudio> {
        self.audio.as_ref().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn service_locator() {
        let mut locator = Locator::new();
        locator.provide(Box::new(ConsoleAudio::new()));
        locator.get_audio().play_sound(1);
    }
}
