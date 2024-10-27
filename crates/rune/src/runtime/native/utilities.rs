use std::{borrow::Cow, num::NonZeroU64};

use wasmtime::component::ResourceTable;
use wgpu_core::{
    binding_model::{BindGroupEntry, BindingResource, BufferBinding},
    command::{LoadOp, StoreOp},
    device::HostMap,
    resource::BufferMapOperation,
};
use wgpu_types::{
    AddressMode, AstcBlock, AstcChannel, BlendComponent, BlendFactor, BlendOperation, BlendState,
    BufferUsages, Color, ColorWrites, CompareFunction, Extent3d, FilterMode, IndexFormat,
    QueryType, ShaderStages, StencilFaceState, StencilOperation, TextureAspect, TextureDimension,
    TextureFormat, TextureUsages,
};

use crate::gpu::{
    GpuAddressMode, GpuBindGroupEntry, GpuBlendComponent, GpuBlendFactor, GpuBlendOperation,
    GpuBlendState, GpuBufferUsage, GpuColorWrite, GpuCompareFunction, GpuExtentD3, GpuFilterMode,
    GpuIndexFormat, GpuLoadOp, GpuMapMode, GpuQueryType, GpuShaderStage, GpuStencilFaceState,
    GpuStencilOperation, GpuStoreOp, GpuTextureAspect, GpuTextureDimension, GpuTextureFormat,
    GpuTextureUsage,
};

// use crate::renderer::{GpuCompareFunction, GpuTextureFormat, GpuStencilOperation, GpuStencilFaceState, GpuColorWrite, GpuBlendState, GpuBlendComponent, GpuBlendFactor, GpuBlendOperation};

pub fn vec_to_color(vec: &Vec<f64>) -> Color {
    assert_eq!(vec.len(), 4);

    Color {
        r: vec[0],
        g: vec[1],
        b: vec[2],
        a: vec[3],
    }
}

impl Into<AddressMode> for GpuAddressMode {
    fn into(self) -> AddressMode {
        match self {
            GpuAddressMode::ClampToEdge => AddressMode::ClampToEdge,
            GpuAddressMode::MirrorRepeat => AddressMode::MirrorRepeat,
            GpuAddressMode::Repeat => AddressMode::Repeat,
        }
    }
}

impl Into<Extent3d> for GpuExtentD3 {
    fn into(self) -> Extent3d {
        Extent3d {
            width: self.width,
            height: self.height,
            depth_or_array_layers: self.depth_or_array_layers,
        }
    }
}

impl Into<CompareFunction> for GpuCompareFunction {
    fn into(self) -> CompareFunction {
        match self {
            GpuCompareFunction::Never => CompareFunction::Never,
            GpuCompareFunction::Less => CompareFunction::Less,
            GpuCompareFunction::Equal => CompareFunction::Equal,
            GpuCompareFunction::LessEqual => CompareFunction::LessEqual,
            GpuCompareFunction::Greater => CompareFunction::Greater,
            GpuCompareFunction::NotEqual => CompareFunction::NotEqual,
            GpuCompareFunction::GreaterEqual => CompareFunction::GreaterEqual,
            GpuCompareFunction::Always => CompareFunction::Always,
        }
    }
}

impl Into<FilterMode> for GpuFilterMode {
    fn into(self) -> FilterMode {
        match self {
            GpuFilterMode::Linear => FilterMode::Linear,
            GpuFilterMode::Nearest => FilterMode::Nearest,
        }
    }
}

impl Into<IndexFormat> for GpuIndexFormat {
    fn into(self) -> IndexFormat {
        match self {
            GpuIndexFormat::Uint16 => IndexFormat::Uint16,
            GpuIndexFormat::Uint32 => IndexFormat::Uint32,
        }
    }
}

impl Into<ShaderStages> for GpuShaderStage {
    fn into(self) -> ShaderStages {
        let mut ss = ShaderStages::empty();
        if self.contains(GpuShaderStage::COMPUTE) {
            ss |= ShaderStages::COMPUTE;
        }
        if self.contains(GpuShaderStage::FRAGMENT) {
            ss |= ShaderStages::FRAGMENT;
        }
        if self.contains(GpuShaderStage::VERTEX) {
            ss |= ShaderStages::VERTEX;
        }

        ss
    }
}

impl Into<TextureAspect> for GpuTextureAspect {
    fn into(self) -> TextureAspect {
        match self {
            GpuTextureAspect::All => TextureAspect::All,
            GpuTextureAspect::DepthOnly => TextureAspect::DepthOnly,
            GpuTextureAspect::StencilOnly => TextureAspect::StencilOnly,
        }
    }
}

