use std::path::PathBuf;

use anyhow::Result;
use cpal::traits::HostTrait;
use libtest_mimic::{Arguments, Trial};
use pollster;

mod common;
mod native;

pub use common::*;

use winit::{
    error::EventLoopError,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopBuilder},
    raw_window_handle::{HasDisplayHandle, HasWindowHandle},
    window::{Window, WindowBuilder},
};

#[cfg(target_os = "macos")]
use winit::platform::macos::WindowBuilderExtMacOS;

use crate::{game::Game};

enum GameEvent {}

async fn run_loop(
    event_loop: EventLoop<GameEvent>,
    window: Window,
    input_path: PathBuf,
    binary: Vec<u8>,
) -> Result<(), EventLoopError> {
    let instance = wgpu_core::global::Global::new(
        "webgpu",
        wgpu_types::InstanceDescriptor {
            backends: wgpu_types::Backends::all(),
            flags: wgpu_types::InstanceFlags::from_build_config(),
            dx12_shader_compiler: wgpu_types::Dx12Compiler::Fxc,
            gles_minor_version: wgpu_types::Gles3MinorVersion::default(),
        },
    );
    let surface_id = unsafe {
        instance
            .instance_create_surface(
                window.display_handle().unwrap().into(),
                window.window_handle().unwrap().into(),
                None,
            )
            .unwrap()
    };
    let adapter_id = instance
        .request_adapter(
            &Default::default(),
            wgpu_types::Backends::all(),
            None
        )
        .unwrap();

    let adapter_limits = instance
        .adapter_limits(adapter_id);

    // Create the logical device and command queue
    let (device_id, queue_id) = instance.adapter_request_device(
        adapter_id,
        &wgpu_types::DeviceDescriptor {
            label: None,
            required_features: wgpu_types::Features::empty(),
            // Make sure we use the texture resolution limits from the adapter, so we can support images the size of the swapchain.
            required_limits:
                wgpu_types::Limits::downlevel_webgl2_defaults().using_resolution(adapter_limits),
            memory_hints: wgpu_types::MemoryHints::default(),
        },
        None,
        None,
        None,
    ).unwrap();

    let audio_host = cpal::default_host();
    let audio_device = audio_host.default_output_device().unwrap();

    let gilrs = gilrs::Gilrs::new().unwrap();

    let mut game = Game::from_binary(&binary).unwrap();
    game.init(
        &window,
        input_path,
        audio_device,
        instance,
        surface_id,
        adapter_id,
        device_id,
        queue_id,
        gilrs,
    )
    .await
    .expect("Game didn't initialize");

    let start_time = std::time::Instant::now();

    let mut last_logic_update = start_time.clone();
    let mut last_render_update = start_time.clone();

    let logic_frame_time = std::time::Duration::from_millis(1000 / 30); // 30 FPS for logic
    let render_frame_time = std::time::Duration::from_millis(1000 / 60); // 60 FPS for rendering

    event_loop.run(move |event, elwt| {
        let now = std::time::Instant::now();

        elwt.set_control_flow(ControlFlow::WaitUntil(std::cmp::min(
            last_logic_update + logic_frame_time,
            last_render_update + render_frame_time,
        )));

        match event {
            Event::UserEvent(_event) => {}
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                game.resize(size);
            }
            Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        event: key_event, ..
                    },
                ..
            } => {
                let generation = game.store.as_ref().unwrap().data().generation;
                let keyboard_state = &mut game.store.as_mut().unwrap().data_mut().keyboard_state;

                if !key_event.state.is_pressed() || key_event.repeat {
                    keyboard_state.active_keys.retain(|key| {
                        !(key.1.eq(&key_event.logical_key) && key.2.eq(&key_event.location))
                    });

                    if key_event.repeat {
                        keyboard_state.active_keys.push((
                            generation,
                            key_event.logical_key,
                            key_event.location,
                        ));
                    }
                } else if !keyboard_state
                    .active_keys
                    .iter()
                    .any(|k| k.1.eq(&key_event.logical_key) && k.2.eq(&key_event.location))
                {
                    keyboard_state.active_keys.push((
                        generation,
                        key_event.logical_key,
                        key_event.location,
                    ));
                }
            }
            Event::AboutToWait => {
                // start gamepad handling -- could this be a winit user event?
                let generation = game.store.as_ref().unwrap().data().generation;

                let game_store = game.store.as_mut().unwrap();
                let gilrs_event = { game_store.data_mut().gilrs.next_event() };
                let gamepad_state = &mut game_store.data_mut().gamepad_state;

                while let Some(button_event) = gilrs_event {
                    //TODO: Handle multiple gamepads

                    match button_event.event {
                        gilrs::EventType::ButtonPressed(button, _) => {
                            if !gamepad_state.active_buttons.iter().any(|b| b.1.eq(&button)) {
                                gamepad_state.active_buttons.push((generation, button));
                            }
                        }
                        gilrs::EventType::ButtonRepeated(button, _) => {
                            if !gamepad_state.active_buttons.iter().any(|b| b.1.eq(&button)) {
                                gamepad_state.active_buttons.push((generation, button));
                            }

                            // TODO: Set is_repeating = true on this button
                        }
                        gilrs::EventType::ButtonReleased(button, _) => {
                            gamepad_state.active_buttons.retain(|b| !b.1.eq(&button));
                        }
                        gilrs::EventType::ButtonChanged(_, _, _) => {}
                        gilrs::EventType::AxisChanged(_, _, _) => {}
                        gilrs::EventType::Connected => todo!(),
                        gilrs::EventType::Disconnected => todo!(),
                        gilrs::EventType::Dropped => continue,
                    }
                }
                // end gamepad handling

                if now - last_logic_update >= logic_frame_time {
                    // TODO/FIXME: Generation should be based on logical frame not visual frame, and guest should specify
                    // its logical frame rate, and then use its own logic to limit render calls if needed
                    let generation = &mut game.store.as_mut().unwrap().data_mut().generation;
                    *generation = *generation + 1;

                    let epoch_time = now - start_time;
                    let delta_time = now - last_render_update;

                    pollster::block_on(game.update(epoch_time, delta_time)).unwrap();
                    last_logic_update = now;
                }

                if now - last_render_update >= render_frame_time {
                    window.request_redraw();
                    last_render_update = now;
                }
            }
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                let epoch_time = now - start_time;
                let delta_time = now - last_render_update;
                pollster::block_on(game.render(epoch_time, delta_time)).unwrap();
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => elwt.exit(),
            _ => {}
        }
    })
}

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
