use std::collections::HashMap;

use wgpu_core::id::{BufferId, QuerySetId, TextureId};
use wgpu_types::{BufferUsages, QueryType, TextureDimension, TextureFormat, TextureUsages};

pub type ComputePass = wgpu_core::command::ComputePass::<crate::Backend>;
pub type RenderPass = wgpu_core::command::RenderPass::<crate::Backend>;

pub struct Buffer {
    pub usage: BufferUsages,
    pub size: u64
}

pub struct QuerySet {
    pub count: u32,
    pub type_: QueryType
}

pub struct Texture {
    pub height: u32,
    pub width: u32,
    pub mip_level_count: u32,
    pub sample_count: u32,
    pub dimension: TextureDimension,
    pub format: TextureFormat,
    pub usage: TextureUsages
}

pub struct GpuState {
    pub buffers: HashMap<BufferId, Buffer>,
    pub query_sets: HashMap<QuerySetId, QuerySet>,
    pub textures: HashMap<TextureId, Texture>
}

impl GpuState {
    pub fn new() -> GpuState {
        GpuState {
            buffers: HashMap::new(),
            query_sets: HashMap::new(),
            textures: HashMap::new()
        }
    }
}

