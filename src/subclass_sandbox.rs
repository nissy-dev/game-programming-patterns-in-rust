#![allow(dead_code)]

struct SoundPlayer {}

impl SoundPlayer {
    fn new() -> Self {
        Self {}
    }

    fn play_sound(&self, sound_id: i32) {
        println!("Playing sound: {}", sound_id);
    }

    fn stop_sound(&self, sound_id: i32) {
        println!("Stopping sound: {}", sound_id);
    }

    fn set_volume(&self, sound_id: f32) {
        println!("Setting volume: {}", sound_id);
    }
}

trait ISuperPower {
    fn init(&mut self, sound_player: SoundPlayer);
    fn activate(&self);
    fn move_faster(&self);
    fn get_sound_player(&self) -> &SoundPlayer;
}

struct SuperPower {
    sound_player: Option<SoundPlayer>,
}

impl SuperPower {
    fn new() -> Self {
        Self { sound_player: None }
    }

    fn create_sky_launch(sound_player: SoundPlayer) -> SkyLaunch {
        let mut power = SkyLaunch::new();
        power.init(sound_player);
        return power;
    }
}

impl ISuperPower for SuperPower {
    fn init(&mut self, sound_player: SoundPlayer) {
        self.sound_player = Some(sound_player);
    }

    fn activate(&self) {
        todo!("activate is implemented in the subclass");
    }

    fn move_faster(&self) {
        println!("Moving faster");
    }

    fn get_sound_player(&self) -> &SoundPlayer {
        if let Some(value) = &self.sound_player {
            &value
        } else {
            panic!("Sound player is not initialized");
        }
    }
}

struct SkyLaunch {
    super_power: SuperPower,
}

impl SkyLaunch {
    fn new() -> Self {
        Self {
            super_power: SuperPower::new(),
        }
    }
}

impl ISuperPower for SkyLaunch {
    fn init(&mut self, sound_player: SoundPlayer) {
        self.super_power.init(sound_player);
    }

    fn activate(&self) {
        self.super_power.move_faster();
        self.super_power.get_sound_player().play_sound(1);
    }

    fn move_faster(&self) {
        self.super_power.move_faster();
    }

    fn get_sound_player(&self) -> &SoundPlayer {
        self.super_power.get_sound_player()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subclass_sandbox() {
        let sky_launch = SuperPower::create_sky_launch(SoundPlayer::new());
        sky_launch.activate();
    }
}
