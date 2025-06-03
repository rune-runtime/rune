use std::path::PathBuf;

use anyhow::{Ok, Result};
use slab::Slab;
use vfs::VfsPath;
use wgpu_types::TextureFormat;
use winit::keyboard::{Key, KeyLocation};

pub mod host;
pub mod runtime;
pub mod tests;
// pub mod debug;

// needed for wasmtime::component::bindgen! as it only looks in the current crate.
pub(crate) use gilrs;
pub(crate) use wgpu_core;

pub type BufferSource = Vec<u8>;

pub fn wgpu_id<I: wgpu_core::id::Marker, E>(
    (id, error): (wgpu_core::id::Id<I>, Option<E>),
) -> Result<wgpu_core::id::Id<I>, E> {
    match error {
        Some(error) => Err(error),
        None => core::result::Result::Ok(id),
    }
}

wasmtime::component::bindgen!({
    world: "rune:runtime/runtime",
    path: "wit/runtime",
    async: true,
    with: {
        "rune:runtime/audio/audio-buffer": web_audio_api::AudioBuffer,
        "rune:runtime/audio/audio-context": web_audio_api::context::AudioContext,
        "rune:runtime/audio/audio-param": web_audio_api::AudioParam,
        "rune:runtime/audio/analyzer-node": web_audio_api::node::AnalyserNode,
        "rune:runtime/audio/audio-buffer-source-node": web_audio_api::node::AudioBufferSourceNode,
        "rune:runtime/audio/audio-destination-node": web_audio_api::node::AudioDestinationNode,
        "rune:runtime/audio/biquad-filter-node": web_audio_api::node::BiquadFilterNode,
        "rune:runtime/audio/constant-source-node": web_audio_api::node::ConstantSourceNode,
        "rune:runtime/audio/convolver-node": web_audio_api::node::ConvolverNode,
        "rune:runtime/audio/channel-merger-node": web_audio_api::node::ChannelMergerNode,
        "rune:runtime/audio/channel-splitter-node": web_audio_api::node::ChannelSplitterNode,
        "rune:runtime/audio/delay-node": web_audio_api::node::DelayNode,
        "rune:runtime/audio/dynamics-compressor-node": web_audio_api::node::DynamicsCompressorNode,
        "rune:runtime/audio/gain-node": web_audio_api::node::GainNode,
        "rune:runtime/audio/iir-filter-node": web_audio_api::node::IIRFilterNode,
        "rune:runtime/audio/oscillator-node": web_audio_api::node::OscillatorNode,
        "rune:runtime/audio/panner-node": web_audio_api::node::PannerNode,
        "rune:runtime/audio/audio-render-capacity": web_audio_api::AudioRenderCapacity,
        "rune:runtime/audio/stereo-panner-node": web_audio_api::node::StereoPannerNode,
        "rune:runtime/audio/wave-shaper-node": web_audio_api::node::WaveShaperNode,
        "rune:runtime/audio/audio-listener": web_audio_api::AudioListener,

        "rune:runtime/gpu/gpu-adapter": wgpu_core::id::AdapterId,
        "rune:runtime/gpu/gpu-device": wgpu_core::id::DeviceId,
        "rune:runtime/gpu/gpu-queue": wgpu_core::id::QueueId,
        "rune:runtime/gpu/gpu-buffer": wgpu_core::id::BufferId,
        "rune:runtime/gpu/gpu-command-encoder": wgpu_core::id::CommandEncoderId,
        "rune:runtime/gpu/gpu-compute-pass-encoder": wgpu_core::command::ComputePass,
        "rune:runtime/gpu/gpu-render-pass-encoder": wgpu_core::command::RenderPass,
        "rune:runtime/gpu/gpu-render-bundle": wgpu_core::id::RenderBundleId,
        "rune:runtime/gpu/gpu-render-bundle-encoder": wgpu_core::command::RenderBundleEncoder,
        "rune:runtime/gpu/gpu-shader-module": wgpu_core::id::ShaderModuleId,
        "rune:runtime/gpu/gpu-bind-group": wgpu_core::id::BindGroupId,
        "rune:runtime/gpu/gpu-bind-group-layout": wgpu_core::id::BindGroupLayoutId,
        "rune:runtime/gpu/gpu-pipeline-layout": wgpu_core::id::PipelineLayoutId,
        "rune:runtime/gpu/gpu-compute-pipeline": wgpu_core::id::ComputePipelineId,
        "rune:runtime/gpu/gpu-render-pipeline": wgpu_core::id::RenderPipelineId,
        "rune:runtime/gpu/gpu-command-buffer": wgpu_core::id::CommandBufferId,
        "rune:runtime/gpu/gpu-sampler": wgpu_core::id::SamplerId,
        "rune:runtime/gpu/gpu-texture": wgpu_core::id::TextureId,
        "rune:runtime/gpu/gpu-texture-view": wgpu_core::id::TextureViewId,
        "rune:runtime/gpu/gpu-query-set": wgpu_core::id::QuerySetId,
        // "rune:runtime/gpu/buffer-source": BufferSource,

        "rune:runtime/input/gamepad-device": gilrs::GamepadId,

        "rune:runtime/network/network-client": crate::runtime::network::NetworkClient,
        "rune:runtime/network/network-server": crate::runtime::network::NetworkServer,
        "rune:runtime/network/network-connection": crate::runtime::network::NetworkConnection,
        "rune:runtime/network/network-http-client": crate::runtime::network::NetworkHttpClient,
    }
});

// wasmtime::component::bindgen!({
//     world: "rune:tests/tests",
//     path: "wit/tests",
//     with: {
//         "rune:runtime/debug": rune::runtime::debug
//     }
// });

pub use exports::rune::runtime::guest;
use gilrs::{Button, Gilrs};
pub use rune::runtime::*;
use uuid::Uuid;
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};
use winit::dpi::PhysicalSize;

pub use runtime::RuneRuntimeState;

impl WasiView for RuneRuntimeState {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }

    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.wasi_ctx
    }
}

pub struct KeyboardState {
    pub active_keys: Vec<(u64, Key, KeyLocation)>,
}

impl KeyboardState {
    pub fn new() -> KeyboardState {
        Self {
            active_keys: Vec::new(),
        }
    }
}

pub struct GamepadState {
    pub active_buttons: Vec<(u64, Button)>,
}

impl GamepadState {
    pub fn new() -> GamepadState {
        Self {
            active_buttons: Vec::new(),
        }
    }
}


