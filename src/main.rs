mod vec;
mod ray;
mod hit;
mod sphere;
mod camera;

use std::io::{stderr, Write};
use rand::Rng;
use vec::{Color, Point3, Vec3};
use ray::Ray;
use hit::{Hit, World};
use sphere::Sphere;
use camera::Camera;

fn ray_color(ray: &Ray, world: &World) -> Color {
    if let Some(rec) = world.hit(ray, 0.0, f64::INFINITY) {
        0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0))
    } else {
        let unit_ray = ray.direction().normalized();
        let t = 0.5 * (unit_ray.y() + 1.0);

        // Background will be a hard coded
        // linear interpolation from white to blue
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u64 = 500;
    const IMAGE_HEIGHT: u64 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u64;
    const SAMPLES_PER_PIXEL: u64 = 100;

    // World
    let mut world = World::new();
    world.push(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.push(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let cam = Camera::new();

    // Image Result
    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    let mut rng = rand::thread_rng();
    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {:3}", IMAGE_HEIGHT - j - 1);
        stderr().flush().unwrap();

        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let random_u: f64 = rng.gen();
                let random_v: f64 = rng.gen();

                let u = ((i as f64) + random_u) / ((IMAGE_WIDTH - 1) as f64);
                let v = ((j as f64) + random_v) / ((IMAGE_HEIGHT - 1) as f64);

                let ray = cam.get_ray(u, v);
                pixel_color += ray_color(&ray, &world);
            }

            println!("{}", pixel_color.format_color(SAMPLES_PER_PIXEL));
        }
    }
    eprintln!("\nDone.");
}
