#![allow(dead_code)]

const MAX_PENDING: usize = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PlayMessage {
    sound_id: i32,
    volume: i32,
}

#[derive(Debug)]
struct Audio {
    pending_: [Option<PlayMessage>; MAX_PENDING + 1],
    pending_size_: usize,
    head_: usize,
    tail_: usize,
}

impl Audio {
    fn new() -> Self {
        Self {
            pending_: [None; MAX_PENDING + 1],
            pending_size_: MAX_PENDING + 1,
            head_: 0,
            tail_: 0,
        }
    }

    fn play_sound(&mut self, sound_id: i32, volume: i32) {
        if (self.tail_ + 1) % self.pending_size_ == self.head_ {
            println!("Audio queue is full!");
            return;
        }

        self.pending_[self.tail_] = Some(PlayMessage {
            sound_id: sound_id,
            volume: volume,
        });
        self.tail_ = (self.tail_ + 1) % self.pending_size_;
    }

    fn update(&mut self) {
        if self.head_ == self.tail_ {
            println!("Audio queue is empty!");
            return;
        }

        if let Some(msg) = self.pending_[self.head_] {
            println!("Playing sound {} at volume {}", msg.sound_id, msg.volume);
            self.pending_[self.head_] = None;
            self.head_ = (self.head_ + 1) % self.pending_size_;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_queue() {
        let mut audio = Audio::new();
        audio.play_sound(1, 10);
        audio.play_sound(3, 100);
        audio.play_sound(2, 5);
        // show Audio queue is full!
        audio.play_sound(4, 30);

        audio.update();
        audio.update();
        audio.update();
        // show Audio queue is empty!
        audio.update();

        audio.play_sound(4, 30);
        println!("{:?}", audio);
    }
}