impl Into<TextureFormat> for GpuTextureFormat {
    fn into(self) -> TextureFormat {
        match self {
            GpuTextureFormat::R8unorm => TextureFormat::R8Unorm,
            GpuTextureFormat::R8snorm => TextureFormat::R8Snorm,
            GpuTextureFormat::R8uint => TextureFormat::R8Uint,
            GpuTextureFormat::R8sint => TextureFormat::R8Sint,
            GpuTextureFormat::R16uint => TextureFormat::R16Uint,
            GpuTextureFormat::R16sint => TextureFormat::R16Sint,
            GpuTextureFormat::R16float => TextureFormat::R16Float,
            GpuTextureFormat::Rg8unorm => TextureFormat::Rg8Unorm,
            GpuTextureFormat::Rg8snorm => TextureFormat::Rg8Snorm,
            GpuTextureFormat::Rg8uint => TextureFormat::Rg8Uint,
            GpuTextureFormat::Rg8sint => TextureFormat::Rg8Sint,
            GpuTextureFormat::R32uint => TextureFormat::R32Uint,
            GpuTextureFormat::R32sint => TextureFormat::R32Sint,
            GpuTextureFormat::R32float => TextureFormat::R32Float,
            GpuTextureFormat::Rg16uint => TextureFormat::Rg16Uint,
            GpuTextureFormat::Rg16sint => TextureFormat::Rg16Sint,
            GpuTextureFormat::Rg16float => TextureFormat::Rg16Float,
            GpuTextureFormat::Rgba8unorm => TextureFormat::Rgba8Unorm,
            GpuTextureFormat::Rgba8unormsrgb => TextureFormat::Rgba8UnormSrgb,
            GpuTextureFormat::Rgba8snorm => TextureFormat::Rgba8Snorm,
            GpuTextureFormat::Rgba8uint => TextureFormat::Rgba8Uint,
            GpuTextureFormat::Rgba8sint => TextureFormat::Rgba8Sint,
            GpuTextureFormat::Bgra8unorm => TextureFormat::Bgra8Unorm,
            GpuTextureFormat::Bgra8unormsrgb => TextureFormat::Bgra8UnormSrgb,
            GpuTextureFormat::Rgb9e5ufloat => TextureFormat::Rgb9e5Ufloat,
            GpuTextureFormat::Rgb10a2unorm => TextureFormat::Rgb10a2Unorm,
            GpuTextureFormat::Rg11b10ufloat => TextureFormat::Rg11b10Ufloat,
            GpuTextureFormat::Rg32uint => TextureFormat::Rg32Uint,
            GpuTextureFormat::Rg32sint => TextureFormat::Rg32Sint,
            GpuTextureFormat::Rg32float => TextureFormat::Rg32Float,
            GpuTextureFormat::Rgba16uint => TextureFormat::Rgba16Uint,
            GpuTextureFormat::Rgba16sint => TextureFormat::Rgba16Sint,
            GpuTextureFormat::Rgba16float => TextureFormat::Rgba16Float,
            GpuTextureFormat::Rgba32uint => TextureFormat::Rgba32Uint,
            GpuTextureFormat::Rgba32sint => TextureFormat::Rgba32Sint,
            GpuTextureFormat::Rgba32float => TextureFormat::Rgba32Float,
            GpuTextureFormat::Stencil8 => TextureFormat::Stencil8,
            GpuTextureFormat::Depth16unorm => TextureFormat::Depth16Unorm,
            GpuTextureFormat::Depth24plus => TextureFormat::Depth24Plus,
            GpuTextureFormat::Depth24plusstencil8 => TextureFormat::Depth24PlusStencil8,
            GpuTextureFormat::Depth32float => TextureFormat::Depth32Float,
            GpuTextureFormat::Depth32floatstencil8 => TextureFormat::Depth32FloatStencil8,
            GpuTextureFormat::Bc1rgbaunorm => TextureFormat::Bc1RgbaUnorm,
            GpuTextureFormat::Bc1rgbaunormsrgb => TextureFormat::Bc1RgbaUnormSrgb,
            GpuTextureFormat::Bc2rgbaunorm => TextureFormat::Bc2RgbaUnorm,
            GpuTextureFormat::Bc2rgbaunormsrgb => TextureFormat::Bc2RgbaUnormSrgb,
            GpuTextureFormat::Bc3rgbaunorm => TextureFormat::Bc3RgbaUnorm,
            GpuTextureFormat::Bc3rgbaunormsrgb => TextureFormat::Bc3RgbaUnormSrgb,
            GpuTextureFormat::Bc4runorm => TextureFormat::Bc4RUnorm,
            GpuTextureFormat::Bc4rsnorm => TextureFormat::Bc4RSnorm,
            GpuTextureFormat::Bc5rgunorm => TextureFormat::Bc5RgUnorm,
            GpuTextureFormat::Bc5rgsnorm => TextureFormat::Bc5RgSnorm,
            GpuTextureFormat::Bc6hrgbufloat => TextureFormat::Bc6hRgbUfloat,
            GpuTextureFormat::Bc6hrgbfloat => TextureFormat::Bc6hRgbFloat,
            GpuTextureFormat::Bc7rgbaunorm => TextureFormat::Bc7RgbaUnorm,
            GpuTextureFormat::Bc7rgbaunormsrgb => TextureFormat::Bc7RgbaUnormSrgb,
            GpuTextureFormat::Etc2rgb8unorm => TextureFormat::Etc2Rgb8Unorm,
            GpuTextureFormat::Etc2rgb8unormsrgb => TextureFormat::Etc2Rgb8UnormSrgb,
            GpuTextureFormat::Etc2rgb8a1unorm => TextureFormat::Etc2Rgb8A1Unorm,
            GpuTextureFormat::Etc2rgb8a1unormsrgb => TextureFormat::Etc2Rgb8A1UnormSrgb,
            GpuTextureFormat::Etc2rgba8unorm => TextureFormat::Etc2Rgba8Unorm,
            GpuTextureFormat::Etc2rgba8unormsrgb => TextureFormat::Etc2Rgba8UnormSrgb,
            GpuTextureFormat::Eacr11unorm => TextureFormat::EacR11Unorm,
            GpuTextureFormat::Eacr11snorm => TextureFormat::EacR11Snorm,
            GpuTextureFormat::Eacrg11unorm => TextureFormat::EacRg11Unorm,
            GpuTextureFormat::Eacrg11snorm => TextureFormat::EacRg11Snorm,
            GpuTextureFormat::Astc4x4unorm => TextureFormat::Astc {
                block: AstcBlock::B4x4,
                channel: AstcChannel::Unorm,
            },
            GpuTextureFormat::Astc4x4unormsrgb => TextureFormat::Astc {
                block: AstcBlock::B4x4,
                channel: AstcChannel::UnormSrgb,
            },
            GpuTextureFormat::Astc5x4unorm => TextureFormat::Astc {
                block: AstcBlock::B5x4,
                channel: AstcChannel::Unorm,
            },
            GpuTextureFormat::Astc5x4unormsrgb => TextureFormat::Astc {
                block: AstcBlock::B5x4,
                channel: AstcChannel::UnormSrgb,
            },
            GpuTextureFormat::Astc5x5unorm => TextureFormat::Astc {
                block: AstcBlock::B5x5,
                channel: AstcChannel::Unorm,
            },
            GpuTextureFormat::Astc5x5unormsrgb => TextureFormat::Astc {
                block: AstcBlock::B5x5,
                channel: AstcChannel::UnormSrgb,
            },
            GpuTextureFormat::Astc6x5unorm => TextureFormat::Astc {
                block: AstcBlock::B6x5,
                channel: AstcChannel::Unorm,
            },
            GpuTextureFormat::Astc6x5unormsrgb => TextureFormat::Astc {
                block: AstcBlock::B6x5,
                channel: AstcChannel::UnormSrgb,
            },
            GpuTextureFormat::Astc6x6unorm => TextureFormat::Astc {
                block: AstcBlock::B6x6,
                channel: AstcChannel::Unorm,
            },
            GpuTextureFormat::Astc6x6unormsrgb => TextureFormat::Astc {
                block: AstcBlock::B6x6,
                channel: AstcChannel::UnormSrgb,
            },
            GpuTextureFormat::Astc8x5unorm => TextureFormat::Astc {
                block: AstcBlock::B8x5,
                channel: AstcChannel::Unorm,
            },
            GpuTextureFormat::Astc8x5unormsrgb => TextureFormat::Astc {
                block: AstcBlock::B8x5,
                channel: AstcChannel::UnormSrgb,
            },
            GpuTextureFormat::Astc8x6unorm => TextureFormat::Astc {
                block: AstcBlock::B8x6,
                channel: AstcChannel::Unorm,
            },
            GpuTextureFormat::Astc8x6unormsrgb => TextureFormat::Astc {
                block: AstcBlock::B8x6,
                channel: AstcChannel::UnormSrgb,
            },
            GpuTextureFormat::Astc8x8unorm => TextureFormat::Astc {
                block: AstcBlock::B8x8,
                channel: AstcChannel::Unorm,
            },
            GpuTextureFormat::Astc8x8unormsrgb => TextureFormat::Astc {
                block: AstcBlock::B8x8,
                channel: AstcChannel::UnormSrgb,
            },
            GpuTextureFormat::Astc10x5unorm => TextureFormat::Astc {
                block: AstcBlock::B10x5,
                channel: AstcChannel::Unorm,
            },
            GpuTextureFormat::Astc10x5unormsrgb => TextureFormat::Astc {
                block: AstcBlock::B10x5,
                channel: AstcChannel::UnormSrgb,
            },
            GpuTextureFormat::Astc10x6unorm => TextureFormat::Astc {
                block: AstcBlock::B10x6,
                channel: AstcChannel::Unorm,
            },
            GpuTextureFormat::Astc10x6unormsrgb => TextureFormat::Astc {
                block: AstcBlock::B10x6,
                channel: AstcChannel::UnormSrgb,
            },
            GpuTextureFormat::Astc10x8unorm => TextureFormat::Astc {
                block: AstcBlock::B10x8,
                channel: AstcChannel::Unorm,
            },
            GpuTextureFormat::Astc10x8unormsrgb => TextureFormat::Astc {
                block: AstcBlock::B10x8,
                channel: AstcChannel::UnormSrgb,
            },
            GpuTextureFormat::Astc10x10unorm => TextureFormat::Astc {
                block: AstcBlock::B10x10,
                channel: AstcChannel::Unorm,
            },
            GpuTextureFormat::Astc10x10unormsrgb => TextureFormat::Astc {
                block: AstcBlock::B10x10,
                channel: AstcChannel::UnormSrgb,
            },
            GpuTextureFormat::Astc12x10unorm => TextureFormat::Astc {
                block: AstcBlock::B12x10,
                channel: AstcChannel::Unorm,
            },
            GpuTextureFormat::Astc12x10unormsrgb => TextureFormat::Astc {
                block: AstcBlock::B12x10,
                channel: AstcChannel::UnormSrgb,
            },
            GpuTextureFormat::Astc12x12unorm => TextureFormat::Astc {
                block: AstcBlock::B12x12,
                channel: AstcChannel::Unorm,
            },
            GpuTextureFormat::Astc12x12unormsrgb => TextureFormat::Astc {
                block: AstcBlock::B12x12,
                channel: AstcChannel::UnormSrgb,
            },
        }
    }
}

