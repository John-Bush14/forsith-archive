use crate::vulkan::{
    pipeline::{GraphicsPipeline, Uniform, VkPipeline}, uniform::{self, VkDescriptorSet}, vertex::{
        Vertex,
        VkBuffer,
        VkDeviceMemory
    }
};

use crate::engine::initialisation::buffer::update_uniform_buffer;


pub type Texture = [f32; 4];


pub struct Drawable {
    drawing: bool,
    pos: [f32;3],
    scale: [f32;3],
    rot: f32,
    tex: Texture,
    translation: [[f32;4];4],
    pub uniform_buffers: Vec<Vec<(VkBuffer, VkDeviceMemory)>>,
    pub indice_buffer: VkBuffer,
    pub indice_memory: VkDeviceMemory,
    vertices: Vec<Vertex>,
    pub descriptor_sets: Vec<VkDescriptorSet>,
    pub indices: Vec<u16>,
    pub id: usize,
    matrix_changed: bool,
    pub vertices_changed: (bool, bool),
    indices_changed: (bool, bool),
    pub device: u64,
    pipeline_id: usize,
}


const RECT: [[f32; 3]; 6] = [
    [-0.5, -0.5, 0.0],
    [-0.5, 0.5, 0.0],
    [0.5, 0.5, 0.0],
    [0.5, 0.5, 0.0],
    [0.5, -0.5, 0.0],
    [-0.5, -0.5, 0.0],
];

const CUBE: [[f32; 3]; 36] = [
    // Front face
    [ -0.5, -0.5,  0.5 ], // Bottom-left
    [  0.5, -0.5,  0.5 ], // Bottom-right
    [  0.5,  0.5,  0.5 ], // Top-right
    [ -0.5, -0.5,  0.5 ], // Bottom-left
    [  0.5,  0.5,  0.5 ], // Top-right
    [ -0.5,  0.5,  0.5 ], // Top-left

    // Back face
    [ -0.5,  0.5, -0.5 ], // Bottom-left
    [  0.5,  0.5, -0.5 ], // Bottom-right
    [ -0.5, -0.5, -0.5 ], // Top-right
    [  0.5,  0.5, -0.5 ], // Bottom-left
    [  0.5, -0.5, -0.5 ], // Top-right
    [ -0.5, -0.5, -0.5 ], // Top-left

    // Left face
    [ -0.5, -0.5, -0.5 ], // Bottom-left
    [ -0.5, -0.5,  0.5 ], // Bottom-right
    [ -0.5,  0.5,  0.5 ], // Top-right
    [ -0.5, -0.5, -0.5 ], // Bottom-left
    [ -0.5,  0.5,  0.5 ], // Top-right
    [ -0.5,  0.5, -0.5 ], // Top-left

    // Right face
    [  0.5,  0.5, -0.5 ], // Bottom-left
    [  0.5,  0.5,  0.5 ], // Bottom-right
    [  0.5, -0.5, -0.5 ], // Top-right
    [  0.5,  0.5,  0.5 ], // Bottom-left
    [  0.5, -0.5,  0.5 ], // Top-right
    [  0.5, -0.5, -0.5 ], // Top-left

    // Top face
    [ -0.5,  0.5,  0.5 ], // Bottom-left
    [  0.5,  0.5,  0.5 ], // Bottom-right
    [ -0.5,  0.5, -0.5 ], // Top-right
    [  0.5,  0.5,  0.5 ], // Bottom-left
    [  0.5,  0.5, -0.5 ], // Top-right
    [ -0.5,  0.5, -0.5 ], // Top-left

    // Bottom face
    [ -0.5, -0.5, -0.5 ], // Bottom-left
    [  0.5, -0.5, -0.5 ], // Bottom-right
    [  0.5, -0.5,  0.5 ], // Top-right
    [ -0.5, -0.5, -0.5 ], // Bottom-left
    [  0.5, -0.5,  0.5 ], // Top-right
    [ -0.5, -0.5,  0.5 ], // Top-left
];

fn points_to_vertices(points: Vec<[f32;3]>, color: Texture) -> Vec<Vertex> {
    points.iter().map(|&point| return Vertex {pos: point, color}).collect()
}


impl Drawable {
    pub fn get_vertices(&self) -> &Vec<Vertex> {
        return &self.vertices
    }

