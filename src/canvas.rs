// canvas.rs
use crate::color::Color;

#[derive(Debug, Clone)]
pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![Color::new(0.0, 0.0, 0.0); width * height],
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn pixels(&self) -> &[Color] {
        &self.pixels
    }

    fn pixel_at_mut(&mut self, x: usize, y: usize) -> &mut Color {
        &mut self.pixels[y * self.width + x]
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        *self.pixel_at_mut(x, y) = color;
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Color {
        self.pixels[y * self.width + x]
    }

    fn ppm_header(&self) -> String {
        format!("P3\n{} {}\n255\n", self.width, self.height)
    }

    fn color_to_ppm(color: Color) -> String {
        let r = (color.red() * 255.0).round().clamp(0.0, 255.0) as u8;
        let g = (color.green() * 255.0).round().clamp(0.0, 255.0) as u8;
        let b = (color.blue() * 255.0).round().clamp(0.0, 255.0) as u8;

        format!("{} {} {}", r, g, b)
    }

    fn ppm_data(&self) -> String {
        let mut data = String::new();

        for y in 0..self.height {
            let mut row = Vec::new();

            for x in 0..self.width {
                row.push(Self::color_to_ppm(self.pixel_at(x, y)));
            }

            data.push_str(&row.join(" "));
            data.push('\n');
        }
        data
    }

    pub fn to_ppm(&self) -> String {
        format!("{}{}", self.ppm_header(), self.ppm_data())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_a_canvas() {
        let c = Canvas::new(10, 20);

        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);
        assert_eq!(c.pixels.len(), 200);
        for pixel in &c.pixels {
            assert_eq!(*pixel, Color::new(0.0, 0.0, 0.0));
        }
    }

    #[test]
    fn writing_pixels_to_canvas() {
        let mut c = Canvas::new(10, 20);
        let red = Color::new(1.0, 0.0, 0.0);

        c.write_pixel(2, 3, red);

        assert_eq!(c.pixel_at(2, 3), red);
    }

    #[test]
    fn ppm_header() {
        let c = Canvas::new(5, 3);
        let ppm = c.ppm_header();
        assert_eq!(ppm, format!("P3\n5 3\n255\n"))
    }

    #[test]
    fn ppm_pixel_data() {
        let mut c = Canvas::new(5, 3);
        let c1 = Color::new(1.5, 0.0, 0.0);
        let c2 = Color::new(0.0, 0.5, 0.0);
        let c3 = Color::new(-0.5, 0.0, 1.0);

        c.write_pixel(0, 0, c1);
        c.write_pixel(2, 1, c2);
        c.write_pixel(4, 2, c3);

        let ppm_data = c.ppm_data();

        assert_eq!(
            ppm_data,
            "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n\
                0 0 0 0 0 0 0 128 0 0 0 0 0 0 0\n\
                0 0 0 0 0 0 0 0 0 0 0 0 0 0 255\n"
        );
    }

    #[test]
    fn ppm_ends_newline() {
        let c = Canvas::new(5, 3);
        let ppm = c.to_ppm();

        assert!(ppm.ends_with('\n'));
    }
}