impl Into<GpuTextureFormat> for TextureFormat {
    fn into(self) -> GpuTextureFormat {
        match self {
            TextureFormat::R8Unorm => GpuTextureFormat::R8unorm,
            TextureFormat::R8Snorm => GpuTextureFormat::R8snorm,
            TextureFormat::R8Uint => GpuTextureFormat::R8uint,
            TextureFormat::R8Sint => GpuTextureFormat::R8sint,
            TextureFormat::R16Uint => GpuTextureFormat::R16uint,
            TextureFormat::R16Sint => GpuTextureFormat::R16sint,
            TextureFormat::R16Float => GpuTextureFormat::R16float,
            TextureFormat::Rg8Unorm => GpuTextureFormat::Rg8unorm,
            TextureFormat::Rg8Snorm => GpuTextureFormat::Rg8snorm,
            TextureFormat::Rg8Uint => GpuTextureFormat::Rg8uint,
            TextureFormat::Rg8Sint => GpuTextureFormat::Rg8sint,
            TextureFormat::R32Uint => GpuTextureFormat::R32uint,
            TextureFormat::R32Sint => GpuTextureFormat::R32sint,
            TextureFormat::R32Float => GpuTextureFormat::R32float,
            TextureFormat::Rg16Uint => GpuTextureFormat::Rg16uint,
            TextureFormat::Rg16Sint => GpuTextureFormat::Rg16sint,
            TextureFormat::Rg16Float => GpuTextureFormat::Rg16float,
            TextureFormat::Rgba8Unorm => GpuTextureFormat::Rgba8unorm,
            TextureFormat::Rgba8UnormSrgb => GpuTextureFormat::Rgba8unormsrgb,
            TextureFormat::Rgba8Snorm => GpuTextureFormat::Rgba8snorm,
            TextureFormat::Rgba8Uint => GpuTextureFormat::Rgba8uint,
            TextureFormat::Rgba8Sint => GpuTextureFormat::Rgba8sint,
            TextureFormat::Bgra8Unorm => GpuTextureFormat::Bgra8unorm,
            TextureFormat::Bgra8UnormSrgb => GpuTextureFormat::Bgra8unormsrgb,
            TextureFormat::Rgb9e5Ufloat => GpuTextureFormat::Rgb9e5ufloat,
            TextureFormat::Rgb10a2Unorm => GpuTextureFormat::Rgb10a2unorm,
            TextureFormat::Rg11b10Ufloat => GpuTextureFormat::Rg11b10ufloat,
            TextureFormat::Rg32Uint => GpuTextureFormat::Rg32uint,
            TextureFormat::Rg32Sint => GpuTextureFormat::Rg32sint,
            TextureFormat::Rg32Float => GpuTextureFormat::Rg32float,
            TextureFormat::Rgba16Uint => GpuTextureFormat::Rgba16uint,
            TextureFormat::Rgba16Sint => GpuTextureFormat::Rgba16sint,
            TextureFormat::Rgba16Float => GpuTextureFormat::Rgba16float,
            TextureFormat::Rgba32Uint => GpuTextureFormat::Rgba32uint,
            TextureFormat::Rgba32Sint => GpuTextureFormat::Rgba32sint,
            TextureFormat::Rgba32Float => GpuTextureFormat::Rgba32float,
            TextureFormat::Stencil8 => GpuTextureFormat::Stencil8,
            TextureFormat::Depth16Unorm => GpuTextureFormat::Depth16unorm,
            TextureFormat::Depth24Plus => GpuTextureFormat::Depth24plus,
            TextureFormat::Depth24PlusStencil8 => GpuTextureFormat::Depth24plusstencil8,
            TextureFormat::Depth32Float => GpuTextureFormat::Depth32float,
            TextureFormat::Depth32FloatStencil8 => GpuTextureFormat::Depth32floatstencil8,
            TextureFormat::Bc1RgbaUnorm => GpuTextureFormat::Bc1rgbaunorm,
            TextureFormat::Bc1RgbaUnormSrgb => GpuTextureFormat::Bc1rgbaunormsrgb,
            TextureFormat::Bc2RgbaUnorm => GpuTextureFormat::Bc2rgbaunorm,
            TextureFormat::Bc2RgbaUnormSrgb => GpuTextureFormat::Bc2rgbaunormsrgb,
            TextureFormat::Bc3RgbaUnorm => GpuTextureFormat::Bc3rgbaunorm,
            TextureFormat::Bc3RgbaUnormSrgb => GpuTextureFormat::Bc3rgbaunormsrgb,
            TextureFormat::Bc4RUnorm => GpuTextureFormat::Bc4runorm,
            TextureFormat::Bc4RSnorm => GpuTextureFormat::Bc4rsnorm,
            TextureFormat::Bc5RgUnorm => GpuTextureFormat::Bc5rgunorm,
            TextureFormat::Bc5RgSnorm => GpuTextureFormat::Bc5rgsnorm,
            TextureFormat::Bc6hRgbUfloat => GpuTextureFormat::Bc6hrgbufloat,
            TextureFormat::Bc6hRgbFloat => GpuTextureFormat::Bc6hrgbfloat,
            TextureFormat::Bc7RgbaUnorm => GpuTextureFormat::Bc7rgbaunorm,
            TextureFormat::Bc7RgbaUnormSrgb => GpuTextureFormat::Bc7rgbaunormsrgb,
            TextureFormat::Etc2Rgb8Unorm => GpuTextureFormat::Etc2rgb8unorm,
            TextureFormat::Etc2Rgb8UnormSrgb => GpuTextureFormat::Etc2rgb8unormsrgb,
            TextureFormat::Etc2Rgb8A1Unorm => GpuTextureFormat::Etc2rgb8a1unorm,
            TextureFormat::Etc2Rgb8A1UnormSrgb => GpuTextureFormat::Etc2rgb8a1unormsrgb,
            TextureFormat::Etc2Rgba8Unorm => GpuTextureFormat::Etc2rgba8unorm,
            TextureFormat::Etc2Rgba8UnormSrgb => GpuTextureFormat::Etc2rgba8unormsrgb,
            TextureFormat::EacR11Unorm => GpuTextureFormat::Eacr11unorm,
            TextureFormat::EacR11Snorm => GpuTextureFormat::Eacr11snorm,
            TextureFormat::EacRg11Unorm => GpuTextureFormat::Eacrg11unorm,
            TextureFormat::EacRg11Snorm => GpuTextureFormat::Eacrg11snorm,
            TextureFormat::Astc {
                block: AstcBlock::B4x4,
                channel: AstcChannel::Unorm,
            } => GpuTextureFormat::Astc4x4unorm,
            TextureFormat::Astc {
                block: AstcBlock::B4x4,
                channel: AstcChannel::UnormSrgb,
            } => GpuTextureFormat::Astc4x4unormsrgb,
            TextureFormat::Astc {
                block: AstcBlock::B5x4,
                channel: AstcChannel::Unorm,
            } => GpuTextureFormat::Astc5x4unorm,
            TextureFormat::Astc {
                block: AstcBlock::B5x4,
                channel: AstcChannel::UnormSrgb,
            } => GpuTextureFormat::Astc5x4unormsrgb,
            TextureFormat::Astc {
                block: AstcBlock::B5x5,
                channel: AstcChannel::Unorm,
            } => GpuTextureFormat::Astc5x5unorm,
            TextureFormat::Astc {
                block: AstcBlock::B5x5,
                channel: AstcChannel::UnormSrgb,
            } => GpuTextureFormat::Astc5x5unormsrgb,
            TextureFormat::Astc {
                block: AstcBlock::B6x5,
                channel: AstcChannel::Unorm,
            } => GpuTextureFormat::Astc6x5unorm,
            TextureFormat::Astc {
                block: AstcBlock::B6x5,
                channel: AstcChannel::UnormSrgb,
            } => GpuTextureFormat::Astc6x5unormsrgb,
            TextureFormat::Astc {
                block: AstcBlock::B6x6,
                channel: AstcChannel::Unorm,
            } => GpuTextureFormat::Astc6x6unorm,
            TextureFormat::Astc {
                block: AstcBlock::B6x6,
                channel: AstcChannel::UnormSrgb,
            } => GpuTextureFormat::Astc6x6unormsrgb,
            TextureFormat::Astc {
                block: AstcBlock::B8x5,
                channel: AstcChannel::Unorm,
            } => GpuTextureFormat::Astc8x5unorm,
            TextureFormat::Astc {
                block: AstcBlock::B8x5,
                channel: AstcChannel::UnormSrgb,
            } => GpuTextureFormat::Astc8x5unormsrgb,
            TextureFormat::Astc {
                block: AstcBlock::B8x6,
                channel: AstcChannel::Unorm,
            } => GpuTextureFormat::Astc8x6unorm,
            TextureFormat::Astc {
                block: AstcBlock::B8x6,
                channel: AstcChannel::UnormSrgb,
            } => GpuTextureFormat::Astc8x6unormsrgb,
            TextureFormat::Astc {
                block: AstcBlock::B8x8,
                channel: AstcChannel::Unorm,
            } => GpuTextureFormat::Astc8x8unorm,
            TextureFormat::Astc {
                block: AstcBlock::B8x8,
                channel: AstcChannel::UnormSrgb,
            } => GpuTextureFormat::Astc8x8unormsrgb,
            TextureFormat::Astc {
                block: AstcBlock::B10x5,
                channel: AstcChannel::Unorm,
            } => GpuTextureFormat::Astc10x5unorm,
            TextureFormat::Astc {
                block: AstcBlock::B10x5,
                channel: AstcChannel::UnormSrgb,
            } => GpuTextureFormat::Astc10x5unormsrgb,
            TextureFormat::Astc {
                block: AstcBlock::B10x6,
                channel: AstcChannel::Unorm,
            } => GpuTextureFormat::Astc10x6unorm,
            TextureFormat::Astc {
                block: AstcBlock::B10x6,
                channel: AstcChannel::UnormSrgb,
            } => GpuTextureFormat::Astc10x6unormsrgb,
            TextureFormat::Astc {
                block: AstcBlock::B10x8,
                channel: AstcChannel::Unorm,
            } => GpuTextureFormat::Astc10x8unorm,
            TextureFormat::Astc {
                block: AstcBlock::B10x8,
                channel: AstcChannel::UnormSrgb,
            } => GpuTextureFormat::Astc10x8unormsrgb,
            TextureFormat::Astc {
                block: AstcBlock::B10x10,
                channel: AstcChannel::Unorm,
            } => GpuTextureFormat::Astc10x10unorm,
            TextureFormat::Astc {
                block: AstcBlock::B10x10,
                channel: AstcChannel::UnormSrgb,
            } => GpuTextureFormat::Astc10x10unormsrgb,
            TextureFormat::Astc {
                block: AstcBlock::B12x10,
                channel: AstcChannel::Unorm,
            } => GpuTextureFormat::Astc12x10unorm,
            TextureFormat::Astc {
                block: AstcBlock::B12x10,
                channel: AstcChannel::UnormSrgb,
            } => GpuTextureFormat::Astc12x10unormsrgb,
            TextureFormat::Astc {
                block: AstcBlock::B12x12,
                channel: AstcChannel::Unorm,
            } => GpuTextureFormat::Astc12x12unorm,
            TextureFormat::Astc {
                block: AstcBlock::B12x12,
                channel: AstcChannel::UnormSrgb,
            } => GpuTextureFormat::Astc12x12unormsrgb,
            _ => panic!("Texture format not supported"),
        }
    }
}

