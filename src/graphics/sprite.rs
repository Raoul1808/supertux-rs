use cgmath::Vector2;
use crate::graphics::Vertex;

pub struct Sprite {
    // todo!("Add texture here")
    position: Vector2<f32>,
    size: Vector2<f32>,
    layer_depth: f32,
}

impl Sprite {
    pub fn new_from_vector(position: Vector2<f32>, size: Vector2<f32>) -> Self {
        Self {
            position,
            size,
            layer_depth: 0.0,
        }
    }

    pub fn new_from_x_y(start_x: f32, start_y: f32, scale_x: f32, scale_y: f32) -> Self {
        Sprite::new_from_vector(Vector2::new(start_x, start_y), Vector2::new(scale_x, scale_y))
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.position.x = x;
        self.position.y = y;
    }

    pub fn set_layer_depth(&mut self, layer_depth: f32) {
        if 0.0 <= layer_depth && layer_depth <= 1.0 {
            self.layer_depth = layer_depth;
        }
    }

    pub fn get_vertex_data(&self) -> Vec<Vertex> {
        let mut vec = Vec::<Vertex>::new();
        vec.push(Vertex {
            position: [self.position.x, self.position.y, self.layer_depth],
            color: [1.0, 1.0, 1.0],
            tex_coords: [0.0, 0.0],
        });
        vec.push(Vertex {
            position: [self.position.x, self.position.y + self.size.y, self.layer_depth],
            color: [1.0, 1.0, 1.0],
            tex_coords: [0.0, 1.0],
        });
        vec.push(Vertex {
            position: [self.position.x + self.size.x, self.position.y + self.size.y, self.layer_depth],
            color: [1.0, 1.0, 1.0],
            tex_coords: [1.0, 1.0],
        });
        vec.push(Vertex {
            position: [self.position.x + self.size.x, self.position.y, self.layer_depth],
            color: [1.0, 1.0, 1.0],
            tex_coords: [1.0, 0.0],
        });
        return vec;
    }
}
