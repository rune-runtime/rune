use js_sys::{Array, ArrayBuffer, Object, Promise, Reflect, Uint8Array};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    gpu::{
        Gpu, GpuAdapter, GpuBindGroup, GpuBindGroupDescriptor, GpuBindGroupLayout,
        GpuBindGroupLayoutDescriptor, GpuBuffer, GpuBufferDescriptor, GpuBufferMapState,
        GpuCommandBuffer, GpuCommandBufferDescriptor, GpuCommandEncoder, GpuCommandEncoderDescriptor, GpuCompilationInfo,
        GpuComputePassDescriptor, GpuComputePassEncoder, GpuComputePipeline,
        GpuComputePipelineDescriptor, GpuDevice, GpuDeviceDescriptor, GpuExtent3dDict,
        GpuImageCopyBuffer, GpuImageCopyTexture, GpuImageDataLayout, GpuIndexFormat, GpuMapModeFlags,
        GpuPipelineLayout, GpuPipelineLayoutDescriptor, GpuQuerySet, GpuQuerySetDescriptor, GpuQueryType,
        GpuQueue, GpuRenderBundle, GpuRenderBundleDescriptor, GpuRenderBundleEncoder, GpuRenderBundleEncoderDescriptor, GpuRenderPassDescriptor,
        GpuRenderPassEncoder, GpuRenderPipeline, GpuRenderPipelineDescriptor,
        GpuRequestAdapterOptions, GpuSampler, GpuSamplerDescriptor, GpuShaderModule,
        GpuShaderModuleDescriptor, GpuTexture, GpuTextureDescriptor, GpuTextureDimension, GpuTextureFormat, GpuTextureView,
        GpuTextureViewDescriptor, GpuColorDict,
    },
    Navigator, Window,
};

// Helper to get the navigator.gpu object
fn get_navigator_gpu() -> Result<Gpu, JsValue> {
    let window: Window = web_sys::window()
        .ok_or_else(|| JsValue::from_str("Failed to get window object"))?;
    let navigator: Navigator = window.navigator();

    // Check if 'gpu' property exists on navigator
    let gpu_property = Reflect::get(&navigator, &JsValue::from_str("gpu"))?;
    if gpu_property.is_undefined() || gpu_property.is_null() {
        return Err(JsValue::from_str(
            "WebGPU not supported or not enabled in this browser.",
        ));
    }
    gpu_property
        .dyn_into::<Gpu>()
        .map_err(|_| JsValue::from_str("Failed to cast navigator.gpu to web_sys::gpu::Gpu type."))
}

#[wasm_bindgen]
async fn gpu_request_adapter() -> Result<JsValue, JsValue> {
    let gpu = get_navigator_gpu()?;
    // Use default options for now
    let options = GpuRequestAdapterOptions::new();
    JsFuture::from(gpu.request_adapter_with_options(&options)).await
}

#[wasm_bindgen]
async fn gpu_adapter_request_device(adapter_js: JsValue) -> Result<JsValue, JsValue> {
    let adapter = adapter_js.dyn_into::<GpuAdapter>().map_err(|_| {
        JsValue::from_str("Failed to cast to GpuAdapter. Ensure a valid GPUAdapter is passed.")
    })?;
    // Use default descriptor for now
    let descriptor = GpuDeviceDescriptor::new();
    JsFuture::from(adapter.request_device_with_descriptor(&descriptor)).await
}

#[wasm_bindgen]
fn gpu_device_get_queue(device_js: JsValue) -> Result<JsValue, JsValue> {
    let device = device_js.dyn_into::<GpuDevice>().map_err(|_| {
        JsValue::from_str("Failed to cast to GpuDevice. Ensure a valid GPUDevice is passed.")
    })?;
    // device.queue is a direct property returning GpuQueue
    Ok(device.queue().into())
}

#[wasm_bindgen]
fn gpu_device_create_buffer(device_js: JsValue, descriptor_js: JsValue) -> Result<JsValue, JsValue> {
    let device = device_js.dyn_into::<GpuDevice>().map_err(|_| {
        JsValue::from_str("Failed to cast to GpuDevice for createBuffer.")
    })?;

    // GpuBufferDescriptor is a typedef for JsValue representing a JS object.
    // The dyn_into call will ensure it's a JsValue, and the browser's WebGPU
    // implementation will validate its structure.
    let descriptor: GpuBufferDescriptor = descriptor_js.dyn_into().map_err(|_| {
        JsValue::from_str(
            "Failed to cast descriptor to GpuBufferDescriptor type for createBuffer.",
        )
    })?;

    let buffer: GpuBuffer = device.create_buffer_with_descriptor(&descriptor);
    Ok(buffer.into())
}

#[wasm_bindgen]
fn gpu_device_create_texture(device_js: JsValue, descriptor_js: JsValue) -> Result<JsValue, JsValue> {
    let device = device_js.dyn_into::<GpuDevice>().map_err(|_| {
        JsValue::from_str("Failed to cast to GpuDevice for createTexture.")
    })?;
    let descriptor: GpuTextureDescriptor = descriptor_js.dyn_into().map_err(|_| {
        JsValue::from_str(
            "Failed to cast descriptor to GpuTextureDescriptor type for createTexture.",
        )
    })?;
    let texture: GpuTexture = device.create_texture_with_descriptor(&descriptor);
    Ok(texture.into())
}

#[wasm_bindgen]
fn gpu_device_create_sampler(device_js: JsValue, descriptor_js: JsValue) -> Result<JsValue, JsValue> {
    let device = device_js.dyn_into::<GpuDevice>().map_err(|_| {
        JsValue::from_str("Failed to cast to GpuDevice for createSampler.")
    })?;
    let descriptor: GpuSamplerDescriptor = descriptor_js.dyn_into().map_err(|_| {
        JsValue::from_str(
            "Failed to cast descriptor to GpuSamplerDescriptor type for createSampler.",
        )
    })?;
    let sampler: GpuSampler = device.create_sampler_with_descriptor(&descriptor);
    Ok(sampler.into())
}

#[wasm_bindgen]
fn gpu_device_create_shader_module(device_js: JsValue, descriptor_js: JsValue) -> Result<JsValue, JsValue> {
    let device = device_js.dyn_into::<GpuDevice>().map_err(|_| {
        JsValue::from_str("Failed to cast to GpuDevice for createShaderModule.")
    })?;
    let descriptor: GpuShaderModuleDescriptor = descriptor_js.dyn_into().map_err(|_| {
        JsValue::from_str(
            "Failed to cast descriptor to GpuShaderModuleDescriptor type for createShaderModule.",
        )
    })?;
    let shader_module: GpuShaderModule = device.create_shader_module_with_descriptor(&descriptor);
    Ok(shader_module.into())
}

#[wasm_bindgen]
fn gpu_device_create_bind_group_layout(device_js: JsValue, descriptor_js: JsValue) -> Result<JsValue, JsValue> {
    let device = device_js.dyn_into::<GpuDevice>().map_err(|_| {
        JsValue::from_str("Failed to cast to GpuDevice for createBindGroupLayout.")
    })?;
    let descriptor: GpuBindGroupLayoutDescriptor = descriptor_js.dyn_into().map_err(|_| {
        JsValue::from_str(
            "Failed to cast descriptor to GpuBindGroupLayoutDescriptor type for createBindGroupLayout.",
        )
    })?;
    let layout: GpuBindGroupLayout = device.create_bind_group_layout_with_descriptor(&descriptor);
    Ok(layout.into())
}

#[wasm_bindgen]
fn gpu_device_create_bind_group(device_js: JsValue, descriptor_js: JsValue) -> Result<JsValue, JsValue> {
    let device = device_js.dyn_into::<GpuDevice>().map_err(|_| {
        JsValue::from_str("Failed to cast to GpuDevice for createBindGroup.")
    })?;
    let descriptor: GpuBindGroupDescriptor = descriptor_js.dyn_into().map_err(|_| {
        JsValue::from_str(
            "Failed to cast descriptor to GpuBindGroupDescriptor type for createBindGroup.",
        )
    })?;
    let bind_group: GpuBindGroup = device.create_bind_group_with_descriptor(&descriptor);
    Ok(bind_group.into())
}

#[wasm_bindgen]
fn gpu_device_create_pipeline_layout(device_js: JsValue, descriptor_js: JsValue) -> Result<JsValue, JsValue> {
    let device = device_js.dyn_into::<GpuDevice>().map_err(|_| {
        JsValue::from_str("Failed to cast to GpuDevice for createPipelineLayout.")
    })?;
    let descriptor: GpuPipelineLayoutDescriptor = descriptor_js.dyn_into().map_err(|_| {
        JsValue::from_str(
            "Failed to cast descriptor to GpuPipelineLayoutDescriptor type for createPipelineLayout.",
        )
    })?;
    let layout: GpuPipelineLayout = device.create_pipeline_layout_with_descriptor(&descriptor);
    Ok(layout.into())
}

#[wasm_bindgen]
fn gpu_device_create_command_encoder(device_js: JsValue, descriptor_js: Option<JsValue>) -> Result<JsValue, JsValue> {
    let device = device_js.dyn_into::<GpuDevice>().map_err(|_| {
        JsValue::from_str("Failed to cast to GpuDevice for createCommandEncoder.")
    })?;
    let encoder: GpuCommandEncoder = match descriptor_js {
        Some(desc_js) => {
            let descriptor: GpuCommandEncoderDescriptor = desc_js.dyn_into().map_err(|_|{
                JsValue::from_str("Failed to cast descriptor to GpuCommandEncoderDescriptor type for createCommandEncoder.")
            })?;
            device.create_command_encoder_with_descriptor(&descriptor)
        }
        None => device.create_command_encoder(),
    };
    Ok(encoder.into())
}

#[wasm_bindgen]
async fn gpu_device_create_compute_pipeline(device_js: JsValue, descriptor_js: JsValue) -> Result<JsValue, JsValue> {
    let device = device_js.dyn_into::<GpuDevice>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuDevice for createComputePipeline.")
    )?;
    let descriptor: GpuComputePipelineDescriptor = descriptor_js.dyn_into().map_err(|_|
        JsValue::from_str("Failed to cast descriptor to GpuComputePipelineDescriptor for createComputePipeline.")
    )?;
    JsFuture::from(device.create_compute_pipeline_async_with_descriptor(&descriptor)).await
}

#[wasm_bindgen]
async fn gpu_device_create_render_pipeline(device_js: JsValue, descriptor_js: JsValue) -> Result<JsValue, JsValue> {
    let device = device_js.dyn_into::<GpuDevice>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuDevice for createRenderPipeline.")
    )?;
    let descriptor: GpuRenderPipelineDescriptor = descriptor_js.dyn_into().map_err(|_|
        JsValue::from_str("Failed to cast descriptor to GpuRenderPipelineDescriptor for createRenderPipeline.")
    )?;
    JsFuture::from(device.create_render_pipeline_async_with_descriptor(&descriptor)).await
}

