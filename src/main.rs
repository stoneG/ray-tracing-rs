mod vec;
mod ray;
mod hit;
mod sphere;

use std::io::{stderr, Write};
use vec::{Color, Point3, Vec3};
use ray::Ray;
use hit::{Hit, World};
use sphere::Sphere;

fn ray_color(ray: &Ray, world: &World) -> Color {
    if let Some(rec) = world.hit(ray, 0.0, f64::INFINITY) {
        0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0))
    } else {
        let unit_ray = ray.direction().normalized();
        let t = 0.5 * (unit_ray.y() + 1.0);

        // hard code a linear interpolation from white to blue
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u64 = 500;
    const IMAGE_HEIGHT: u64 = ((500 as f64) / ASPECT_RATIO) as u64;

    // World
    let mut world = World::new();
    world.push(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.push(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {:3}", IMAGE_HEIGHT - j - 1);
        stderr().flush().unwrap();

        for i in 0..IMAGE_WIDTH {
            let u = (i as f64) / ((IMAGE_WIDTH - 1) as f64);
            let v = (j as f64) / ((IMAGE_HEIGHT - 1) as f64);

            let ray = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin
            );

            let pixel_color = ray_color(&ray, &world);
            println!("{}", pixel_color.format_color());
        }
    }
    eprintln!("\nDone.");
}
