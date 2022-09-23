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
    next: Option<Box<Particle>>,
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

    fn get_next(&mut self) -> Option<Box<Particle>> {
        self.next.take()
    }

    fn set_next(&mut self, particle: Box<Particle>) {
        self.next = Some(particle);
    }
}

struct ParticlePool {
    head: Option<Box<Particle>>,
}

impl ParticlePool {
    fn new() -> Self {
        let mut particle = Box::new(Particle::new());
        for _ in 0..PARTICLE_SIZE - 1 {
            let mut new_particle = Box::new(Particle::new());
            new_particle.set_next(particle);
            particle = new_particle;
        }

        ParticlePool {
            head: Some(particle),
        }
    }

    fn animate(&mut self) {
        while let Some(particle) = &mut self.head {
            particle.animate();
            self.head = particle.get_next();
        }
    }

    fn create(&mut self, x: u32, y: u32, x_vel: u32, y_vel: u32, lifetime: u32) {
        if let Some(particle) = &mut self.head {
            self.head = particle.get_next();
            if let Some(particle) = &mut self.head {
                particle.init(x, y, x_vel, y_vel, lifetime);
            }
        }
    }
}
