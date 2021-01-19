pub struct Renderer {
    pub image_width: u32,
    pub image_height: u32,
    pub buffer: Vec<u8>
}

impl Renderer {
    pub fn new(image_width: u32, image_height: u32) -> Self {
        Self {
            image_width: image_width,
            image_height: image_height,
            buffer: vec![0; (3 * image_width * image_height) as usize]
        }
    }

    pub fn render<'a, T: FnMut(f64) + 'a>(&mut self, path: &str, progress: T) {
        let mut clojure = progress;

        for x in 0..self.image_width {
            (&mut clojure)(x as f64 / self.image_width as f64);

            for y in 0..self.image_height {
                let index = (3 * (y * self.image_width + x)) as usize;
                let u = x as f64 / self.image_width as f64;
                let v = 1.0 - (y as f64 / self.image_height as f64);
                let red_f = [u, v, 0.25];
                
                for i in 0..3 {
                    self.buffer[index + i] = (255.999 * red_f[i]) as u8;
                }
            }
        }

        image::save_buffer(path, &self.buffer, self.image_width, self.image_height, image::ColorType::Rgb8).unwrap();

        (&mut clojure)(1.0);
    }
}