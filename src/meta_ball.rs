pub struct MetaBall {
    pub position: [f32; 2],
    pub radius: f32,
}

impl MetaBall {
    pub fn f(&self, x: f32, y: f32) -> f32 {
        let [x0, y0] = self.position;
        return 1.0 / ((x - x0).powf(2.0) + (y - y0).powf(2.0)).sqrt();
    }
}