impl Into<TextureDimension> for GpuTextureDimension {
    fn into(self) -> TextureDimension {
        match self {
            GpuTextureDimension::D1 => TextureDimension::D1,
            GpuTextureDimension::D2 => TextureDimension::D2,
            GpuTextureDimension::D3 => TextureDimension::D3,
        }
    }
}

impl Into<GpuTextureDimension> for TextureDimension {
    fn into(self) -> GpuTextureDimension {
        match self {
            TextureDimension::D1 => GpuTextureDimension::D1,
            TextureDimension::D2 => GpuTextureDimension::D2,
            TextureDimension::D3 => GpuTextureDimension::D3,
        }
    }
}

impl Into<StencilOperation> for GpuStencilOperation {
    fn into(self) -> StencilOperation {
        match self {
            GpuStencilOperation::Keep => StencilOperation::Keep,
            GpuStencilOperation::Zero => StencilOperation::Zero,
            GpuStencilOperation::Replace => StencilOperation::Replace,
            GpuStencilOperation::Invert => StencilOperation::Invert,
            GpuStencilOperation::IncrementClamp => StencilOperation::IncrementClamp,
            GpuStencilOperation::DecrementClamp => StencilOperation::DecrementClamp,
            GpuStencilOperation::IncrementWrap => StencilOperation::IncrementWrap,
            GpuStencilOperation::DecrementWrap => StencilOperation::DecrementWrap,
        }
    }
}

