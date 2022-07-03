use cgmath::Vector3;

pub struct Ray {
    pub origin: Vector3<f32>,
    pub dir: Vector3<f32>,
}

impl Ray {
    pub fn new(origin: Vector3<f32>, dir: Vector3<f32>) -> Self {
        Self { origin, dir }
    }

    pub fn at(&self, t: f32) -> Vector3<f32> {
        self.origin + self.dir * t
    }
}