#[wasm_bindgen]
fn gpu_device_create_render_bundle_encoder(device_js: JsValue, descriptor_js: JsValue) -> Result<JsValue, JsValue> {
    let device = device_js.dyn_into::<GpuDevice>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuDevice for createRenderBundleEncoder.")
    )?;
    let descriptor: GpuRenderBundleEncoderDescriptor = descriptor_js.dyn_into().map_err(|_|
        JsValue::from_str("Failed to cast descriptor to GpuRenderBundleEncoderDescriptor for createRenderBundleEncoder.")
    )?;
    let encoder: GpuRenderBundleEncoder = device.create_render_bundle_encoder_with_descriptor(&descriptor);
    Ok(encoder.into())
}

#[wasm_bindgen]
fn gpu_device_create_query_set(device_js: JsValue, descriptor_js: JsValue) -> Result<JsValue, JsValue> {
    let device = device_js.dyn_into::<GpuDevice>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuDevice for createQuerySet.")
    )?;
    let descriptor: GpuQuerySetDescriptor = descriptor_js.dyn_into().map_err(|_|
        JsValue::from_str("Failed to cast descriptor to GpuQuerySetDescriptor for createQuerySet.")
    )?;
    let query_set: GpuQuerySet = device.create_query_set_with_descriptor(&descriptor);
    Ok(query_set.into())
}

#[wasm_bindgen]
fn gpu_queue_submit(queue_js: JsValue, command_buffers_js: JsValue) -> Result<(), JsValue> {
    let queue = queue_js.dyn_into::<GpuQueue>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuQueue for submit.")
    )?;
    // command_buffers_js is expected to be a JS array of GpuCommandBuffer objects.
    // GpuQueue.submit() takes a sequence<GPUCommandBuffer>, which web_sys represents as &JsValue (for an array).
    queue.submit_with_command_buffers(&command_buffers_js);
    Ok(())
}

#[wasm_bindgen]
fn gpu_queue_write_buffer(
    queue_js: JsValue,
    buffer_js: JsValue,
    buffer_offset: f64, // WIT uses u64, WebGPU uses unsigned long long (f64 in JS/web_sys)
    data_js: JsValue,   // WIT uses list<u8>, WebGPU uses BufferSource (ArrayBufferView or ArrayBuffer)
    data_offset: f64, // WIT uses u64, WebGPU uses unsigned long long (f64 in JS/web_sys)
    size: f64,        // WIT uses u64, WebGPU uses unsigned long long (f64 in JS/web_sys)
) -> Result<(), JsValue> {
    let queue = queue_js.dyn_into::<GpuQueue>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuQueue for writeBuffer.")
    )?;
    let buffer = buffer_js.dyn_into::<GpuBuffer>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuBuffer for writeBuffer.")
    )?;

    // data_js is expected to be an ArrayBuffer or ArrayBufferView (e.g., Uint8Array).
    // The web_sys binding for write_buffer takes &JsValue for the data source.
    queue.write_buffer_with_u32_and_buffer_source_and_u32_and_u32(
        &buffer,
        buffer_offset,
        &data_js,
        data_offset, // dataOffset in spec
        size,        // size in spec
    );
    Ok(())
}

#[wasm_bindgen]
fn gpu_queue_write_texture(
    queue_js: JsValue,
    destination_js: JsValue, // WIT: gpu-image-copy-texture, web_sys: GpuImageCopyTexture (dictionary)
    data_js: JsValue,        // WIT: buffer-source (list<u8>), web_sys: Uint8Array for this specific binding
    data_layout_js: JsValue, // WIT: gpu-image-data-layout, web_sys: GpuImageDataLayout (dictionary)
    size_js: JsValue,        // WIT: gpu-extent-d3, web_sys: GpuExtent3dDict (dictionary)
) -> Result<(), JsValue> {
    let queue = queue_js.dyn_into::<GpuQueue>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuQueue for writeTexture.")
    )?;

    let destination: GpuImageCopyTexture = destination_js.dyn_into().map_err(|_|
        JsValue::from_str("Failed to cast destination to GpuImageCopyTexture for writeTexture.")
    )?;
    
    // data_js needs to be a Uint8Array for this specific web_sys binding
    let data_array = data_js.dyn_into::<Uint8Array>().map_err(|_|
        JsValue::from_str("data for writeTexture must be a Uint8Array.")
    )?;
    let data_slice = data_array.to_vec(); // Get a Vec<u8>, web_sys wants &[u8]

    let data_layout: GpuImageDataLayout = data_layout_js.dyn_into().map_err(|_|
        JsValue::from_str("Failed to cast data_layout to GpuImageDataLayout for writeTexture.")
    )?;
    let size: GpuExtent3dDict = size_js.dyn_into().map_err(|_|
        JsValue::from_str("Failed to cast size to GpuExtent3dDict for writeTexture.")
    )?;

    queue.write_texture_with_image_copy_texture_and_u8_array_and_image_data_layout_and_extent_3d_dict(
        &destination,
        &data_slice, // web_sys binding takes &[u8]
        &data_layout,
        &size,
    );
    Ok(())
}

#[wasm_bindgen]
fn gpu_buffer_size(buffer_js: JsValue) -> Result<f64, JsValue> {
    let buffer = buffer_js.dyn_into::<GpuBuffer>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuBuffer for size.")
    )?;
    Ok(buffer.size())
}

#[wasm_bindgen]
fn gpu_buffer_usage(buffer_js: JsValue) -> Result<u32, JsValue> {
    let buffer = buffer_js.dyn_into::<GpuBuffer>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuBuffer for usage.")
    )?;
    Ok(buffer.usage())
}

#[wasm_bindgen]
fn gpu_buffer_map_state(buffer_js: JsValue) -> Result<JsValue, JsValue> {
    let buffer = buffer_js.dyn_into::<GpuBuffer>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuBuffer for mapState.")
    )?;
    Ok(buffer.map_state().into())
}

#[wasm_bindgen]
async fn gpu_buffer_map_async(
    buffer_js: JsValue,
    mode: u32, // Corresponds to GpuMapModeFlags (GPUMapMode.READ or GPUMapMode.WRITE)
    offset: f64,
    size: f64,
) -> Result<JsValue, JsValue> {
    let buffer = buffer_js.dyn_into::<GpuBuffer>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuBuffer for mapAsync.")
    )?;
    JsFuture::from(buffer.map_async_with_u32_and_f64_and_f64(mode, offset, size)).await
}

#[wasm_bindgen]
fn gpu_buffer_get_mapped_range(
    buffer_js: JsValue,
    offset: f64,
    size: f64,
) -> Result<ArrayBuffer, JsValue> {
    let buffer = buffer_js.dyn_into::<GpuBuffer>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuBuffer for getMappedRange.")
    )?;
    Ok(buffer.get_mapped_range_with_f64_and_f64(offset, size))
}

#[wasm_bindgen]
fn gpu_buffer_unmap(buffer_js: JsValue) -> Result<(), JsValue> {
    let buffer = buffer_js.dyn_into::<GpuBuffer>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuBuffer for unmap.")
    )?;
    buffer.unmap();
    Ok(())
}

#[wasm_bindgen]
fn gpu_buffer_destroy(buffer_js: JsValue) -> Result<(), JsValue> {
    let buffer = buffer_js.dyn_into::<GpuBuffer>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuBuffer for destroy.")
    )?;
    buffer.destroy();
    Ok(())
}

#[wasm_bindgen]
fn gpu_texture_create_view(texture_js: JsValue, descriptor_js: Option<JsValue>) -> Result<JsValue, JsValue> {
    let texture = texture_js.dyn_into::<GpuTexture>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuTexture for createView.")
    )?;
    match descriptor_js {
        Some(desc_js) => {
            let descriptor: GpuTextureViewDescriptor = desc_js.dyn_into().map_err(|_|
                JsValue::from_str("Failed to cast descriptor to GpuTextureViewDescriptor for createView.")
            )?;
            let view: GpuTextureView = texture.create_view_with_descriptor(&descriptor);
            Ok(view.into())
        }
        None => {
            let view: GpuTextureView = texture.create_view();
            Ok(view.into())
        }
    }
}

#[wasm_bindgen]
fn gpu_texture_destroy(texture_js: JsValue) -> Result<(), JsValue> {
    let texture = texture_js.dyn_into::<GpuTexture>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuTexture for destroy.")
    )?;
    texture.destroy();
    Ok(())
}

#[wasm_bindgen]
async fn gpu_shader_module_get_compilation_info(shader_module_js: JsValue) -> Result<JsValue, JsValue> {
    let shader_module = shader_module_js.dyn_into::<GpuShaderModule>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuShaderModule for getCompilationInfo.")
    )?;
    JsFuture::from(shader_module.get_compilation_info()).await
}

#[wasm_bindgen]
fn gpu_compute_pipeline_get_bind_group_layout(pipeline_js: JsValue, index: u32) -> Result<JsValue, JsValue> {
    let pipeline = pipeline_js.dyn_into::<GpuComputePipeline>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuComputePipeline for getBindGroupLayout.")
    )?;
    let layout: GpuBindGroupLayout = pipeline.get_bind_group_layout(index);
    Ok(layout.into())
}

#[wasm_bindgen]
fn gpu_render_pipeline_get_bind_group_layout(pipeline_js: JsValue, index: u32) -> Result<JsValue, JsValue> {
    let pipeline = pipeline_js.dyn_into::<GpuRenderPipeline>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuRenderPipeline for getBindGroupLayout.")
    )?;
    let layout: GpuBindGroupLayout = pipeline.get_bind_group_layout(index);
    Ok(layout.into())
}

#[wasm_bindgen]
fn gpu_command_encoder_begin_render_pass(encoder_js: JsValue, descriptor_js: JsValue) -> Result<JsValue, JsValue> {
    let encoder = encoder_js.dyn_into::<GpuCommandEncoder>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuCommandEncoder for beginRenderPass.")
    )?;
    let descriptor: GpuRenderPassDescriptor = descriptor_js.dyn_into().map_err(|_|
        JsValue::from_str("Failed to cast descriptor to GpuRenderPassDescriptor for beginRenderPass.")
    )?;
    let pass_encoder: GpuRenderPassEncoder = encoder.begin_render_pass(&descriptor);
    Ok(pass_encoder.into())
}

