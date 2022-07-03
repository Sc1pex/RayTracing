use crate::{hittable::Hittable, ray::Ray};
use cgmath::{Array, InnerSpace, Vector3, Zero};
use rayon::prelude::*;

struct Data {
    pub origin: Vector3<f32>,
    pub viewport_height: f32,
    pub viewport_width: f32,
    pub focal_length: f32,

    pub horizontal: Vector3<f32>,
    pub vertical: Vector3<f32>,
    pub lower_left_corner: Vector3<f32>,
}

impl Data {
    pub fn recalculate(&mut self, aspect_ratio: f32) {
        self.viewport_width = self.viewport_height * aspect_ratio;

        self.horizontal.x = self.viewport_width;
        self.vertical.y = self.viewport_height;
        self.lower_left_corner = self.origin
            - self.horizontal / 2.
            - self.vertical / 2.
            - Vector3::new(0., 0., self.focal_length);
    }
}

impl Default for Data {
    fn default() -> Self {
        Self {
            origin: Vector3::zero(),
            viewport_height: 2.0,
            viewport_width: 2.0,
            focal_length: 1.0,

            horizontal: Vector3::<f32>::zero(),
            vertical: Vector3::<f32>::zero(),
            lower_left_corner: Vector3::<f32>::zero(),
        }
    }
}

pub struct RayTracer {
    data: Data,
}

impl RayTracer {
    pub fn new() -> Self {
        Self {
            data: Data::default(),
        }
    }

    pub fn resize_viewport(&mut self, aspect_ratio: f32) {
        self.data.recalculate(aspect_ratio)
    }

    fn ray_color<T: Hittable>(ray: &Ray, world: &T) -> Vector3<f32> {
        if let Some(hit) = world.hit(ray, 0., f32::INFINITY) {
            return 0.5 * (hit.normal + Vector3::from_value(1.));
        }

        let unit_dir = ray.dir.normalize_to(1.0);
        let t = 0.5 * (unit_dir.y + 1.);
        let color = (1. - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0);
        color
    }

    pub fn generate_image<T: Hittable>(
        &self,
        image_width: usize,
        image_height: usize,
        world: &T,
    ) -> Vec<[f32; 3]> {
        (0..image_width * image_height)
            .into_par_iter()
            .map(|index| {
                let i = image_height - index / image_width - 1;
                let j = index % image_width;

                let u = i as f32 / (image_height - 1) as f32;
                let v = j as f32 / (image_width - 1) as f32;

                let ray = Ray::new(
                    self.data.origin,
                    self.data.lower_left_corner + u * self.data.vertical + v * self.data.horizontal,
                );

                Self::ray_color(&ray, world).into()
            })
            .collect::<Vec<_>>()
    }
}
