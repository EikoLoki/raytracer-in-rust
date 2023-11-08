use crate::color::{write_color, Color};
use crate::hittable::Hittable;
use crate::material::Scatterable;
use crate::ray::{Point3, Ray};
use crate::utility::*;
use crate::vec3::*;

use log::info;
use std::time::Instant;

#[derive(Default)]
pub struct Camera {
    pub aspect_ratio: Option<f64>,      // Ratio of image width over height
    pub image_width: Option<i32>,       // Rendered image width in pixel count
    pub samples_per_pixel: Option<i32>, // Count of random samples for each pixel
    pub max_depth: Option<i32>,         // Maximum number of ray bounces into scene

    pub vfov: Option<f64>,         // Vertical view angle (field of view)
    pub look_from: Option<Point3>, // Point camera is looking from
    pub look_at: Option<Point3>,   // Point camera is looking at
    pub vup: Option<Point3>,       // Camera-relative "up" direction

    pub defocus_angle: Option<f64>, // Variation angle of rays through each pixel
    pub focus_dist: Option<f64>,    // Distance from camera lookfrom point to plane of perfect focus

    image_height: i32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    fn initialize(&mut self) {
        if self.aspect_ratio.is_none() {
            self.aspect_ratio = Some(1.0);
        }
        if self.image_width.is_none() {
            self.image_width = Some(100);
        }
        if self.samples_per_pixel.is_none() {
            self.samples_per_pixel = Some(10);
        }
        if self.max_depth.is_none() {
            self.max_depth = Some(10);
        }
        if self.vfov.is_none() {
            self.vfov = Some(90.0);
        }
        if self.look_from.is_none() {
            self.look_from = Some(Point3::from(0.0, 0.0, -1.0));
        }
        if self.look_at.is_none() {
            self.look_at = Some(Point3::from(0.0, 0.0, 0.0));
        }
        if self.vup.is_none() {
            self.vup = Some(Point3::from(0.0, 1.0, 0.0));
        }
        if self.defocus_angle.is_none() {
            self.defocus_angle = Some(0.0);
        }
        if self.focus_dist.is_none() {
            self.focus_dist = Some(10.0);
        }

        self.image_height = (self.image_width.unwrap() as f64 / self.aspect_ratio.unwrap()) as i32;
        self.image_height = if self.image_height > 1 {
            self.image_height
        } else {
            1
        };

        self.center = self.look_from.unwrap();

        // Determine viewport dimensions.
        let theta = degrees_to_radians(
            self.vfov
                .expect("Camera is missing member vfov: vertical field of view"),
        );
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist.unwrap();
        let viewport_width =
            viewport_height * (self.image_width.unwrap() as f64 / self.image_height as f64);

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame
        self.w = unit_vector(self.look_from.unwrap() - self.look_at.unwrap());
        self.u = unit_vector(cross(&self.vup.unwrap(), &self.w));
        self.v = cross(&self.w, &self.u);

        info!("u: {:?}, v: {:?}, w: {:?}", self.u, self.v, self.w);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = viewport_width * self.u;
        let viewport_v = viewport_height * -self.v;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = viewport_u / self.image_width.unwrap() as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            self.center - self.focus_dist.unwrap() * self.w - viewport_u / 2.0 - viewport_v / 2.0;
        info!("upper left {:?}", viewport_upper_left);
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        // Calculate the camera defocus disk basis vectors.
        let defocus_radius =
            self.focus_dist.unwrap() * degrees_to_radians(self.defocus_angle.unwrap() / 2.0).tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let pixel_center =
            self.pixel00_loc + (i as f64 * self.pixel_delta_u) + (j as f64 * self.pixel_delta_v);
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin = if self.defocus_angle.unwrap() <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;

        Ray::from(ray_origin, ray_direction)
    }

    fn pixel_sample_square(&self) -> Vec3 {
        let px = -0.5 + random();
        let py = -0.5 + random();

        px * self.pixel_delta_u + py * self.pixel_delta_v
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let p = random_in_unit_disk();
        self.center + p[0] * self.defocus_disk_u + p[1] * self.defocus_disk_v
    }

    fn ray_color(r: &Ray, depth: i32, world: &dyn Hittable) -> Color {
        // When exceeds the ray bounce limit, no more light is gathered
        if depth <= 0 {
            return Color::new();
        }

        if let Some(rec) = world.hit(
            r,
            Interval {
                min: 0.001,
                max: INFINITY,
            },
        ) {
            if let Some((scattered, attenuation)) = rec.mat.scatter(r, &rec) {
                return attenuation * Self::ray_color(&scattered, depth - 1, world);
            }
            return Color::new();
        }

        let unit_direction = unit_vector(r.direction());
        let a = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - a) * Color::from(1.0, 1.0, 1.0) + a * Color::from(0.5, 0.7, 1.0)
    }

    pub fn render(&mut self, world: &dyn Hittable) {
        self.initialize();

        // Render
        let time_start = Instant::now();
        println!(
            "P3\n{} {}\n255",
            self.image_width.unwrap(),
            self.image_height
        );
        for j in 0..self.image_height {
            info!("Scanlines remaining {}", (self.image_height - j));
            for i in 0..self.image_width.unwrap() {
                let mut pixel_color = Color::new();
                for _s in 0..self.samples_per_pixel.unwrap() {
                    let r = self.get_ray(i, j);
                    pixel_color += Self::ray_color(&r, self.max_depth.unwrap(), world);
                }
                write_color(pixel_color, self.samples_per_pixel.unwrap());
            }
        }
        let duration = time_start.elapsed();
        info!("Done in {:?}.", duration);
    }
}