#[wasm_bindgen]
fn gpu_command_encoder_begin_compute_pass(encoder_js: JsValue, descriptor_js: Option<JsValue>) -> Result<JsValue, JsValue> {
    let encoder = encoder_js.dyn_into::<GpuCommandEncoder>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuCommandEncoder for beginComputePass.")
    )?;
    let pass_encoder: GpuComputePassEncoder = match descriptor_js {
        Some(desc_js) => {
            let descriptor: GpuComputePassDescriptor = desc_js.dyn_into().map_err(|_|
                JsValue::from_str("Failed to cast descriptor to GpuComputePassDescriptor for beginComputePass.")
            )?;
            encoder.begin_compute_pass_with_descriptor(&descriptor)
        }
        None => encoder.begin_compute_pass(),
    };
    Ok(pass_encoder.into())
}

#[wasm_bindgen]
fn gpu_command_encoder_copy_buffer_to_buffer(
    encoder_js: JsValue,
    source_buffer_js: JsValue,
    source_offset: f64,
    destination_buffer_js: JsValue,
    destination_offset: f64,
    size: f64,
) -> Result<(), JsValue> {
    let encoder = encoder_js.dyn_into::<GpuCommandEncoder>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuCommandEncoder for copyBufferToBuffer.")
    )?;
    let source_buffer = source_buffer_js.dyn_into::<GpuBuffer>().map_err(|_|
        JsValue::from_str("Failed to cast source to GpuBuffer for copyBufferToBuffer.")
    )?;
    let destination_buffer = destination_buffer_js.dyn_into::<GpuBuffer>().map_err(|_|
        JsValue::from_str("Failed to cast destination to GpuBuffer for copyBufferToBuffer.")
    )?;
    encoder.copy_buffer_to_buffer_with_u32_and_u32_and_u32(
        &source_buffer,
        source_offset,
        &destination_buffer,
        destination_offset,
        size,
    );
    Ok(())
}

#[wasm_bindgen]
fn gpu_command_encoder_copy_buffer_to_texture(
    encoder_js: JsValue,
    source_js: JsValue, // GpuImageCopyBuffer
    destination_js: JsValue, // GpuImageCopyTexture
    copy_size_js: JsValue, // GpuExtent3dDict
) -> Result<(), JsValue> {
    let encoder = encoder_js.dyn_into::<GpuCommandEncoder>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuCommandEncoder for copyBufferToTexture.")
    )?;
    let source: GpuImageCopyBuffer = source_js.dyn_into().map_err(|_|
        JsValue::from_str("Failed to cast source to GpuImageCopyBuffer for copyBufferToTexture.")
    )?;
    let destination: GpuImageCopyTexture = destination_js.dyn_into().map_err(|_|
        JsValue::from_str("Failed to cast destination to GpuImageCopyTexture for copyBufferToTexture.")
    )?;
    let copy_size: GpuExtent3dDict = copy_size_js.dyn_into().map_err(|_|
        JsValue::from_str("Failed to cast copy_size to GpuExtent3dDict for copyBufferToTexture.")
    )?;
    encoder.copy_buffer_to_texture_with_image_copy_buffer_and_image_copy_texture_and_extent_3d_dict(
        &source,
        &destination,
        &copy_size,
    );
    Ok(())
}

#[wasm_bindgen]
fn gpu_command_encoder_copy_texture_to_buffer(
    encoder_js: JsValue,
    source_js: JsValue, // GpuImageCopyTexture
    destination_js: JsValue, // GpuImageCopyBuffer
    copy_size_js: JsValue, // GpuExtent3dDict
) -> Result<(), JsValue> {
    let encoder = encoder_js.dyn_into::<GpuCommandEncoder>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuCommandEncoder for copyTextureToBuffer.")
    )?;
    let source: GpuImageCopyTexture = source_js.dyn_into().map_err(|_|
        JsValue::from_str("Failed to cast source to GpuImageCopyTexture for copyTextureToBuffer.")
    )?;
    let destination: GpuImageCopyBuffer = destination_js.dyn_into().map_err(|_|
        JsValue::from_str("Failed to cast destination to GpuImageCopyBuffer for copyTextureToBuffer.")
    )?;
    let copy_size: GpuExtent3dDict = copy_size_js.dyn_into().map_err(|_|
        JsValue::from_str("Failed to cast copy_size to GpuExtent3dDict for copyTextureToBuffer.")
    )?;
    encoder.copy_texture_to_buffer_with_image_copy_texture_and_image_copy_buffer_and_extent_3d_dict(
        &source,
        &destination,
        &copy_size,
    );
    Ok(())
}

#[wasm_bindgen]
fn gpu_command_encoder_copy_texture_to_texture(
    encoder_js: JsValue,
    source_js: JsValue, // GpuImageCopyTexture
    destination_js: JsValue, // GpuImageCopyTexture
    copy_size_js: JsValue, // GpuExtent3dDict
) -> Result<(), JsValue> {
    let encoder = encoder_js.dyn_into::<GpuCommandEncoder>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuCommandEncoder for copyTextureToTexture.")
    )?;
    let source: GpuImageCopyTexture = source_js.dyn_into().map_err(|_|
        JsValue::from_str("Failed to cast source to GpuImageCopyTexture for copyTextureToTexture.")
    )?;
    let destination: GpuImageCopyTexture = destination_js.dyn_into().map_err(|_|
        JsValue::from_str("Failed to cast destination to GpuImageCopyTexture for copyTextureToTexture.")
    )?;
    let copy_size: GpuExtent3dDict = copy_size_js.dyn_into().map_err(|_|
        JsValue::from_str("Failed to cast copy_size to GpuExtent3dDict for copyTextureToTexture.")
    )?;
    encoder.copy_texture_to_texture_with_image_copy_texture_and_image_copy_texture_and_extent_3d_dict(
        &source,
        &destination,
        &copy_size,
    );
    Ok(())
}

#[wasm_bindgen]
fn gpu_command_encoder_clear_buffer(
    encoder_js: JsValue,
    buffer_js: JsValue,
    offset_js: Option<f64>,
    size_js: Option<f64>,
) -> Result<(), JsValue> {
    let encoder = encoder_js.dyn_into::<GpuCommandEncoder>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuCommandEncoder for clearBuffer.")
    )?;
    let buffer = buffer_js.dyn_into::<GpuBuffer>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuBuffer for clearBuffer.")
    )?;

    match (offset_js, size_js) {
        (Some(offset), Some(size)) => {
            encoder.clear_buffer_with_buffer_and_f64_and_f64(&buffer, offset, size);
        }
        (Some(offset), None) => {
            encoder.clear_buffer_with_buffer_and_f64(&buffer, offset);
        }
        (None, None) => {
            encoder.clear_buffer_with_buffer(&buffer);
        }
        (None, Some(_)) => {
            // Size without offset is not a valid combination for the web_sys clear_buffer methods
            return Err(JsValue::from_str("clearBuffer: size provided without offset is not supported."));
        }
    }
    Ok(())
}

#[wasm_bindgen]
fn gpu_command_encoder_write_timestamp(
    encoder_js: JsValue,
    query_set_js: JsValue,
    query_index: u32,
) -> Result<(), JsValue> {
    let encoder = encoder_js.dyn_into::<GpuCommandEncoder>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuCommandEncoder for writeTimestamp.")
    )?;
    let query_set = query_set_js.dyn_into::<GpuQuerySet>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuQuerySet for writeTimestamp.")
    )?;
    encoder.write_timestamp_with_query_set_and_u32(&query_set, query_index);
    Ok(())
}

#[wasm_bindgen]
fn gpu_command_encoder_resolve_query_set(
    encoder_js: JsValue,
    query_set_js: JsValue,
    first_query: u32,
    query_count: u32,
    destination_buffer_js: JsValue,
    destination_offset: f64,
) -> Result<(), JsValue> {
    let encoder = encoder_js.dyn_into::<GpuCommandEncoder>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuCommandEncoder for resolveQuerySet.")
    )?;
    let query_set = query_set_js.dyn_into::<GpuQuerySet>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuQuerySet for resolveQuerySet.")
    )?;
    let destination_buffer = destination_buffer_js.dyn_into::<GpuBuffer>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuBuffer for resolveQuerySet.")
    )?;
    encoder.resolve_query_set_with_query_set_and_u32_and_u32_and_buffer_and_u32(
        &query_set,
        first_query,
        query_count,
        &destination_buffer,
        destination_offset,
    );
    Ok(())
}

#[wasm_bindgen]
fn gpu_command_encoder_finish(encoder_js: JsValue, descriptor_js: Option<JsValue>) -> Result<JsValue, JsValue> {
    let encoder = encoder_js.dyn_into::<GpuCommandEncoder>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuCommandEncoder for finish.")
    )?;
    let command_buffer: GpuCommandBuffer = match descriptor_js {
        Some(desc_js) => {
            let descriptor: GpuCommandBufferDescriptor = desc_js.dyn_into().map_err(|_|
                JsValue::from_str("Failed to cast descriptor to GpuCommandBufferDescriptor for finish.")
            )?;
            encoder.finish_with_descriptor(&descriptor)
        }
        None => encoder.finish(),
    };
    Ok(command_buffer.into())
}

#[wasm_bindgen]
fn gpu_compute_pass_encoder_set_pipeline(pass_encoder_js: JsValue, pipeline_js: JsValue) -> Result<(), JsValue> {
    let pass_encoder = pass_encoder_js.dyn_into::<GpuComputePassEncoder>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuComputePassEncoder for setPipeline.")
    )?;
    let pipeline = pipeline_js.dyn_into::<GpuComputePipeline>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuComputePipeline for setPipeline.")
    )?;
    pass_encoder.set_pipeline(&pipeline);
    Ok(())
}

#[wasm_bindgen]
fn gpu_compute_pass_encoder_dispatch_workgroups(
    pass_encoder_js: JsValue,
    workgroup_count_x: u32,
    workgroup_count_y: Option<u32>,
    workgroup_count_z: Option<u32>,
) -> Result<(), JsValue> {
    let pass_encoder = pass_encoder_js.dyn_into::<GpuComputePassEncoder>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuComputePassEncoder for dispatchWorkgroups.")
    )?;
    pass_encoder.dispatch_workgroups_with_workgroup_count_y_and_workgroup_count_z(
        workgroup_count_x,
        workgroup_count_y.unwrap_or(1),
        workgroup_count_z.unwrap_or(1),
    );
    Ok(())
}

