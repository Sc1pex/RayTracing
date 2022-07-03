use crate::ray::Ray;
use cgmath::{InnerSpace, Vector3, Zero};
use eframe::egui;
use rayon::prelude::*;

pub struct App {
    image: Option<egui::TextureHandle>,
    image_width: usize,
    image_height: usize,
    image_aspect_ratio: f32,

    data: Data,
}

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

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());

        Self {
            image: None,
            image_width: 0,
            image_height: 0,
            image_aspect_ratio: 1.0,

            data: Data::default(),
        }
    }

    fn vec_to_color(vec: &Vector3<f32>) -> egui::Color32 {
        egui::Color32::from(egui::Rgba::from_rgb(vec.x, vec.y, vec.z))
    }

    fn hit_sphere(center: &Vector3<f32>, raduis: f32, ray: &Ray) -> Option<f32> {
        let oc = ray.origin - center;
        let a = cgmath::dot(ray.dir, ray.dir);
        let half_b = cgmath::dot(oc, ray.dir);
        let c = cgmath::dot(oc, oc) - raduis * raduis;
        let disctiminant = half_b * half_b - a * c;
        if disctiminant < 0. {
            None
        } else {
            Some((-half_b - disctiminant.sqrt()) / a)
        }
    }

    fn ray_color(ray: &Ray) -> egui::Color32 {
        if let Some(t) = Self::hit_sphere(&Vector3::new(0., 0., -1.), 0.5, ray) {
            let n = ray.at(t) - Vector3::new(0., 0., -1.);
            return Self::vec_to_color(&(0.5 * (Vector3::new(n.x + 1., n.y + 1., n.z + 1.))));
        }

        let unit_dir = ray.dir.normalize_to(1.0);
        let t = 0.5 * (unit_dir.y + 1.);
        let color = (1. - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0);
        Self::vec_to_color(&color)
    }

    fn generate_image(&mut self) -> egui::ColorImage {
        let data = (0..self.image_width * self.image_height)
            .into_par_iter()
            .map(|index| {
                let i = self.image_height - index / self.image_width - 1;
                let j = index % self.image_width;

                let u = i as f32 / (self.image_height - 1) as f32;
                let v = j as f32 / (self.image_width - 1) as f32;

                let ray = Ray::new(
                    self.data.origin,
                    self.data.lower_left_corner + u * self.data.vertical + v * self.data.horizontal,
                );

                Self::ray_color(&ray).to_array()
            })
            .flatten()
            .collect::<Vec<u8>>();

        egui::ColorImage::from_rgba_unmultiplied([self.image_width, self.image_height], &data)
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let size = ui.min_size();
            let sizex = (size.x * ctx.pixels_per_point()) as usize;
            let sizey = (size.y * ctx.pixels_per_point()) as usize;

            if sizex != self.image_width || sizey != self.image_width {
                self.image_width = sizex;
                self.image_height = sizey;
                self.image_aspect_ratio = size.x / size.y;
                self.data.recalculate(self.image_aspect_ratio);
            }

            if let Some(image) = &self.image {
                ui.image(image, image.size_vec2());
            } else {
                ui.with_layout(
                    egui::Layout::centered_and_justified(egui::Direction::TopDown),
                    |ui| {
                        ui.add(egui::Spinner::new().size(30.0));
                    },
                );
            }
        });

        egui::Window::new("Options")
            .anchor(egui::Align2::LEFT_TOP, (0.0, 0.0))
            .show(ctx, |ui| {
                if ui.button("Trace").clicked() {
                    self.image = Some(ctx.load_texture("image", self.generate_image()));
                }
            });
    }
}