impl Into<LoadOp> for GpuLoadOp {
    fn into(self) -> LoadOp {
        match self {
            GpuLoadOp::Clear => LoadOp::Clear,
            GpuLoadOp::Load => LoadOp::Load,
        }
    }
}

impl Into<GpuLoadOp> for LoadOp {
    fn into(self) -> GpuLoadOp {
        match self {
            LoadOp::Clear => GpuLoadOp::Clear,
            LoadOp::Load => GpuLoadOp::Load,
        }
    }
}

impl Into<StoreOp> for GpuStoreOp {
    fn into(self) -> StoreOp {
        match self {
            GpuStoreOp::Discard => StoreOp::Discard,
            GpuStoreOp::Store => StoreOp::Store,
        }
    }
}

impl Into<GpuStoreOp> for StoreOp {
    fn into(self) -> GpuStoreOp {
        match self {
            StoreOp::Discard => GpuStoreOp::Discard,
            StoreOp::Store => GpuStoreOp::Store,
        }
    }
}

impl Into<StencilFaceState> for GpuStencilFaceState {
    fn into(self) -> StencilFaceState {
        StencilFaceState {
            compare: match self.compare {
                None => CompareFunction::Always,
                Some(compare) => compare.into(),
            },
            fail_op: match self.fail_op {
                None => StencilOperation::default(),
                Some(fail_op) => fail_op.into(),
            },
            depth_fail_op: match self.depth_fail_op {
                None => StencilOperation::default(),
                Some(depth_fail_op) => depth_fail_op.into(),
            },
            pass_op: match self.pass_op {
                None => StencilOperation::default(),
                Some(pass_op) => pass_op.into(),
            },
        }
    }
}

impl Into<BlendState> for GpuBlendState {
    fn into(self) -> BlendState {
        BlendState {
            color: self.color.into(),
            alpha: self.alpha.into(),
        }
    }
}

impl Into<BlendComponent> for GpuBlendComponent {
    fn into(self) -> BlendComponent {
        BlendComponent {
            src_factor: match self.src_factor {
                None => BlendFactor::One,
                Some(src_factor) => src_factor.into(),
            },
            dst_factor: match self.dst_factor {
                None => BlendFactor::Zero,
                Some(src_factor) => src_factor.into(),
            },
            operation: match self.operation {
                None => BlendOperation::Add,
                Some(operation) => operation.into(),
            },
        }
    }
}