#[wasm_bindgen]
fn gpu_compute_pass_encoder_dispatch_workgroups_indirect(
    pass_encoder_js: JsValue,
    indirect_buffer_js: JsValue,
    indirect_offset: f64,
) -> Result<(), JsValue> {
    let pass_encoder = pass_encoder_js.dyn_into::<GpuComputePassEncoder>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuComputePassEncoder for dispatchWorkgroupsIndirect.")
    )?;
    let indirect_buffer = indirect_buffer_js.dyn_into::<GpuBuffer>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuBuffer for dispatchWorkgroupsIndirect.")
    )?;
    pass_encoder.dispatch_workgroups_indirect(&indirect_buffer, indirect_offset);
    Ok(())
}

#[wasm_bindgen]
fn gpu_compute_pass_encoder_end(pass_encoder_js: JsValue) -> Result<(), JsValue> {
    let pass_encoder = pass_encoder_js.dyn_into::<GpuComputePassEncoder>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuComputePassEncoder for end.")
    )?;
    pass_encoder.end();
    Ok(())
}

#[wasm_bindgen]
fn gpu_compute_pass_encoder_set_bind_group(
    pass_encoder_js: JsValue,
    index: u32,
    bind_group_js: Option<JsValue>,
    dynamic_offsets_js: Option<JsValue>, // Expected to be a JS array of numbers (u32)
) -> Result<(), JsValue> {
    let pass_encoder = pass_encoder_js.dyn_into::<GpuComputePassEncoder>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuComputePassEncoder for setBindGroup.")
    )?;
    let bind_group = match bind_group_js {
        Some(bg_js) => Some(bg_js.dyn_into::<GpuBindGroup>().map_err(|_|
            JsValue::from_str("Failed to cast bindGroup to GpuBindGroup.")
        )?),
        None => None,
    };

    // The web_sys binding set_bind_group_with_u32_array takes &JsValue for dynamic_offsets, which should be an array.
    // If dynamic_offsets_js is None, we should pass an empty array or undefined, depending on how web_sys handles it.
    // Passing an empty array if None is safer.
    let offsets_val = match dynamic_offsets_js {
        Some(val) => val,
        None => Array::new().into(), // Create an empty JS array
    };

    pass_encoder.set_bind_group_with_u32_array(index, bind_group.as_ref(), &offsets_val);
    Ok(())
}

#[wasm_bindgen]
fn gpu_compute_pass_encoder_set_bind_group_with_data(
    pass_encoder_js: JsValue,
    index: u32,
    bind_group_js: Option<JsValue>,
    dynamic_offsets_data_js: JsValue, // Expected to be Uint32Array
    dynamic_offsets_data_start: u32, // WIT: gpu-size-u64, web-sys: u32
    dynamic_offsets_data_length: u32,
) -> Result<(), JsValue> {
    let pass_encoder = pass_encoder_js.dyn_into::<GpuComputePassEncoder>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuComputePassEncoder for setBindGroupWithData.")
    )?;
    let bind_group = match bind_group_js {
        Some(bg_js) => Some(bg_js.dyn_into::<GpuBindGroup>().map_err(|_|
            JsValue::from_str("Failed to cast bindGroup to GpuBindGroup for setBindGroupWithData.")
        )?),
        None => None,
    };
    let dynamic_offsets_data = dynamic_offsets_data_js.dyn_into::<Uint32Array>().map_err(|_|
        JsValue::from_str("dynamic_offsets_data must be a Uint32Array for setBindGroupWithData.")
    )?;

    pass_encoder.set_bind_group_with_u32_array_and_u32_and_u32(
        index,
        bind_group.as_ref(),
        &dynamic_offsets_data,
        dynamic_offsets_data_start, // This is an element count, not byte offset
        dynamic_offsets_data_length,
    );
    Ok(())
}

#[wasm_bindgen]
fn gpu_compute_pass_encoder_push_debug_group(pass_encoder_js: JsValue, group_label: String) -> Result<(), JsValue> {
    let pass_encoder = pass_encoder_js.dyn_into::<GpuComputePassEncoder>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuComputePassEncoder for pushDebugGroup.")
    )?;
    pass_encoder.push_debug_group(&group_label);
    Ok(())
}

#[wasm_bindgen]
fn gpu_compute_pass_encoder_pop_debug_group(pass_encoder_js: JsValue) -> Result<(), JsValue> {
    let pass_encoder = pass_encoder_js.dyn_into::<GpuComputePassEncoder>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuComputePassEncoder for popDebugGroup.")
    )?;
    pass_encoder.pop_debug_group();
    Ok(())
}

#[wasm_bindgen]
fn gpu_compute_pass_encoder_insert_debug_marker(pass_encoder_js: JsValue, marker_label: String) -> Result<(), JsValue> {
    let pass_encoder = pass_encoder_js.dyn_into::<GpuComputePassEncoder>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuComputePassEncoder for insertDebugMarker.")
    )?;
    pass_encoder.insert_debug_marker(&marker_label);
    Ok(())
}

#[wasm_bindgen]
fn gpu_render_pass_encoder_set_pipeline(pass_encoder_js: JsValue, pipeline_js: JsValue) -> Result<(), JsValue> {
    let pass_encoder = pass_encoder_js.dyn_into::<GpuRenderPassEncoder>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuRenderPassEncoder for setPipeline.")
    )?;
    let pipeline = pipeline_js.dyn_into::<GpuRenderPipeline>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuRenderPipeline for setPipeline.")
    )?;
    pass_encoder.set_pipeline(&pipeline);
    Ok(())
}

#[wasm_bindgen]
fn gpu_render_pass_encoder_set_index_buffer(
    pass_encoder_js: JsValue,
    buffer_js: JsValue,
    index_format_js: JsValue, // String: "uint16" or "uint32"
    offset: f64,
    size: Option<f64>,
) -> Result<(), JsValue> {
    let pass_encoder = pass_encoder_js.dyn_into::<GpuRenderPassEncoder>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuRenderPassEncoder for setIndexBuffer.")
    )?;
    let buffer = buffer_js.dyn_into::<GpuBuffer>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuBuffer for setIndexBuffer.")
    )?;
    let index_format: GpuIndexFormat = GpuIndexFormat::from_js_value(&index_format_js).map_err(|_|
        JsValue::from_str("Invalid indexFormat string for setIndexBuffer.")
    )?;

    if let Some(s) = size {
        pass_encoder.set_index_buffer_with_f64_and_f64(&buffer, index_format, offset, s);
    } else {
        pass_encoder.set_index_buffer_with_f64(&buffer, index_format, offset);
    }
    Ok(())
}

#[wasm_bindgen]
fn gpu_render_pass_encoder_set_vertex_buffer(
    pass_encoder_js: JsValue,
    slot: u32,
    buffer_js: JsValue,
    offset: f64,
    size: Option<f64>,
) -> Result<(), JsValue> {
    let pass_encoder = pass_encoder_js.dyn_into::<GpuRenderPassEncoder>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuRenderPassEncoder for setVertexBuffer.")
    )?;
    let buffer = buffer_js.dyn_into::<GpuBuffer>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuBuffer for setVertexBuffer.")
    )?;

    if let Some(s) = size {
        pass_encoder.set_vertex_buffer_with_f64_and_f64(slot, &buffer, offset, s);
    } else {
        pass_encoder.set_vertex_buffer_with_f64(slot, &buffer, offset);
    }
    Ok(())
}

#[wasm_bindgen]
fn gpu_render_pass_encoder_draw(
    pass_encoder_js: JsValue,
    vertex_count: u32,
    instance_count: Option<u32>,
    first_vertex: Option<u32>,
    first_instance: Option<u32>,
) -> Result<(), JsValue> {
    let pass_encoder = pass_encoder_js.dyn_into::<GpuRenderPassEncoder>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuRenderPassEncoder for draw.")
    )?;
    pass_encoder.draw_with_instance_count_and_first_vertex_and_first_instance(
        vertex_count,
        instance_count.unwrap_or(1),
        first_vertex.unwrap_or(0),
        first_instance.unwrap_or(0),
    );
    Ok(())
}

#[wasm_bindgen]
fn gpu_render_pass_encoder_draw_indexed(
    pass_encoder_js: JsValue,
    index_count: u32,
    instance_count: Option<u32>,
    first_index: Option<u32>,
    base_vertex: Option<i32>,
    first_instance: Option<u32>,
) -> Result<(), JsValue> {
    let pass_encoder = pass_encoder_js.dyn_into::<GpuRenderPassEncoder>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuRenderPassEncoder for drawIndexed.")
    )?;
    pass_encoder.draw_indexed_with_instance_count_and_first_index_and_base_vertex_and_first_instance(
        index_count,
        instance_count.unwrap_or(1),
        first_index.unwrap_or(0),
        base_vertex.unwrap_or(0),
        first_instance.unwrap_or(0),
    );
    Ok(())
}

#[wasm_bindgen]
fn gpu_render_pass_encoder_draw_indirect(
    pass_encoder_js: JsValue,
    indirect_buffer_js: JsValue,
    indirect_offset: f64,
) -> Result<(), JsValue> {
    let pass_encoder = pass_encoder_js.dyn_into::<GpuRenderPassEncoder>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuRenderPassEncoder for drawIndirect.")
    )?;
    let indirect_buffer = indirect_buffer_js.dyn_into::<GpuBuffer>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuBuffer for drawIndirect.")
    )?;
    pass_encoder.draw_indirect(&indirect_buffer, indirect_offset);
    Ok(())
}

#[wasm_bindgen]
fn gpu_render_pass_encoder_draw_indexed_indirect(
    pass_encoder_js: JsValue,
    indirect_buffer_js: JsValue,
    indirect_offset: f64,
) -> Result<(), JsValue> {
    let pass_encoder = pass_encoder_js.dyn_into::<GpuRenderPassEncoder>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuRenderPassEncoder for drawIndexedIndirect.")
    )?;
    let indirect_buffer = indirect_buffer_js.dyn_into::<GpuBuffer>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuBuffer for drawIndexedIndirect.")
    )?;
    pass_encoder.draw_indexed_indirect(&indirect_buffer, indirect_offset);
    Ok(())
}

#[wasm_bindgen]
fn gpu_render_pass_encoder_set_viewport(
    pass_encoder_js: JsValue,
    x: f32, y: f32, width: f32, height: f32,
    min_depth: f32, max_depth: f32,
) -> Result<(), JsValue> {
    let pass_encoder = pass_encoder_js.dyn_into::<GpuRenderPassEncoder>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuRenderPassEncoder for setViewport.")
    )?;
    pass_encoder.set_viewport(x, y, width, height, min_depth, max_depth);
    Ok(())
}

#[wasm_bindgen]
fn gpu_render_pass_encoder_set_scissor_rect(
    pass_encoder_js: JsValue,
    x: u32, y: u32, width: u32, height: u32,
) -> Result<(), JsValue> {
    let pass_encoder = pass_encoder_js.dyn_into::<GpuRenderPassEncoder>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuRenderPassEncoder for setScissorRect.")
    )?;
    pass_encoder.set_scissor_rect(x, y, width, height);
    Ok(())
}

