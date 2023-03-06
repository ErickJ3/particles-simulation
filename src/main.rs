extern crate piston_window;
extern crate rand;

use piston_window::*;
use rand::Rng;

struct Particle {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    color: [f32; 4],
}

fn main() {
    let mut particles: Vec<Particle> = vec![];

    for _i in 0..100 {
        let p = Particle {
            x: random_range(0.0, 640.0),
            y: random_range(0.0, 480.0),
            vx: random_range(-50.0, 50.0),
            vy: random_range(-50.0, 50.0),
            color: [20.0, 10.0, 10.0, 1.0],
        };
        particles.push(p);
    }

    let mut window: PistonWindow = WindowSettings::new("Particles Simulation", [1024, 720])
        .exit_on_esc(true)
        .build()
        .unwrap();

    while let Some(event) = window.next() {
        for p in &mut particles {
            p.x += p.vx * random_range(0.0, 1.0);
            p.y += p.vy * random_range(0.0, 1.0);

            if p.x < 0.0 || p.x > random_range(0.0, 229.0) {
                p.vx = -p.vx;
            }

            if p.y < 0.0 || p.y > random_range(0.0, 2.0) {
                p.vy = -p.vy;
            }
        }

        window.draw_2d(&event, |c, g, _device| {
            clear([0.1, 0.2, 0.3, 1.0], g);
            for p in &particles {
                rectangle(p.color, [p.x - 2.0, p.y - 2.0, 4.0, 4.0], c.transform, g);
            }
        });
    }
}

fn random_range(v1: f64, v2: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(v1..v2)
}
