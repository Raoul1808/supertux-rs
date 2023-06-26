use winit::dpi::PhysicalSize;

#[rustfmt::skip]
const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);


pub struct Camera {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl Camera {
    pub fn new(size: PhysicalSize<u32>) -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width: size.width as f32,
            height: size.height as f32,
        }
    }

    pub fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        let view = cgmath::Matrix4::look_at_rh(
            (0.0, 0.0, 1.0).into(),
            (0.0, 0.0, 0.0).into(),
            cgmath::Vector3::unit_y()
        );
        let proj = cgmath::ortho(self.x, self.width, self.height, self.y, 0.0, 1.0);
        return OPENGL_TO_WGPU_MATRIX * proj * view;
    }
}