#[wasm_bindgen]
fn gpu_render_pass_encoder_set_blend_constant(pass_encoder_js: JsValue, color_js: JsValue) -> Result<(), JsValue> {
    let pass_encoder = pass_encoder_js.dyn_into::<GpuRenderPassEncoder>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuRenderPassEncoder for setBlendConstant.")
    )?;
    // color_js is expected to be a sequence<double> (JS array of numbers)
    // or GpuColorDict. JsValue can represent the sequence directly.
    pass_encoder.set_blend_constant_with_color_sequence(&color_js);
    Ok(())
}

#[wasm_bindgen]
fn gpu_render_pass_encoder_set_stencil_reference(pass_encoder_js: JsValue, reference: u32) -> Result<(), JsValue> {
    let pass_encoder = pass_encoder_js.dyn_into::<GpuRenderPassEncoder>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuRenderPassEncoder for setStencilReference.")
    )?;
    pass_encoder.set_stencil_reference(reference);
    Ok(())
}

#[wasm_bindgen]
fn gpu_render_pass_encoder_begin_occlusion_query(pass_encoder_js: JsValue, query_index: u32) -> Result<(), JsValue> {
    let pass_encoder = pass_encoder_js.dyn_into::<GpuRenderPassEncoder>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuRenderPassEncoder for beginOcclusionQuery.")
    )?;
    pass_encoder.begin_occlusion_query(query_index);
    Ok(())
}

#[wasm_bindgen]
fn gpu_render_pass_encoder_end_occlusion_query(pass_encoder_js: JsValue) -> Result<(), JsValue> {
    let pass_encoder = pass_encoder_js.dyn_into::<GpuRenderPassEncoder>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuRenderPassEncoder for endOcclusionQuery.")
    )?;
    pass_encoder.end_occlusion_query();
    Ok(())
}

#[wasm_bindgen]
fn gpu_render_pass_encoder_execute_bundles(pass_encoder_js: JsValue, bundles_js: JsValue) -> Result<(), JsValue> {
    let pass_encoder = pass_encoder_js.dyn_into::<GpuRenderPassEncoder>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuRenderPassEncoder for executeBundles.")
    )?;
    // bundles_js is expected to be a sequence<GPURenderBundle> (JS array of GPURenderBundle objects)
    pass_encoder.execute_bundles(&bundles_js);
    Ok(())
}

#[wasm_bindgen]
fn gpu_render_pass_encoder_end(pass_encoder_js: JsValue) -> Result<(), JsValue> {
    let pass_encoder = pass_encoder_js.dyn_into::<GpuRenderPassEncoder>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuRenderPassEncoder for end.")
    )?;
    pass_encoder.end();
    Ok(())
}

#[wasm_bindgen]
fn gpu_render_pass_encoder_set_bind_group(
    pass_encoder_js: JsValue,
    index: u32,
    bind_group_js: Option<JsValue>,
    dynamic_offsets_js: Option<JsValue>, // Expected to be a JS array of numbers (u32)
) -> Result<(), JsValue> {
    let pass_encoder = pass_encoder_js.dyn_into::<GpuRenderPassEncoder>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuRenderPassEncoder for setBindGroup.")
    )?;
    let bind_group = match bind_group_js {
        Some(bg_js) => Some(bg_js.dyn_into::<GpuBindGroup>().map_err(|_|
            JsValue::from_str("Failed to cast bindGroup to GpuBindGroup.")
        )?),
        None => None,
    };
    let offsets_val = match dynamic_offsets_js {
        Some(val) => val,
        None => Array::new().into(),
    };
    pass_encoder.set_bind_group_with_u32_array(index, bind_group.as_ref(), &offsets_val);
    Ok(())
}

#[wasm_bindgen]
fn gpu_render_pass_encoder_set_bind_group_with_data(
    pass_encoder_js: JsValue,
    index: u32,
    bind_group_js: Option<JsValue>,
    dynamic_offsets_data_js: JsValue, // Expected to be Uint32Array
    dynamic_offsets_data_start: u32,
    dynamic_offsets_data_length: u32,
) -> Result<(), JsValue> {
    let pass_encoder = pass_encoder_js.dyn_into::<GpuRenderPassEncoder>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuRenderPassEncoder for setBindGroupWithData.")
    )?;
    let bind_group = match bind_group_js {
        Some(bg_js) => Some(bg_js.dyn_into::<GpuBindGroup>().map_err(|_|
            JsValue::from_str("Failed to cast bindGroup to GpuBindGroup for setBindGroupWithData.")
        )?),
        None => None,
    };
    let dynamic_offsets_data = dynamic_offsets_data_js.dyn_into::<Uint32Array>().map_err(|_|
        JsValue::from_str("dynamic_offsets_data must be a Uint32Array for setBindGroupWithData.")
    )?;

    pass_encoder.set_bind_group_with_u32_array_and_u32_and_u32(
        index,
        bind_group.as_ref(),
        &dynamic_offsets_data,
        dynamic_offsets_data_start,
        dynamic_offsets_data_length,
    );
    Ok(())
}

#[wasm_bindgen]
fn gpu_render_pass_encoder_push_debug_group(pass_encoder_js: JsValue, group_label: String) -> Result<(), JsValue> {
    let pass_encoder = pass_encoder_js.dyn_into::<GpuRenderPassEncoder>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuRenderPassEncoder for pushDebugGroup.")
    )?;
    pass_encoder.push_debug_group(&group_label);
    Ok(())
}

#[wasm_bindgen]
fn gpu_render_pass_encoder_pop_debug_group(pass_encoder_js: JsValue) -> Result<(), JsValue> {
    let pass_encoder = pass_encoder_js.dyn_into::<GpuRenderPassEncoder>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuRenderPassEncoder for popDebugGroup.")
    )?;
    pass_encoder.pop_debug_group();
    Ok(())
}

#[wasm_bindgen]
fn gpu_render_pass_encoder_insert_debug_marker(pass_encoder_js: JsValue, marker_label: String) -> Result<(), JsValue> {
    let pass_encoder = pass_encoder_js.dyn_into::<GpuRenderPassEncoder>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuRenderPassEncoder for insertDebugMarker.")
    )?;
    pass_encoder.insert_debug_marker(&marker_label);
    Ok(())
}

#[wasm_bindgen]
fn gpu_render_bundle_encoder_finish(encoder_js: JsValue, descriptor_js: JsValue) -> Result<JsValue, JsValue> {
    let encoder = encoder_js.dyn_into::<GpuRenderBundleEncoder>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuRenderBundleEncoder for finish.")
    )?;
    let descriptor: GpuRenderBundleDescriptor = descriptor_js.dyn_into().map_err(|_|
        JsValue::from_str("Failed to cast descriptor to GpuRenderBundleDescriptor for finish.")
    )?;
    let bundle: GpuRenderBundle = encoder.finish_with_descriptor(&descriptor);
    Ok(bundle.into())
}

#[wasm_bindgen]
fn gpu_query_set_type(query_set_js: JsValue) -> Result<JsValue, JsValue> {
    let query_set = query_set_js.dyn_into::<GpuQuerySet>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuQuerySet for type.")
    )?;
    Ok(query_set.type_().into())
}

#[wasm_bindgen]
fn gpu_query_set_count(query_set_js: JsValue) -> Result<u32, JsValue> {
    let query_set = query_set_js.dyn_into::<GpuQuerySet>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuQuerySet for count.")
    )?;
    Ok(query_set.count())
}

#[wasm_bindgen]
fn gpu_query_set_destroy(query_set_js: JsValue) -> Result<(), JsValue> {
    let query_set = query_set_js.dyn_into::<GpuQuerySet>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuQuerySet for destroy.")
    )?;
    query_set.destroy();
    Ok(())
}

#[wasm_bindgen]
fn gpu_texture_get_width(texture_js: JsValue) -> Result<u32, JsValue> {
    let texture = texture_js.dyn_into::<GpuTexture>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuTexture for getWidth.")
    )?;
    Ok(texture.width())
}

#[wasm_bindgen]
fn gpu_texture_get_height(texture_js: JsValue) -> Result<u32, JsValue> {
    let texture = texture_js.dyn_into::<GpuTexture>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuTexture for getHeight.")
    )?;
    Ok(texture.height())
}

#[wasm_bindgen]
fn gpu_texture_get_depth_or_array_layers(texture_js: JsValue) -> Result<u32, JsValue> {
    let texture = texture_js.dyn_into::<GpuTexture>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuTexture for getDepthOrArrayLayers.")
    )?;
    Ok(texture.depth_or_array_layers())
}

#[wasm_bindgen]
fn gpu_texture_get_mip_level_count(texture_js: JsValue) -> Result<u32, JsValue> {
    let texture = texture_js.dyn_into::<GpuTexture>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuTexture for getMipLevelCount.")
    )?;
    Ok(texture.mip_level_count())
}

#[wasm_bindgen]
fn gpu_texture_get_sample_count(texture_js: JsValue) -> Result<u32, JsValue> {
    let texture = texture_js.dyn_into::<GpuTexture>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuTexture for getSampleCount.")
    )?;
    Ok(texture.sample_count())
}

#[wasm_bindgen]
fn gpu_texture_get_dimension(texture_js: JsValue) -> Result<JsValue, JsValue> {
    let texture = texture_js.dyn_into::<GpuTexture>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuTexture for getDimension.")
    )?;
    Ok(texture.dimension().into())
}

#[wasm_bindgen]
fn gpu_texture_get_format(texture_js: JsValue) -> Result<JsValue, JsValue> {
    let texture = texture_js.dyn_into::<GpuTexture>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuTexture for getFormat.")
    )?;
    Ok(texture.format().into())
}

#[wasm_bindgen]
fn gpu_texture_get_usage(texture_js: JsValue) -> Result<u32, JsValue> {
    let texture = texture_js.dyn_into::<GpuTexture>().map_err(|_|
        JsValue::from_str("Failed to cast to GpuTexture for getUsage.")
    )?;
    Ok(texture.usage())
}

