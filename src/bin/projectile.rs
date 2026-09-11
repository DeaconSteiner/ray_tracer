// projectile.rs
use ray_tracer::{Tuple, canvas::Canvas, color::Color};

pub struct Projectile {
    pub position: Tuple,
    pub velocity: Tuple,
}

pub struct Environment {
    pub gravity: Tuple,
    pub wind: Tuple,
}

pub fn tick(env: &Environment, proj: &Projectile) -> Projectile {
    Projectile {
        position: proj.position + proj.velocity,
        velocity: proj.velocity + env.gravity + env.wind,
    }
}

fn main() -> std::io::Result<()> {
    // projectile starts 1 unit above origin
    // velocity is normalized to 1 unit/tick
    let mut p = Projectile {
        position: Tuple::point(0.0, 1.0, 0.0),
        velocity: Tuple::vector(1.0, 1.0, 0.0).normalize() * 11.25,
    };

    // gravity -0.1 unit/tick, and wind is -0.01 unit/tick
    let e = Environment {
        gravity: Tuple::vector(0.0, -0.1, 0.0),
        wind: Tuple::vector(-0.01, 0.0, 0.0),
    };

    // 900x550 canvas
    let mut c = Canvas::new(900, 550);

    let red = Color::new(1.0, 0.0, 0.0);

    while p.position.y() > 0.0 {
        let x = p.position.x().round() as usize;
        let y = (c.height() as f64 - p.position.y().round()) as usize;

        if x < c.width() && y < c.height() {
            c.write_pixel(x, y, red);
        }
        p = tick(&e, &p);
    }

    let ppm = c.to_ppm();

    match std::fs::write("output/projectile.ppm", ppm) {
        Ok(()) => println!("File Written!"),
        Err(error) => println!("Failed to write file: {}", error),
    }

    Ok(())
}