impl Into<BlendFactor> for GpuBlendFactor {
    fn into(self) -> BlendFactor {
        match self {
            GpuBlendFactor::Zero => BlendFactor::Zero,
            GpuBlendFactor::One => BlendFactor::One,
            GpuBlendFactor::Src => BlendFactor::Src,
            GpuBlendFactor::OneMinusSrc => BlendFactor::OneMinusSrc,
            GpuBlendFactor::SrcAlpha => BlendFactor::SrcAlpha,
            GpuBlendFactor::OneMinusSrcAlpha => BlendFactor::OneMinusSrcAlpha,
            GpuBlendFactor::Dst => BlendFactor::Dst,
            GpuBlendFactor::OneMinusDst => BlendFactor::OneMinusDst,
            GpuBlendFactor::DstAlpha => BlendFactor::DstAlpha,
            GpuBlendFactor::OneMinusDstAlpha => BlendFactor::OneMinusDstAlpha,
            GpuBlendFactor::SrcAlphaSaturated => BlendFactor::SrcAlphaSaturated,
            GpuBlendFactor::Constant => BlendFactor::Constant,
            GpuBlendFactor::OneMinusConstant => BlendFactor::OneMinusConstant,
        }
    }
}

impl Into<BlendOperation> for GpuBlendOperation {
    fn into(self) -> BlendOperation {
        match self {
            GpuBlendOperation::Add => BlendOperation::Add,
            GpuBlendOperation::Subtract => BlendOperation::Subtract,
            GpuBlendOperation::ReverseSubtract => BlendOperation::ReverseSubtract,
            GpuBlendOperation::Min => BlendOperation::Min,
            GpuBlendOperation::Max => BlendOperation::Max,
        }
    }
}

impl Into<ColorWrites> for GpuColorWrite {
    fn into(self) -> ColorWrites {
        let mut cw = ColorWrites::empty();
        if self.contains(GpuColorWrite::RED) {
            cw |= ColorWrites::RED;
        }
        if self.contains(GpuColorWrite::GREEN) {
            cw |= ColorWrites::GREEN;
        }
        if self.contains(GpuColorWrite::BLUE) {
            cw |= ColorWrites::BLUE;
        }
        if self.contains(GpuColorWrite::ALPHA) {
            cw |= ColorWrites::ALPHA;
        }

        if self.contains(GpuColorWrite::ALL) {
            cw = ColorWrites::all();
        }

        cw
    }
}

impl Into<BufferUsages> for GpuBufferUsage {
    fn into(self) -> BufferUsages {
        let mut cw = BufferUsages::empty();

        if self.contains(GpuBufferUsage::MAP_READ) {
            cw |= BufferUsages::MAP_READ;
        }
        if self.contains(GpuBufferUsage::MAP_WRITE) {
            cw |= BufferUsages::MAP_WRITE;
        }
        if self.contains(GpuBufferUsage::COPY_SRC) {
            cw |= BufferUsages::COPY_SRC;
        }
        if self.contains(GpuBufferUsage::COPY_DST) {
            cw |= BufferUsages::COPY_DST;
        }
        if self.contains(GpuBufferUsage::INDEX) {
            cw |= BufferUsages::INDEX;
        }
        if self.contains(GpuBufferUsage::VERTEX) {
            cw |= BufferUsages::VERTEX;
        }
        if self.contains(GpuBufferUsage::UNIFORM) {
            cw |= BufferUsages::UNIFORM;
        }
        if self.contains(GpuBufferUsage::INDIRECT) {
            cw |= BufferUsages::INDIRECT;
        }
        if self.contains(GpuBufferUsage::QUERY_RESOLVE) {
            cw |= BufferUsages::QUERY_RESOLVE;
        }
        if self.contains(GpuBufferUsage::COPY_DST) {
            cw |= BufferUsages::COPY_DST;
        }

        cw
    }
}

impl Into<GpuBufferUsage> for BufferUsages {
    fn into(self) -> GpuBufferUsage {
        let mut cw = GpuBufferUsage::empty();

        if self.contains(BufferUsages::MAP_READ) {
            cw |= GpuBufferUsage::MAP_READ;
        }
        if self.contains(BufferUsages::MAP_WRITE) {
            cw |= GpuBufferUsage::MAP_WRITE;
        }
        if self.contains(BufferUsages::COPY_SRC) {
            cw |= GpuBufferUsage::COPY_SRC;
        }
        if self.contains(BufferUsages::COPY_DST) {
            cw |= GpuBufferUsage::COPY_DST;
        }
        if self.contains(BufferUsages::INDEX) {
            cw |= GpuBufferUsage::INDEX;
        }
        if self.contains(BufferUsages::VERTEX) {
            cw |= GpuBufferUsage::VERTEX;
        }
        if self.contains(BufferUsages::UNIFORM) {
            cw |= GpuBufferUsage::UNIFORM;
        }
        if self.contains(BufferUsages::INDIRECT) {
            cw |= GpuBufferUsage::INDIRECT;
        }
        if self.contains(BufferUsages::QUERY_RESOLVE) {
            cw |= GpuBufferUsage::QUERY_RESOLVE;
        }
        if self.contains(BufferUsages::COPY_DST) {
            cw |= GpuBufferUsage::COPY_DST;
        }

        cw
    }
}

impl Into<TextureUsages> for GpuTextureUsage {
    fn into(self) -> TextureUsages {
        let mut cw = TextureUsages::empty();
        if self.contains(GpuTextureUsage::COPY_SRC) {
            cw |= TextureUsages::COPY_SRC;
        }
        if self.contains(GpuTextureUsage::COPY_DST) {
            cw |= TextureUsages::COPY_DST;
        }
        if self.contains(GpuTextureUsage::TEXTURE_BINDING) {
            cw |= TextureUsages::TEXTURE_BINDING;
        }
        if self.contains(GpuTextureUsage::STORAGE_BINDING) {
            cw |= TextureUsages::STORAGE_BINDING;
        }
        if self.contains(GpuTextureUsage::RENDER_ATTACHMENT) {
            cw |= TextureUsages::RENDER_ATTACHMENT;
        }

        cw
    }
}