pub fn export() -> Result<JsValue, JsValue> {
    let gpu_obj = Object::new();

    let request_adapter_closure =
        Closure::wrap(Box::new(gpu_request_adapter) as Box<dyn FnMut() -> Promise>);
    Reflect::set(
        &gpu_obj,
        &JsValue::from_str("requestAdapter"),
        request_adapter_closure.as_ref().unchecked_ref(),
    )?;
    request_adapter_closure.forget();

    let request_device_closure =
        Closure::wrap(Box::new(gpu_adapter_request_device) as Box<dyn FnMut(JsValue) -> Promise>);
    Reflect::set(
        &gpu_obj,
        &JsValue::from_str("requestDevice"),
        request_device_closure.as_ref().unchecked_ref(),
    )?;
    request_device_closure.forget();

    let get_queue_closure = Closure::wrap(Box::new(gpu_device_get_queue)
        as Box<dyn FnMut(JsValue) -> Result<JsValue, JsValue>>);
    Reflect::set(
        &gpu_obj,
        &JsValue::from_str("getQueue"),
        get_queue_closure.as_ref().unchecked_ref(),
    )?;
    get_queue_closure.forget();

    let create_buffer_closure = Closure::wrap(Box::new(gpu_device_create_buffer)
        as Box<dyn FnMut(JsValue, JsValue) -> Result<JsValue, JsValue>>);
    Reflect::set(
        &gpu_obj,
        &JsValue::from_str("createBuffer"),
        create_buffer_closure.as_ref().unchecked_ref(),
    )?;
    create_buffer_closure.forget();

    let create_texture_closure = Closure::wrap(Box::new(gpu_device_create_texture)
        as Box<dyn FnMut(JsValue, JsValue) -> Result<JsValue, JsValue>>);
    Reflect::set(
        &gpu_obj,
        &JsValue::from_str("createTexture"),
        create_texture_closure.as_ref().unchecked_ref(),
    )?;
    create_texture_closure.forget();

    let create_sampler_closure = Closure::wrap(Box::new(gpu_device_create_sampler)
        as Box<dyn FnMut(JsValue, JsValue) -> Result<JsValue, JsValue>>);
    Reflect::set(
        &gpu_obj,
        &JsValue::from_str("createSampler"),
        create_sampler_closure.as_ref().unchecked_ref(),
    )?;
    create_sampler_closure.forget();

    let create_shader_module_closure = Closure::wrap(Box::new(gpu_device_create_shader_module)
        as Box<dyn FnMut(JsValue, JsValue) -> Result<JsValue, JsValue>>);
    Reflect::set(
        &gpu_obj,
        &JsValue::from_str("createShaderModule"),
        create_shader_module_closure.as_ref().unchecked_ref(),
    )?;
    create_shader_module_closure.forget();

    let create_bind_group_layout_closure = Closure::wrap(Box::new(gpu_device_create_bind_group_layout)
        as Box<dyn FnMut(JsValue, JsValue) -> Result<JsValue, JsValue>>);
    Reflect::set(
        &gpu_obj,
        &JsValue::from_str("createBindGroupLayout"),
        create_bind_group_layout_closure.as_ref().unchecked_ref(),
    )?;
    create_bind_group_layout_closure.forget();

    let create_bind_group_closure = Closure::wrap(Box::new(gpu_device_create_bind_group)
        as Box<dyn FnMut(JsValue, JsValue) -> Result<JsValue, JsValue>>);
    Reflect::set(
        &gpu_obj,
        &JsValue::from_str("createBindGroup"),
        create_bind_group_closure.as_ref().unchecked_ref(),
    )?;
    create_bind_group_closure.forget();

    let create_pipeline_layout_closure = Closure::wrap(Box::new(gpu_device_create_pipeline_layout)
        as Box<dyn FnMut(JsValue, JsValue) -> Result<JsValue, JsValue>>);
    Reflect::set(
        &gpu_obj,
        &JsValue::from_str("createPipelineLayout"),
        create_pipeline_layout_closure.as_ref().unchecked_ref(),
    )?;
    create_pipeline_layout_closure.forget();

    let device_create_command_encoder_closure = Closure::wrap(Box::new(gpu_device_create_command_encoder)
        as Box<dyn FnMut(JsValue, Option<JsValue>) -> Result<JsValue, JsValue>>);
    Reflect::set(
        &gpu_obj,
        &JsValue::from_str("createCommandEncoder"),
        device_create_command_encoder_closure.as_ref().unchecked_ref(),
    )?;
    device_create_command_encoder_closure.forget();

    let create_compute_pipeline_closure = Closure::wrap(Box::new(gpu_device_create_compute_pipeline)
        as Box<dyn FnMut(JsValue, JsValue) -> Promise>);
    Reflect::set(
        &gpu_obj,
        &JsValue::from_str("createComputePipeline"),
        create_compute_pipeline_closure.as_ref().unchecked_ref(),
    )?;
    create_compute_pipeline_closure.forget();

    let create_render_pipeline_closure = Closure::wrap(Box::new(gpu_device_create_render_pipeline)
        as Box<dyn FnMut(JsValue, JsValue) -> Promise>);
    Reflect::set(
        &gpu_obj,
        &JsValue::from_str("createRenderPipeline"),
        create_render_pipeline_closure.as_ref().unchecked_ref(),
    )?;
    create_render_pipeline_closure.forget();

    let device_create_render_bundle_encoder_closure = Closure::wrap(Box::new(gpu_device_create_render_bundle_encoder) // Explicitly gpu_device_ method
        as Box<dyn FnMut(JsValue, JsValue) -> Result<JsValue, JsValue>>);
    Reflect::set(
        &gpu_obj,
        &JsValue::from_str("createRenderBundleEncoder"), // This is GPUDevice.createRenderBundleEncoder
        device_create_render_bundle_encoder_closure.as_ref().unchecked_ref(),
    )?;
    device_create_render_bundle_encoder_closure.forget();

    let create_query_set_closure = Closure::wrap(Box::new(gpu_device_create_query_set)
        as Box<dyn FnMut(JsValue, JsValue) -> Result<JsValue, JsValue>>);
    Reflect::set(
        &gpu_obj,
        &JsValue::from_str("createQuerySet"),
        create_query_set_closure.as_ref().unchecked_ref(),
    )?;
    create_query_set_closure.forget();

    let queue_submit_closure = Closure::wrap(Box::new(gpu_queue_submit)
        as Box<dyn FnMut(JsValue, JsValue) -> Result<(), JsValue>>);
    Reflect::set(
        &gpu_obj,
        &JsValue::from_str("queueSubmit"),
        queue_submit_closure.as_ref().unchecked_ref(),
    )?;
    queue_submit_closure.forget();

    let queue_write_buffer_closure = Closure::wrap(Box::new(gpu_queue_write_buffer)
        as Box<dyn FnMut(JsValue, JsValue, f64, JsValue, f64, f64) -> Result<(), JsValue>>);
    Reflect::set(
        &gpu_obj,
        &JsValue::from_str("queueWriteBuffer"),
        queue_write_buffer_closure.as_ref().unchecked_ref(),
    )?;
    queue_write_buffer_closure.forget();

    let queue_write_texture_closure = Closure::wrap(Box::new(gpu_queue_write_texture)
        as Box<dyn FnMut(JsValue, JsValue, JsValue, JsValue, JsValue) -> Result<(), JsValue>>);
    Reflect::set(
        &gpu_obj,
        &JsValue::from_str("queueWriteTexture"),
        queue_write_texture_closure.as_ref().unchecked_ref(),
    )?;
    queue_write_texture_closure.forget();

    let buffer_size_closure = Closure::wrap(Box::new(gpu_buffer_size)
        as Box<dyn FnMut(JsValue) -> Result<f64, JsValue>>);
    Reflect::set(
        &gpu_obj,
        &JsValue::from_str("bufferSize"),
        buffer_size_closure.as_ref().unchecked_ref(),
    )?;
    buffer_size_closure.forget();

    let buffer_usage_closure = Closure::wrap(Box::new(gpu_buffer_usage)
        as Box<dyn FnMut(JsValue) -> Result<u32, JsValue>>);
    Reflect::set(
        &gpu_obj,
        &JsValue::from_str("bufferUsage"),
        buffer_usage_closure.as_ref().unchecked_ref(),
    )?;
    buffer_usage_closure.forget();

    let buffer_map_state_closure = Closure::wrap(Box::new(gpu_buffer_map_state)
        as Box<dyn FnMut(JsValue) -> Result<JsValue, JsValue>>);
    Reflect::set(
        &gpu_obj,
        &JsValue::from_str("bufferMapState"),
        buffer_map_state_closure.as_ref().unchecked_ref(),
    )?;
    buffer_map_state_closure.forget();

    let buffer_map_async_closure = Closure::wrap(Box::new(gpu_buffer_map_async)
        as Box<dyn FnMut(JsValue, u32, f64, f64) -> Promise>);
    Reflect::set(
        &gpu_obj,
        &JsValue::from_str("bufferMapAsync"),
        buffer_map_async_closure.as_ref().unchecked_ref(),
    )?;
    buffer_map_async_closure.forget();

    let buffer_get_mapped_range_closure = Closure::wrap(Box::new(gpu_buffer_get_mapped_range)
        as Box<dyn FnMut(JsValue, f64, f64) -> Result<ArrayBuffer, JsValue>>);
    Reflect::set(
        &gpu_obj,
        &JsValue::from_str("bufferGetMappedRange"),
        buffer_get_mapped_range_closure.as_ref().unchecked_ref(),
    )?;
    buffer_get_mapped_range_closure.forget();

    let buffer_unmap_closure = Closure::wrap(Box::new(gpu_buffer_unmap)
        as Box<dyn FnMut(JsValue) -> Result<(), JsValue>>);
    Reflect::set(
        &gpu_obj,
        &JsValue::from_str("bufferUnmap"),
        buffer_unmap_closure.as_ref().unchecked_ref(),
    )?;
    buffer_unmap_closure.forget();

    let buffer_destroy_closure = Closure::wrap(Box::new(gpu_buffer_destroy)
        as Box<dyn FnMut(JsValue) -> Result<(), JsValue>>);
    Reflect::set(
        &gpu_obj,
        &JsValue::from_str("bufferDestroy"),
        buffer_destroy_closure.as_ref().unchecked_ref(),
    )?;
    buffer_destroy_closure.forget();

    let texture_create_view_closure = Closure::wrap(Box::new(gpu_texture_create_view)
        as Box<dyn FnMut(JsValue, Option<JsValue>) -> Result<JsValue, JsValue>>);
    Reflect::set(
        &gpu_obj,
        &JsValue::from_str("textureCreateView"),
        texture_create_view_closure.as_ref().unchecked_ref(),
    )?;
    texture_create_view_closure.forget();

    let texture_destroy_closure = Closure::wrap(Box::new(gpu_texture_destroy)
        as Box<dyn FnMut(JsValue) -> Result<(), JsValue>>);
    Reflect::set(
        &gpu_obj,
        &JsValue::from_str("textureDestroy"),
        texture_destroy_closure.as_ref().unchecked_ref(),
    )?;
    texture_destroy_closure.forget();

    let shader_module_get_compilation_info_closure = Closure::wrap(Box::new(gpu_shader_module_get_compilation_info)
        as Box<dyn FnMut(JsValue) -> Promise>);
    Reflect::set(
        &gpu_obj,
        &JsValue::from_str("shaderModuleGetCompilationInfo"),
        shader_module_get_compilation_info_closure.as_ref().unchecked_ref(),
    )?;
    shader_module_get_compilation_info_closure.forget();

    let compute_pipeline_get_bind_group_layout_closure = Closure::wrap(Box::new(gpu_compute_pipeline_get_bind_group_layout)
        as Box<dyn FnMut(JsValue, u32) -> Result<JsValue, JsValue>>);
    Reflect::set(
        &gpu_obj,
        &JsValue::from_str("computePipelineGetBindGroupLayout"),
        compute_pipeline_get_bind_group_layout_closure.as_ref().unchecked_ref(),
    )?;
    compute_pipeline_get_bind_group_layout_closure.forget();

    let render_pipeline_get_bind_group_layout_closure = Closure::wrap(Box::new(gpu_render_pipeline_get_bind_group_layout)
        as Box<dyn FnMut(JsValue, u32) -> Result<JsValue, JsValue>>);
    Reflect::set(
        &gpu_obj,
        &JsValue::from_str("renderPipelineGetBindGroupLayout"),
        render_pipeline_get_bind_group_layout_closure.as_ref().unchecked_ref(),
    )?;
    render_pipeline_get_bind_group_layout_closure.forget();

    let command_encoder_begin_render_pass_closure = Closure::wrap(Box::new(gpu_command_encoder_begin_render_pass)
        as Box<dyn FnMut(JsValue, JsValue) -> Result<JsValue, JsValue>>);
    Reflect::set(
        &gpu_obj,
        &JsValue::from_str("commandEncoderBeginRenderPass"),
        command_encoder_begin_render_pass_closure.as_ref().unchecked_ref(),
    )?;
    command_encoder_begin_render_pass_closure.forget();

    let command_encoder_begin_compute_pass_closure = Closure::wrap(Box::new(gpu_command_encoder_begin_compute_pass)
        as Box<dyn FnMut(JsValue, Option<JsValue>) -> Result<JsValue, JsValue>>);
    Reflect::set(
        &gpu_obj,
        &JsValue::from_str("commandEncoderBeginComputePass"),
        command_encoder_begin_compute_pass_closure.as_ref().unchecked_ref(),
    )?;
    command_encoder_begin_compute_pass_closure.forget();

    let command_encoder_copy_buffer_to_buffer_closure = Closure::wrap(Box::new(gpu_command_encoder_copy_buffer_to_buffer)
        as Box<dyn FnMut(JsValue, JsValue, f64, JsValue, f64, f64) -> Result<(), JsValue>>);
    Reflect::set(
        &gpu_obj,
        &JsValue::from_str("commandEncoderCopyBufferToBuffer"),
        command_encoder_copy_buffer_to_buffer_closure.as_ref().unchecked_ref(),
    )?;
    command_encoder_copy_buffer_to_buffer_closure.forget();

    let command_encoder_copy_buffer_to_texture_closure = Closure::wrap(Box::new(gpu_command_encoder_copy_buffer_to_texture)
        as Box<dyn FnMut(JsValue, JsValue, JsValue, JsValue) -> Result<(), JsValue>>);
    Reflect::set(
        &gpu_obj,
        &JsValue::from_str("commandEncoderCopyBufferToTexture"),
        command_encoder_copy_buffer_to_texture_closure.as_ref().unchecked_ref(),
    )?;
    command_encoder_copy_buffer_to_texture_closure.forget();

    let command_encoder_copy_texture_to_buffer_closure = Closure::wrap(Box::new(gpu_command_encoder_copy_texture_to_buffer)
        as Box<dyn FnMut(JsValue, JsValue, JsValue, JsValue) -> Result<(), JsValue>>);
    Reflect::set(
        &gpu_obj,
        &JsValue::from_str("commandEncoderCopyTextureToBuffer"),
        command_encoder_copy_texture_to_buffer_closure.as_ref().unchecked_ref(),
    )?;
    command_encoder_copy_texture_to_buffer_closure.forget();

    let command_encoder_copy_texture_to_texture_closure = Closure::wrap(Box::new(gpu_command_encoder_copy_texture_to_texture)
        as Box<dyn FnMut(JsValue, JsValue, JsValue, JsValue) -> Result<(), JsValue>>);
    Reflect::set(
        &gpu_obj,
        &JsValue::from_str("commandEncoderCopyTextureToTexture"),
        command_encoder_copy_texture_to_texture_closure.as_ref().unchecked_ref(),
    )?;
    command_encoder_copy_texture_to_texture_closure.forget();

    let command_encoder_clear_buffer_closure = Closure::wrap(Box::new(gpu_command_encoder_clear_buffer)
        as Box<dyn FnMut(JsValue, JsValue, Option<f64>, Option<f64>) -> Result<(), JsValue>>);
    Reflect::set(
        &gpu_obj,
        &JsValue::from_str("commandEncoderClearBuffer"),
        command_encoder_clear_buffer_closure.as_ref().unchecked_ref(),
    )?;
    command_encoder_clear_buffer_closure.forget();

    let command_encoder_write_timestamp_closure = Closure::wrap(Box::new(gpu_command_encoder_write_timestamp)
        as Box<dyn FnMut(JsValue, JsValue, u32) -> Result<(), JsValue>>);
    Reflect::set(
        &gpu_obj,
        &JsValue::from_str("commandEncoderWriteTimestamp"),
        command_encoder_write_timestamp_closure.as_ref().unchecked_ref(),
    )?;
    command_encoder_write_timestamp_closure.forget();

    let command_encoder_resolve_query_set_closure = Closure::wrap(Box::new(gpu_command_encoder_resolve_query_set)
        as Box<dyn FnMut(JsValue, JsValue, u32, u32, JsValue, f64) -> Result<(), JsValue>>);
    Reflect::set(
        &gpu_obj,
        &JsValue::from_str("commandEncoderResolveQuerySet"),
        command_encoder_resolve_query_set_closure.as_ref().unchecked_ref(),
    )?;
    command_encoder_resolve_query_set_closure.forget();

    let command_encoder_finish_closure = Closure::wrap(Box::new(gpu_command_encoder_finish)
        as Box<dyn FnMut(JsValue, Option<JsValue>) -> Result<JsValue, JsValue>>);
    Reflect::set(
        &gpu_obj,
        &JsValue::from_str("commandEncoderFinish"),
        command_encoder_finish_closure.as_ref().unchecked_ref(),
    )?;
    command_encoder_finish_closure.forget();

    let compute_pass_set_pipeline_closure = Closure::wrap(Box::new(gpu_compute_pass_encoder_set_pipeline)
        as Box<dyn FnMut(JsValue, JsValue) -> Result<(), JsValue>>);
    Reflect::set(
        &gpu_obj,
        &JsValue::from_str("computePassEncoderSetPipeline"),
        compute_pass_set_pipeline_closure.as_ref().unchecked_ref(),
    )?;
    compute_pass_set_pipeline_closure.forget();

    let compute_pass_dispatch_workgroups_closure = Closure::wrap(Box::new(gpu_compute_pass_encoder_dispatch_workgroups)
        as Box<dyn FnMut(JsValue, u32, Option<u32>, Option<u32>) -> Result<(), JsValue>>);
    Reflect::set(
        &gpu_obj,
        &JsValue::from_str("computePassEncoderDispatchWorkgroups"),
        compute_pass_dispatch_workgroups_closure.as_ref().unchecked_ref(),
    )?;
    compute_pass_dispatch_workgroups_closure.forget();

    let compute_pass_dispatch_workgroups_indirect_closure = Closure::wrap(Box::new(gpu_compute_pass_encoder_dispatch_workgroups_indirect)
        as Box<dyn FnMut(JsValue, JsValue, f64) -> Result<(), JsValue>>);
    Reflect::set(
        &gpu_obj,
        &JsValue::from_str("computePassEncoderDispatchWorkgroupsIndirect"),
        compute_pass_dispatch_workgroups_indirect_closure.as_ref().unchecked_ref(),
    )?;
    compute_pass_dispatch_workgroups_indirect_closure.forget();

    let compute_pass_end_closure = Closure::wrap(Box::new(gpu_compute_pass_encoder_end)
        as Box<dyn FnMut(JsValue) -> Result<(), JsValue>>);
    Reflect::set(
        &gpu_obj,
        &JsValue::from_str("computePassEncoderEnd"),
        compute_pass_end_closure.as_ref().unchecked_ref(),
    )?;
    compute_pass_end_closure.forget();

    let compute_pass_set_bind_group_closure = Closure::wrap(Box::new(gpu_compute_pass_encoder_set_bind_group)
        as Box<dyn FnMut(JsValue, u32, Option<JsValue>, Option<JsValue>) -> Result<(), JsValue>>);
    Reflect::set(
        &gpu_obj,
        &JsValue::from_str("computePassEncoderSetBindGroup"),
        compute_pass_set_bind_group_closure.as_ref().unchecked_ref(),
    )?;
    compute_pass_set_bind_group_closure.forget();

    let compute_pass_set_bind_group_with_data_closure = Closure::wrap(Box::new(gpu_compute_pass_encoder_set_bind_group_with_data)
        as Box<dyn FnMut(JsValue, u32, Option<JsValue>, JsValue, u32, u32) -> Result<(), JsValue>>);
    Reflect::set(
        &gpu_obj,
        &JsValue::from_str("computePassEncoderSetBindGroupWithData"),
        compute_pass_set_bind_group_with_data_closure.as_ref().unchecked_ref(),
    )?;
    compute_pass_set_bind_group_with_data_closure.forget();

    let compute_pass_push_debug_group_closure = Closure::wrap(Box::new(gpu_compute_pass_encoder_push_debug_group)
        as Box<dyn FnMut(JsValue, String) -> Result<(), JsValue>>);
    Reflect::set(
        &gpu_obj,
        &JsValue::from_str("computePassEncoderPushDebugGroup"),
        compute_pass_push_debug_group_closure.as_ref().unchecked_ref(),
    )?;
    compute_pass_push_debug_group_closure.forget();

    let compute_pass_pop_debug_group_closure = Closure::wrap(Box::new(gpu_compute_pass_encoder_pop_debug_group)
        as Box<dyn FnMut(JsValue) -> Result<(), JsValue>>);
    Reflect::set(
        &gpu_obj,
        &JsValue::from_str("computePassEncoderPopDebugGroup"),
        compute_pass_pop_debug_group_closure.as_ref().unchecked_ref(),
    )?;
    compute_pass_pop_debug_group_closure.forget();

    let compute_pass_insert_debug_marker_closure = Closure::wrap(Box::new(gpu_compute_pass_encoder_insert_debug_marker)
        as Box<dyn FnMut(JsValue, String) -> Result<(), JsValue>>);
    Reflect::set(
        &gpu_obj,
        &JsValue::from_str("computePassEncoderInsertDebugMarker"),
        compute_pass_insert_debug_marker_closure.as_ref().unchecked_ref(),
    )?;
    compute_pass_insert_debug_marker_closure.forget();

    let render_pass_set_pipeline_closure = Closure::wrap(Box::new(gpu_render_pass_encoder_set_pipeline)
        as Box<dyn FnMut(JsValue, JsValue) -> Result<(), JsValue>>);
    Reflect::set(
        &gpu_obj,
        &JsValue::from_str("renderPassEncoderSetPipeline"),
        render_pass_set_pipeline_closure.as_ref().unchecked_ref(),
    )?;
    render_pass_set_pipeline_closure.forget();

    let render_pass_set_index_buffer_closure = Closure::wrap(Box::new(gpu_render_pass_encoder_set_index_buffer)
        as Box<dyn FnMut(JsValue, JsValue, JsValue, f64, Option<f64>) -> Result<(), JsValue>>);
    Reflect::set(
        &gpu_obj,
        &JsValue::from_str("renderPassEncoderSetIndexBuffer"),
        render_pass_set_index_buffer_closure.as_ref().unchecked_ref(),
    )?;
    render_pass_set_index_buffer_closure.forget();

    let render_pass_set_vertex_buffer_closure = Closure::wrap(Box::new(gpu_render_pass_encoder_set_vertex_buffer)
        as Box<dyn FnMut(JsValue, u32, JsValue, f64, Option<f64>) -> Result<(), JsValue>>);
    Reflect::set(
        &gpu_obj,
        &JsValue::from_str("renderPassEncoderSetVertexBuffer"),
        render_pass_set_vertex_buffer_closure.as_ref().unchecked_ref(),
    )?;
    render_pass_set_vertex_buffer_closure.forget();

    let render_pass_draw_closure = Closure::wrap(Box::new(gpu_render_pass_encoder_draw)
        as Box<dyn FnMut(JsValue, u32, Option<u32>, Option<u32>, Option<u32>) -> Result<(), JsValue>>);
    Reflect::set(
        &gpu_obj,
        &JsValue::from_str("renderPassEncoderDraw"),
        render_pass_draw_closure.as_ref().unchecked_ref(),
    )?;
    render_pass_draw_closure.forget();

    let render_pass_draw_indexed_closure = Closure::wrap(Box::new(gpu_render_pass_encoder_draw_indexed)
        as Box<dyn FnMut(JsValue, u32, Option<u32>, Option<u32>, Option<i32>, Option<u32>) -> Result<(), JsValue>>);
    Reflect::set(
        &gpu_obj,
        &JsValue::from_str("renderPassEncoderDrawIndexed"),
        render_pass_draw_indexed_closure.as_ref().unchecked_ref(),
    )?;
    render_pass_draw_indexed_closure.forget();

    let render_pass_draw_indirect_closure = Closure::wrap(Box::new(gpu_render_pass_encoder_draw_indirect)
        as Box<dyn FnMut(JsValue, JsValue, f64) -> Result<(), JsValue>>);
    Reflect::set(
        &gpu_obj,
        &JsValue::from_str("renderPassEncoderDrawIndirect"),
        render_pass_draw_indirect_closure.as_ref().unchecked_ref(),
    )?;
    render_pass_draw_indirect_closure.forget();

    let render_pass_draw_indexed_indirect_closure = Closure::wrap(Box::new(gpu_render_pass_encoder_draw_indexed_indirect)
        as Box<dyn FnMut(JsValue, JsValue, f64) -> Result<(), JsValue>>);
    Reflect::set(
        &gpu_obj,
        &JsValue::from_str("renderPassEncoderDrawIndexedIndirect"),
        render_pass_draw_indexed_indirect_closure.as_ref().unchecked_ref(),
    )?;
    render_pass_draw_indexed_indirect_closure.forget();

    let render_pass_set_viewport_closure = Closure::wrap(Box::new(gpu_render_pass_encoder_set_viewport)
        as Box<dyn FnMut(JsValue, f32, f32, f32, f32, f32, f32) -> Result<(), JsValue>>);
    Reflect::set(
        &gpu_obj,
        &JsValue::from_str("renderPassEncoderSetViewport"),
        render_pass_set_viewport_closure.as_ref().unchecked_ref(),
    )?;
    render_pass_set_viewport_closure.forget();

    let render_pass_set_scissor_rect_closure = Closure::wrap(Box::new(gpu_render_pass_encoder_set_scissor_rect)
        as Box<dyn FnMut(JsValue, u32, u32, u32, u32) -> Result<(), JsValue>>);
    Reflect::set(
        &gpu_obj,
        &JsValue::from_str("renderPassEncoderSetScissorRect"),
        render_pass_set_scissor_rect_closure.as_ref().unchecked_ref(),
    )?;
    render_pass_set_scissor_rect_closure.forget();

    let render_pass_set_blend_constant_closure = Closure::wrap(Box::new(gpu_render_pass_encoder_set_blend_constant)
        as Box<dyn FnMut(JsValue, JsValue) -> Result<(), JsValue>>);
    Reflect::set(
        &gpu_obj,
        &JsValue::from_str("renderPassEncoderSetBlendConstant"),
        render_pass_set_blend_constant_closure.as_ref().unchecked_ref(),
    )?;
    render_pass_set_blend_constant_closure.forget();

    let render_pass_set_stencil_reference_closure = Closure::wrap(Box::new(gpu_render_pass_encoder_set_stencil_reference)
        as Box<dyn FnMut(JsValue, u32) -> Result<(), JsValue>>);
    Reflect::set(
        &gpu_obj,
        &JsValue::from_str("renderPassEncoderSetStencilReference"),
        render_pass_set_stencil_reference_closure.as_ref().unchecked_ref(),
    )?;
    render_pass_set_stencil_reference_closure.forget();

    let render_pass_begin_occlusion_query_closure = Closure::wrap(Box::new(gpu_render_pass_encoder_begin_occlusion_query)
        as Box<dyn FnMut(JsValue, u32) -> Result<(), JsValue>>);
    Reflect::set(
        &gpu_obj,
        &JsValue::from_str("renderPassEncoderBeginOcclusionQuery"),
        render_pass_begin_occlusion_query_closure.as_ref().unchecked_ref(),
    )?;
    render_pass_begin_occlusion_query_closure.forget();

    let render_pass_end_occlusion_query_closure = Closure::wrap(Box::new(gpu_render_pass_encoder_end_occlusion_query)
        as Box<dyn FnMut(JsValue) -> Result<(), JsValue>>);
    Reflect::set(
        &gpu_obj,
        &JsValue::from_str("renderPassEncoderEndOcclusionQuery"),
        render_pass_end_occlusion_query_closure.as_ref().unchecked_ref(),
    )?;
    render_pass_end_occlusion_query_closure.forget();

    let render_pass_execute_bundles_closure = Closure::wrap(Box::new(gpu_render_pass_encoder_execute_bundles)
        as Box<dyn FnMut(JsValue, JsValue) -> Result<(), JsValue>>);
    Reflect::set(
        &gpu_obj,
        &JsValue::from_str("renderPassEncoderExecuteBundles"),
        render_pass_execute_bundles_closure.as_ref().unchecked_ref(),
    )?;
    render_pass_execute_bundles_closure.forget();

    let render_pass_end_closure = Closure::wrap(Box::new(gpu_render_pass_encoder_end)
        as Box<dyn FnMut(JsValue) -> Result<(), JsValue>>);
    Reflect::set(
        &gpu_obj,
        &JsValue::from_str("renderPassEncoderEnd"),
        render_pass_end_closure.as_ref().unchecked_ref(),
    )?;
    render_pass_end_closure.forget();

    let render_pass_set_bind_group_closure = Closure::wrap(Box::new(gpu_render_pass_encoder_set_bind_group)
        as Box<dyn FnMut(JsValue, u32, Option<JsValue>, Option<JsValue>) -> Result<(), JsValue>>);
    Reflect::set(
        &gpu_obj,
        &JsValue::from_str("renderPassEncoderSetBindGroup"),
        render_pass_set_bind_group_closure.as_ref().unchecked_ref(),
    )?;
    render_pass_set_bind_group_closure.forget();

    let render_pass_set_bind_group_with_data_closure = Closure::wrap(Box::new(gpu_render_pass_encoder_set_bind_group_with_data)
        as Box<dyn FnMut(JsValue, u32, Option<JsValue>, JsValue, u32, u32) -> Result<(), JsValue>>);
    Reflect::set(
        &gpu_obj,
        &JsValue::from_str("renderPassEncoderSetBindGroupWithData"),
        render_pass_set_bind_group_with_data_closure.as_ref().unchecked_ref(),
    )?;
    render_pass_set_bind_group_with_data_closure.forget();

    let render_pass_push_debug_group_closure = Closure::wrap(Box::new(gpu_render_pass_encoder_push_debug_group)
        as Box<dyn FnMut(JsValue, String) -> Result<(), JsValue>>);
    Reflect::set(
        &gpu_obj,
        &JsValue::from_str("renderPassEncoderPushDebugGroup"),
        render_pass_push_debug_group_closure.as_ref().unchecked_ref(),
    )?;
    render_pass_push_debug_group_closure.forget();

    let render_pass_pop_debug_group_closure = Closure::wrap(Box::new(gpu_render_pass_encoder_pop_debug_group)
        as Box<dyn FnMut(JsValue) -> Result<(), JsValue>>);
    Reflect::set(
        &gpu_obj,
        &JsValue::from_str("renderPassEncoderPopDebugGroup"),
        render_pass_pop_debug_group_closure.as_ref().unchecked_ref(),
    )?;
    render_pass_pop_debug_group_closure.forget();

    let render_pass_insert_debug_marker_closure = Closure::wrap(Box::new(gpu_render_pass_encoder_insert_debug_marker)
        as Box<dyn FnMut(JsValue, String) -> Result<(), JsValue>>);
    Reflect::set(
        &gpu_obj,
        &JsValue::from_str("renderPassEncoderInsertDebugMarker"),
        render_pass_insert_debug_marker_closure.as_ref().unchecked_ref(),
    )?;
    render_pass_insert_debug_marker_closure.forget();

    Ok(gpu_obj.into())
} 