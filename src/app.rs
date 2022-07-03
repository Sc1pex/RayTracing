use crate::{
    hittable::{HittableList, Sphere},
    ray_tracer::RayTracer,
};
use cgmath::Vector3;
use eframe::egui;

pub struct App {
    image: Option<egui::TextureHandle>,
    image_width: usize,
    image_height: usize,
    image_aspect_ratio: f32,

    ray_tracer: RayTracer,
    world: HittableList,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());

        let mut world = HittableList::new();
        world.add(Box::new(Sphere::new(0.5, Vector3::<f32>::new(0., 0., -1.))));
        world.add(Box::new(Sphere::new(
            100.,
            Vector3::<f32>::new(0., -100.5, -1.),
        )));

        Self {
            image: None,
            image_width: 0,
            image_height: 0,
            image_aspect_ratio: 1.0,

            ray_tracer: RayTracer::new(),
            world,
        }
    }

    fn generate_image(&mut self) -> egui::ColorImage {
        let data = self
            .ray_tracer
            .generate_image(self.image_width, self.image_height, &self.world)
            .into_iter()
            .map(|x| Into::<egui::Color32>::into(egui::Rgba::from_rgb(x[0], x[1], x[2])).to_array())
            .flatten()
            .collect::<Vec<_>>();
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
                self.ray_tracer.resize_viewport(self.image_aspect_ratio);
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