impl Into<GpuTextureUsage> for TextureUsages {
    fn into(self) -> GpuTextureUsage {
        let mut cw = GpuTextureUsage::empty();
        if self.contains(TextureUsages::COPY_SRC) {
            cw |= GpuTextureUsage::COPY_SRC;
        }
        if self.contains(TextureUsages::COPY_DST) {
            cw |= GpuTextureUsage::COPY_DST;
        }
        if self.contains(TextureUsages::TEXTURE_BINDING) {
            cw |= GpuTextureUsage::TEXTURE_BINDING;
        }
        if self.contains(TextureUsages::STORAGE_BINDING) {
            cw |= GpuTextureUsage::STORAGE_BINDING;
        }
        if self.contains(TextureUsages::RENDER_ATTACHMENT) {
            cw |= GpuTextureUsage::RENDER_ATTACHMENT;
        }

        cw
    }
}

impl Into<QueryType> for GpuQueryType {
    fn into(self) -> QueryType {
        match self {
            GpuQueryType::Occlusion => QueryType::Occlusion,
            GpuQueryType::Timestamp => QueryType::Timestamp,
        }
    }
}

impl Into<GpuQueryType> for QueryType {
    fn into(self) -> GpuQueryType {
        match self {
            QueryType::Occlusion => GpuQueryType::Occlusion,
            QueryType::Timestamp => GpuQueryType::Timestamp,
            _ => panic!("Query type not supported"),
        }
    }
}

impl Into<BufferMapOperation> for GpuMapMode {
    fn into(self) -> BufferMapOperation {
        let host = {
            if self.contains(GpuMapMode::READ) {
                HostMap::Read
            } else {
                HostMap::Write
            }
        };

        BufferMapOperation {
            host,
            callback: None,
        }
    }
}

pub fn convert_bind_group_entry(
    resource_table: &ResourceTable,
    entry: GpuBindGroupEntry,
) -> BindGroupEntry {
    BindGroupEntry {
        binding: entry.binding,
        resource: match entry.resource {
            crate::gpu::GpuBindingResource::Buffer(buffer_binding) => {
                BindingResource::Buffer(BufferBinding {
                    buffer_id: resource_table
                        .get(&buffer_binding.buffer)
                        .unwrap()
                        .to_owned(),
                    offset: buffer_binding.offset,
                    size: buffer_binding.size.map(|s| NonZeroU64::new(s).unwrap()),
                })
            }
            crate::gpu::GpuBindingResource::BufferArray(buffers) => {
                BindingResource::BufferArray(Cow::Owned(
                    buffers
                        .iter()
                        .map(|b| BufferBinding {
                            buffer_id: resource_table.get(&b.buffer).unwrap().to_owned(),
                            offset: b.offset,
                            size: b.size.map(|s| NonZeroU64::new(s).unwrap()),
                        })
                        .collect(),
                ))
            }
            crate::gpu::GpuBindingResource::Sampler(sampler) => {
                BindingResource::Sampler(resource_table.get(&sampler).unwrap().to_owned())
            }
            crate::gpu::GpuBindingResource::SamplerArray(samplers) => {
                BindingResource::SamplerArray(Cow::Owned(
                    samplers
                        .iter()
                        .map(|s| resource_table.get(&s).unwrap().to_owned())
                        .collect(),
                ))
            }
            crate::gpu::GpuBindingResource::TextureView(texture_view) => {
                BindingResource::TextureView(resource_table.get(&texture_view).unwrap().to_owned())
            }
            crate::gpu::GpuBindingResource::TextureViewArray(texture_views) => {
                BindingResource::TextureViewArray(Cow::Owned(
                    texture_views
                        .iter()
                        .map(|v| resource_table.get(&v).unwrap().to_owned())
                        .collect(),
                ))
            }
        },
    }
}

impl Into<crate::audio::BiquadFilterType> for web_audio_api::node::BiquadFilterType {
    fn into(self) -> crate::audio::BiquadFilterType {
        match self {
            web_audio_api::node::BiquadFilterType::Allpass => {
                crate::audio::BiquadFilterType::Allpass
            }
            web_audio_api::node::BiquadFilterType::Lowpass => {
                crate::audio::BiquadFilterType::Lowpass
            }
            web_audio_api::node::BiquadFilterType::Highpass => {
                crate::audio::BiquadFilterType::Highpass
            }
            web_audio_api::node::BiquadFilterType::Bandpass => {
                crate::audio::BiquadFilterType::Bandpass
            }
            web_audio_api::node::BiquadFilterType::Notch => crate::audio::BiquadFilterType::Notch,
            web_audio_api::node::BiquadFilterType::Peaking => {
                crate::audio::BiquadFilterType::Peaking
            }
            web_audio_api::node::BiquadFilterType::Lowshelf => {
                crate::audio::BiquadFilterType::Lowshelf
            }
            web_audio_api::node::BiquadFilterType::Highshelf => {
                crate::audio::BiquadFilterType::Highshelf
            }
        }
    }
}

impl Into<web_audio_api::node::BiquadFilterType> for crate::audio::BiquadFilterType {
    fn into(self) -> web_audio_api::node::BiquadFilterType {
        match self {
            crate::audio::BiquadFilterType::Allpass => {
                web_audio_api::node::BiquadFilterType::Allpass
            }
            crate::audio::BiquadFilterType::Lowpass => {
                web_audio_api::node::BiquadFilterType::Lowpass
            }
            crate::audio::BiquadFilterType::Highpass => {
                web_audio_api::node::BiquadFilterType::Highpass
            }
            crate::audio::BiquadFilterType::Bandpass => {
                web_audio_api::node::BiquadFilterType::Bandpass
            }
            crate::audio::BiquadFilterType::Notch => web_audio_api::node::BiquadFilterType::Notch,
            crate::audio::BiquadFilterType::Peaking => {
                web_audio_api::node::BiquadFilterType::Peaking
            }
            crate::audio::BiquadFilterType::Lowshelf => {
                web_audio_api::node::BiquadFilterType::Lowshelf
            }
            crate::audio::BiquadFilterType::Highshelf => {
                web_audio_api::node::BiquadFilterType::Highshelf
            }
        }
    }
}

impl Into<crate::audio::AutomationRate> for web_audio_api::AutomationRate {
    fn into(self) -> crate::audio::AutomationRate {
        match self {
            web_audio_api::AutomationRate::A => crate::audio::AutomationRate::A,
            web_audio_api::AutomationRate::K => crate::audio::AutomationRate::K,
        }
    }
}

