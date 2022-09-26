#![allow(dead_code)]

#[derive(Debug, Copy, Clone)]
struct Particle_ {
    frame_left: u32,
    x: u32,
    y: u32,
    x_vel: u32,
    y_vel: u32,
}

impl Particle_ {
    fn new() -> Self {
        Particle_ {
            frame_left: 0,
            x: 0,
            y: 0,
            x_vel: 0,
            y_vel: 0,
        }
    }

    fn init(&mut self, x: u32, y: u32, x_vel: u32, y_vel: u32, lifetime: u32) {
        self.frame_left = lifetime;
        self.x = x;
        self.y = y;
        self.x_vel = x_vel;
        self.y_vel = y_vel;
    }

    fn animate(&mut self) {
        if !self.is_use() {
            return;
        }

        self.x += self.x_vel;
        self.y += self.y_vel;
        self.frame_left -= 1;
    }

    fn is_use(&self) -> bool {
        self.frame_left > 0
    }
}

const PARTICLE_SIZE: usize = 10;

struct ParticlePool_ {
    particles: [Particle_; PARTICLE_SIZE],
}

impl ParticlePool_ {
    fn new() -> Self {
        ParticlePool_ {
            particles: [Particle_::new(); PARTICLE_SIZE],
        }
    }

    fn animate(&mut self) {
        for particle in self.particles.iter_mut() {
            particle.animate();
        }
    }

    fn create(&mut self, x: u32, y: u32, x_vel: u32, y_vel: u32, lifetime: u32) {
        for particle in self.particles.iter_mut() {
            if !particle.is_use() {
                particle.init(x, y, x_vel, y_vel, lifetime);
                return;
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Particle {
    frame_left: u32,
    x: u32,
    y: u32,
    x_vel: u32,
    y_vel: u32,
    next: Option<usize>,
}

impl Particle {
    fn new() -> Self {
        Particle {
            frame_left: 0,
            x: 0,
            y: 0,
            x_vel: 0,
            y_vel: 0,
            next: None,
        }
    }

    fn init(&mut self, x: u32, y: u32, x_vel: u32, y_vel: u32, lifetime: u32) {
        self.frame_left = lifetime;
        self.x = x;
        self.y = y;
        self.x_vel = x_vel;
        self.y_vel = y_vel;
    }

    fn animate(&mut self) -> bool {
        if !self.is_use() {
            return false;
        }

        self.x += self.x_vel;
        self.y += self.y_vel;
        self.frame_left -= 1;
        return self.frame_left == 0;
    }

    fn is_use(&self) -> bool {
        self.frame_left > 0
    }

    fn get_next_id(&mut self) -> Option<usize> {
        self.next
    }

    fn set_next_id(&mut self, id: usize) {
        self.next = Some(id);
    }
}

#[derive(Debug, Clone)]
struct ParticlePool {
    head_id: Option<usize>,
    particles: Vec<Particle>,
}

impl ParticlePool {
    fn new() -> Self {
        let mut particles = Vec::with_capacity(PARTICLE_SIZE);
        for _ in 0..PARTICLE_SIZE {
            particles.push(Particle::new());
        }

        for i in 0..PARTICLE_SIZE - 1 {
            particles[i].set_next_id(i + 1);
        }

        ParticlePool {
            head_id: Some(0),
            particles,
        }
    }

    fn animate(&mut self) {
        for i in 0..PARTICLE_SIZE {
            if self.particles[i].animate() {
                self.particles[i].set_next_id(self.head_id.unwrap());
                self.head_id = Some(i);
            }
        }
    }

    fn create(&mut self, x: u32, y: u32, x_vel: u32, y_vel: u32, lifetime: u32) {
        if let Some(id) = self.head_id {
            self.head_id = self.particles[id].get_next_id();
            if let Some(new_id) = self.head_id {
                self.particles[new_id].init(x, y, x_vel, y_vel, lifetime);
            }
        }
    }

    fn get_particle(&self) -> &Particle {
        &self.particles[self.head_id.unwrap()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn object_pool() {
        let mut pool = ParticlePool::new();
        pool.create(0, 0, 1, 1, 10);
        assert_eq!(pool.get_particle().frame_left, 10);
        pool.animate();
        assert_eq!(pool.get_particle().frame_left, 9);
    }
}
