use std::path::PathBuf;

use anyhow::Result;
use cpal::traits::HostTrait;
use libtest_mimic::{Arguments, Trial};
use pollster;

pub use common::*;

pub use native::web::RuneRuntimeState;

use winit::{
    error::EventLoopError,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopBuilder},
    raw_window_handle::{HasDisplayHandle, HasWindowHandle},
    window::{Window, WindowBuilder},
};

#[cfg(target_os = "macos")]
use winit::platform::macos::WindowBuilderExtMacOS;

#[cfg(target_arch = "wasm32")]
use crate::runtime::web::run;

#[cfg(target_os = "macos", target_os = "windows")]
use crate::runtime::native::run;

use crate::{game::Game};

pub fn run(input_path: PathBuf, binary: Vec<u8>) {
    let event_loop = EventLoopBuilder::<GameEvent>::with_user_event()
        .build()
        .unwrap();

    let window_builder = WindowBuilder::new()
        .with_title("Game");

    #[cfg(target_os = "macos")]
    let window_builder = window_builder.with_titlebar_hidden(true);

    let window = window_builder.build(&event_loop).unwrap();

    // #[cfg(not(target_arch = "wasm32"))]
    // {
    // env_logger::init();
    // Temporarily avoid srgb formats for the swapchain on the web
    pollster::block_on(run_loop(event_loop, window, input_path, binary)).ok();
    // }
    // #[cfg(target_arch = "wasm32")]
    // {
    //     std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    //     console_log::init().expect("could not initialize logger");
    //     use winit::platform::web::WindowExtWebSys;
    //     // On wasm, append the canvas to the document body
    //     web_sys::window()
    //         .and_then(|win| win.document())
    //         .and_then(|doc| doc.body())
    //         .and_then(|body| {
    //             body.append_child(&web_sys::Element::from(window.canvas()))
    //                 .ok()
    //         })
    //         .expect("couldn't append canvas to document body");

    //     wasm_bindgen_futures::spawn_local(run_loop(event_loop, window));
    // }
}



pub async fn test(_input_path: PathBuf, _binary: Vec<u8>) {
    // Parse command line arguments
    let args = Arguments::from_args();

    // Create a list of tests and/or benchmarks (in this case: two dummy tests).
    let tests = vec![
        Trial::test("succeeding_test", move || Ok(())),
        Trial::test("failing_test", move || Err("Woops".into())),
    ];

    // TODO: Setup Rune host in test mode

    // TODO: Guest implements Rune "test guest", which has a setup_tests() function

    // TODO: Guest calls add-test method which registers the test

    // TODO: Registered tests go into tests vec above. Test function invokes the "run_test(name)" method on the guest.
    // Guest invokes the test associated with the name passed to run_test.

    // let event_loop = EventLoopBuilder::<GameEvent>::with_user_event()
    //     .build()
    //     .unwrap();

    // let window = WindowBuilder::new()
    //     .with_title("Game")
    //     .with_titlebar_hidden(true)
    //     .build(&event_loop)
    //     .unwrap();

    // let instance = wgpu_core::global::Global::new(
    //     "webgpu",
    //     wgpu_types::InstanceDescriptor {
    //         backends: wgpu_types::Backends::all(),
    //         flags: wgpu_types::InstanceFlags::from_build_config(),
    //         dx12_shader_compiler: wgpu_types::Dx12Compiler::Fxc,
    //         gles_minor_version: wgpu_types::Gles3MinorVersion::default(),
    //     },
    // );
    // let surface_id = unsafe {
    //     instance.instance_create_surface(
    //         window.raw_display_handle().unwrap(),
    //         window.raw_window_handle().unwrap(),
    //         None,
    //     ).unwrap()
    // };
    // let adapter_id = instance
    //     .request_adapter(
    //         &Default::default(),
    //         wgpu_core::instance::AdapterInputs::Mask(wgpu_types::Backends::all(), |_| None),
    //     )
    //     .unwrap();

    // let adapter_limits = instance.adapter_limits::<crate::Backend>(adapter_id).unwrap();

    // // Create the logical device and command queue
    // let (device_id, queue_id) = instance.adapter_request_device::<crate::Backend>(
    //     adapter_id,
    //     &wgpu_types::DeviceDescriptor {
    //         label: None,
    //         required_features: wgpu_types::Features::empty(),
    //         // Make sure we use the texture resolution limits from the adapter, so we can support images the size of the swapchain.
    //         required_limits: wgpu_types::Limits::downlevel_webgl2_defaults()
    //             .using_resolution(adapter_limits),
    //     },
    //     None,
    //     None,
    //     None
    // )
    // .unwrap();

    // let audio_host = cpal::default_host();
    // let audio_device = audio_host.default_output_device().unwrap();

    // let mut gilrs = gilrs::Gilrs::new().unwrap();

    // let mut test = Tests::from_binary(&binary).unwrap();
    // test.init(&window, input_path, audio_device, instance, surface_id, adapter_id, device_id, queue_id).await.expect("Tests didn't initialize");

    libtest_mimic::run(&args, tests).exit_code();
}