impl Into<web_audio_api::AutomationRate> for crate::audio::AutomationRate {
    fn into(self) -> web_audio_api::AutomationRate {
        match self {
            crate::audio::AutomationRate::A => web_audio_api::AutomationRate::A,
            crate::audio::AutomationRate::K => web_audio_api::AutomationRate::K,
        }
    }
}

impl Into<crate::audio::OscillatorType> for web_audio_api::node::OscillatorType {
    fn into(self) -> crate::audio::OscillatorType {
        match self {
            web_audio_api::node::OscillatorType::Sine => crate::audio::OscillatorType::Sine,
            web_audio_api::node::OscillatorType::Square => crate::audio::OscillatorType::Square,
            web_audio_api::node::OscillatorType::Sawtooth => crate::audio::OscillatorType::Sawtooth,
            web_audio_api::node::OscillatorType::Triangle => crate::audio::OscillatorType::Triangle,
            web_audio_api::node::OscillatorType::Custom => crate::audio::OscillatorType::Custom,
        }
    }
}

impl Into<web_audio_api::node::OscillatorType> for crate::audio::OscillatorType {
    fn into(self) -> web_audio_api::node::OscillatorType {
        match self {
            crate::audio::OscillatorType::Sine => web_audio_api::node::OscillatorType::Sine,
            crate::audio::OscillatorType::Square => web_audio_api::node::OscillatorType::Square,
            crate::audio::OscillatorType::Sawtooth => web_audio_api::node::OscillatorType::Sawtooth,
            crate::audio::OscillatorType::Triangle => web_audio_api::node::OscillatorType::Triangle,
            crate::audio::OscillatorType::Custom => web_audio_api::node::OscillatorType::Custom,
        }
    }
}

impl Into<crate::audio::DistanceModelType> for web_audio_api::node::DistanceModelType {
    fn into(self) -> crate::audio::DistanceModelType {
        match self {
            web_audio_api::node::DistanceModelType::Linear => {
                crate::audio::DistanceModelType::Linear
            }
            web_audio_api::node::DistanceModelType::Inverse => {
                crate::audio::DistanceModelType::Inverse
            }
            web_audio_api::node::DistanceModelType::Exponential => {
                crate::audio::DistanceModelType::Exponential
            }
        }
    }
}

impl Into<web_audio_api::node::DistanceModelType> for crate::audio::DistanceModelType {
    fn into(self) -> web_audio_api::node::DistanceModelType {
        match self {
            crate::audio::DistanceModelType::Linear => {
                web_audio_api::node::DistanceModelType::Linear
            }
            crate::audio::DistanceModelType::Inverse => {
                web_audio_api::node::DistanceModelType::Inverse
            }
            crate::audio::DistanceModelType::Exponential => {
                web_audio_api::node::DistanceModelType::Exponential
            }
        }
    }
}

impl Into<web_audio_api::node::PanningModelType> for crate::audio::PanningModelType {
    fn into(self) -> web_audio_api::node::PanningModelType {
        match self {
            crate::audio::PanningModelType::EqualPower => {
                web_audio_api::node::PanningModelType::EqualPower
            }
            crate::audio::PanningModelType::Hrtf => web_audio_api::node::PanningModelType::HRTF,
        }
    }
}

impl Into<crate::audio::PanningModelType> for web_audio_api::node::PanningModelType {
    fn into(self) -> crate::audio::PanningModelType {
        match self {
            web_audio_api::node::PanningModelType::EqualPower => {
                crate::audio::PanningModelType::EqualPower
            }
            web_audio_api::node::PanningModelType::HRTF => crate::audio::PanningModelType::Hrtf,
        }
    }
}

impl Into<web_audio_api::node::OverSampleType> for crate::audio::OverSampleType {
    fn into(self) -> web_audio_api::node::OverSampleType {
        match self {
            crate::audio::OverSampleType::None => web_audio_api::node::OverSampleType::None,
            crate::audio::OverSampleType::X2 => web_audio_api::node::OverSampleType::X2,
            crate::audio::OverSampleType::X4 => web_audio_api::node::OverSampleType::X4,
        }
    }
}

impl Into<crate::audio::OverSampleType> for web_audio_api::node::OverSampleType {
    fn into(self) -> crate::audio::OverSampleType {
        match self {
            web_audio_api::node::OverSampleType::None => crate::audio::OverSampleType::None,
            web_audio_api::node::OverSampleType::X2 => crate::audio::OverSampleType::X2,
            web_audio_api::node::OverSampleType::X4 => crate::audio::OverSampleType::X4,
        }
    }
}

impl Into<web_audio_api::PeriodicWaveOptions> for crate::audio::PeriodicWaveOptions {
    fn into(self) -> web_audio_api::PeriodicWaveOptions {
        web_audio_api::PeriodicWaveOptions {
            real: self.real,
            imag: self.imag,
            disable_normalization: self.disable_normalization,
        }
    }
}

impl Into<crate::audio::PeriodicWaveOptions> for web_audio_api::PeriodicWaveOptions {
    fn into(self) -> crate::audio::PeriodicWaveOptions {
        crate::audio::PeriodicWaveOptions {
            real: self.real,
            imag: self.imag,
            disable_normalization: self.disable_normalization,
        }
    }
}

impl Into<crate::audio::AudioContextState> for web_audio_api::context::AudioContextState {
    fn into(self) -> crate::audio::AudioContextState {
        match self {
            web_audio_api::context::AudioContextState::Suspended => {
                crate::audio::AudioContextState::Suspended
            }
            web_audio_api::context::AudioContextState::Running => {
                crate::audio::AudioContextState::Running
            }
            web_audio_api::context::AudioContextState::Closed => {
                crate::audio::AudioContextState::Closed
            }
        }
    }
}

impl Into<web_audio_api::context::AudioContextState> for crate::audio::AudioContextState {
    fn into(self) -> web_audio_api::context::AudioContextState {
        match self {
            crate::audio::AudioContextState::Suspended => {
                web_audio_api::context::AudioContextState::Suspended
            }
            crate::audio::AudioContextState::Running => {
                web_audio_api::context::AudioContextState::Running
            }
            crate::audio::AudioContextState::Closed => {
                web_audio_api::context::AudioContextState::Closed
            }
        }
    }
}
