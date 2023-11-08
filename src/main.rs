use render::camera::Camera;
use render::color::Color;
use render::hittable_list::HittableList;
use render::material::{Dieletric, Lambertian, Material, Metal};
use render::ray::Point3;
use render::sphere::Sphere;
use render::utility::*;
use render::vec3::Vec3;

fn main() {
    env_logger::init();

    // World
    let mut world: HittableList = HittableList::default();

    let ground_material = Lambertian::from(Color::from(0.5, 0.5, 0.5));
    world.add(Box::new(Sphere::from(
        Point3::from(0.0, -1000.0, 0.0),
        1000.0,
        Material::Lambertian(ground_material),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random();
            let center = Point3::from(a as f64 + 0.9 * random(), 0.2, b as f64 + 0.9 * random());

            if (center - Point3::from(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Material;

                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    sphere_material = Material::Lambertian(Lambertian::from(albedo));
                    world.add(Box::new(Sphere::from(center, 0.2, sphere_material)))
                } else if choose_mat < 0.95 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    let fuzz = random_in_range(0.0, 0.5);
                    sphere_material = Material::Metal(Metal::from(albedo, fuzz));
                    world.add(Box::new(Sphere::from(center, 0.2, sphere_material)))
                } else {
                    // diffuse
                    sphere_material = Material::Dieletric(Dieletric::from(1.5));
                    world.add(Box::new(Sphere::from(center, 0.2, sphere_material)))
                }
            }
        }
    }

    let material1 = Material::Dieletric(Dieletric::from(1.5));
    world.add(Box::new(Sphere::from(
        Point3::from(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));
    let material2 = Material::Lambertian(Lambertian::from(Color::from(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::from(
        Point3::from(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));
    let material3 = Material::Metal(Metal::from(Color::from(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::from(
        Point3::from(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    let mut camera = Camera::default();
    camera.aspect_ratio = Some(16.0 / 9.0);
    camera.image_width = Some(400);
    camera.samples_per_pixel = Some(500);
    camera.max_depth = Some(50);

    camera.vfov = Some(20.0);
    camera.look_from = Some(Point3::from(13.0, 2.0, 3.0));
    camera.look_at = Some(Point3::from(0.0, 0.0, 0.0));
    camera.vup = Some(Vec3::from(0.0, 1.0, 0.0));

    camera.defocus_angle = Some(0.6);
    camera.focus_dist = Some(10.0);

    camera.render(&world);
}
