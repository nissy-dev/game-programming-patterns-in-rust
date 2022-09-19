#![allow(dead_code)]

const PIXEL_WIDTH: usize = 3;
const PIXEL_HEIGHT: usize = 4;

#[derive(Debug, Clone, Copy)]
struct FrameBuffer {
    pixels: [u32; (PIXEL_WIDTH * PIXEL_HEIGHT)],
}

impl FrameBuffer {
    fn new() -> Self {
        Self {
            pixels: [0; (PIXEL_WIDTH * PIXEL_HEIGHT)],
        }
    }

    fn clear(&mut self) {
        for pixel in self.pixels.iter_mut() {
            *pixel = 0;
        }
    }

    fn draw(&mut self, x: usize, y: usize) {
        self.pixels[y * PIXEL_WIDTH + x] = 1;
    }

    fn get_pixels(&self) -> [u32; (PIXEL_WIDTH * PIXEL_HEIGHT)] {
        self.pixels
    }
}

struct Scene {
    current_buffer: FrameBuffer,
    next_buffer: FrameBuffer,
}

impl Scene {
    fn new() -> Self {
        Self {
            current_buffer: FrameBuffer::new(),
            next_buffer: FrameBuffer::new(),
        }
    }

    fn draw(&mut self) {
        self.next_buffer.clear();
        self.next_buffer.draw(1, 1);
        self.next_buffer.draw(2, 3);
        self.swap_buffers();
    }

    fn swap_buffers(&mut self) {
        std::mem::swap(&mut self.next_buffer, &mut self.current_buffer);
    }

    fn get_buffer(&self) -> &FrameBuffer {
        &self.current_buffer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn double_buffer() {
        let mut scene = Scene::new();
        scene.draw();
        assert_eq!(
            scene.get_buffer().get_pixels(),
            [0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1]
        );
    }
}
