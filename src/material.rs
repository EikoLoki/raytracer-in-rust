use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::utility::random;
use crate::vec3::*;

pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dieletric(Dieletric),
}

impl Scatterable for Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        match self {
            Material::Lambertian(l) => l.scatter(r_in, rec),
            Material::Metal(m) => m.scatter(r_in, rec),
            Material::Dieletric(d) => d.scatter(r_in, rec),
        }
    }
}

pub trait Scatterable {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)>;
}

#[derive(Clone, Copy)]
pub struct Lambertian {
    albedo: Color,
}

#[derive(Clone, Copy)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

#[derive(Clone, Copy)]
pub struct Dieletric {
    ir: f64,
}

impl Lambertian {
    pub fn from(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Metal {
    pub fn from(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Dieletric {
    pub fn from(ir: f64) -> Self {
        Self { ir }
    }
}

impl Scatterable for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction = rec.normal + random_unit_vector();
        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = Ray::from(rec.p, scatter_direction);
        Some((scattered, self.albedo))
    }
}

impl Scatterable for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = reflect(&unit_vector(r_in.direction()), &rec.normal);
        let scattered = Ray::from(rec.p, reflected + self.fuzz * random_unit_vector());
        if dot(&scattered.direction(), &rec.normal) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}

impl Scatterable for Dieletric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let attenuation = Color::from(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = unit_vector(r_in.direction());

        let cos_theta = f64::min(dot(&-unit_direction, &rec.normal), 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);
        let cannot_refract = sin_theta * refraction_ratio > 1.0;

        let direction: Vec3 =
            if cannot_refract || reflectance(cos_theta, refraction_ratio) > random() {
                reflect(&unit_direction, &rec.normal)
            } else {
                refract(&unit_direction, &rec.normal, refraction_ratio)
            };

        let scattered = Ray::from(rec.p, direction);
        Some((scattered, attenuation))
    }
}

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    (*v) - 2.0 * dot(v, n) * (*n)
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    // Use Schlick's approximation for reflectance
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;

    r0 + (1.0 - r0) * f64::powi(1.0 - cosine, 5)
}

fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = f64::min(dot(&-(*uv), n), 1.0);
    let r_out_perp = etai_over_etat * (*uv + cos_theta * *n);
    let r_out_parallel = -f64::sqrt(f64::abs(1.0 - r_out_perp.length_squared())) * *n;

    r_out_parallel + r_out_perp
}