    pub fn update(&mut self, _image_index: usize, device: u64, pipeline: &GraphicsPipeline) -> (bool, (bool, bool), (bool, bool)) { 
        let result = (self.matrix_changed, self.vertices_changed, self.indices_changed);

        pipeline.uniforms.iter().filter(|x| match x {Uniform::Camera2d => false, Uniform::Camera3d => false, _ => true})
        .enumerate()
        .for_each(|(i, uniform)| {
            if self.matrix_changed && match uniform {Uniform::Model3d => true, Uniform::Model2d => true, _ => false} {
                let uniform_buffer = &self.uniform_buffers[i];

                match uniform {
                    Uniform::Model3d => {
                        let rot_radians = self.rot.to_radians();
                        let cos = rot_radians.cos(); let sin = rot_radians.sin();
    
                        self.translation = [
                            [cos*self.scale[0], sin, 0.0, self.pos[0]],
                            [-sin, cos*self.scale[1], 0.0, self.pos[1]],
                            [0.0, 0.0, self.scale[2], self.pos[2]],
                            [0.0, 0.0, 0.0, 1.0]
                        ];
                    },
                    Uniform::Model2d => {
                        let rot_radians = self.rot.to_radians();
                        let cos = rot_radians.cos(); let sin = rot_radians.sin();
    
                        self.translation = [
                            [cos*self.scale[0], sin, 0.0, self.pos[0]],
                            [-sin, cos*self.scale[1], 0.0, self.pos[1]],
                            [0.0, 0.0, 0.0, 0.0],
                            [0.0, 0.0, 0.0, 1.0]
                        ];
                    },
                    _ => {}
                }

                println!("{:?}", self.translation);

                for image_index in 0 .. uniform_buffer.len() {
                    update_uniform_buffer(uniform_buffer[image_index].1, device, self.translation);
                }
                
                self.matrix_changed = false;
            }
        });

        return result;
    }
    
    pub fn get_texture(&self) -> &Texture {return &self.tex}

    pub fn is_drawing(&self) -> bool {return self.drawing}

    pub fn get_id(&self) -> usize {return self.id}
}

impl Drawable {
    pub fn matrix_change(&mut self) {self.matrix_changed = true}

    pub fn pos(&self) -> &[f32;3] {return &self.pos}
    pub fn set_pos(&mut self, pos: [f32;3]) {self.pos = pos; self.matrix_change();}

    pub fn scale(&self) -> &[f32; 3] {return &self.scale}
    pub fn set_scale(&mut self, scale: [f32; 3]) {self.scale = scale; self.matrix_change();}
    
    pub fn rot(&self) -> &f32 {return &self.rot}
    pub fn set_rot(&mut self, rot: f32) {self.rot = rot; self.matrix_change();}

    pub fn set_texture(&mut self, texture: Texture) {self.tex = texture;}

    pub fn set_drawing(&mut self, drawing: bool) {self.drawing = drawing;}

    pub fn get_pipeline_id(&self) -> usize {return self.pipeline_id}
}

impl Default for Drawable {
    fn default() -> Drawable {
        return Drawable {
            drawing: true,
            pos: [0f32, 0f32, 0f32],
            scale: [1f32; 3],
            rot: 0f32,
            tex: [0f32;4],
            translation: [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0]],
            uniform_buffers: vec!(),
            descriptor_sets: vec!(),
            indice_buffer: 0,
            indice_memory: 0,
            vertices: vec!(),
            indices: vec!(),
            id: 0usize,
            matrix_changed: true,
            vertices_changed: (false, false),
            indices_changed: (true, true),
            device: 0,
            pipeline_id: 0,
        };
    }
}

impl Drawable {
    pub fn cube_from_transform(pos: [f32;3], width: f32, height: f32, depth: f32, col: Texture) -> Drawable {
        let mut drawable: Drawable = Default::default();

        drawable.tex = col;
        drawable.pos = pos;
        drawable.scale = [width, height, depth];
        drawable.vertices = points_to_vertices(CUBE.to_vec(), col);

        return drawable;
    }
    
    pub fn rect_from_transform(pos: [f32;2], width: f32, height: f32, rot: f32, col: Texture, ui: bool) -> Drawable {
        let mut drawable: Drawable = Default::default();

        drawable.tex = col;
        drawable.pos = [pos[0], pos[1], 0.0];
        drawable.scale = [width, height, 1.0];
        drawable.rot = rot;
        drawable.vertices = points_to_vertices(RECT.to_vec(), col);
        drawable.pipeline_id = if ui {3} else {1};

        return drawable;
    }
}
