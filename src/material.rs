use crate::vec::Vec3;

use super::hit::HitRecord;
use super::ray::Ray;
use super::vec::Color;

pub trait Scatter {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
} 

pub struct Lambertian {
    albedo: Color
}

impl Lambertian {
    pub fn new(color: Color) -> Lambertian {
        Lambertian {
            albedo: color
        }
    }
}

impl Scatter for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        // let scatter_direction = rec.normal + Vec3::random_in_unit_sphere().normalized();
        let mut scatter_direction = rec.p + Vec3::random_in_hemisphere(rec.normal);

        if scatter_direction.near_zero() {
            // Catch degenerate scatter direction
            scatter_direction = rec.normal;
        }

        // let ray = Ray::new(rec.p, target - rec.p);
        let scattered = Ray::new(rec.p, scatter_direction - rec.p);

        Some((self.albedo, scattered))
    }
}

pub struct Metal {
    albedo: Color
}

impl Metal {
    pub fn new(color: Color) -> Metal {
        Metal {
            albedo: color
        }
    }
}

impl Scatter for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = r_in.direction().reflect(rec.normal).normalized();
        let scattered = Ray::new(rec.p, reflected);

        if scattered.direction().dot(rec.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}