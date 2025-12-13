use log::{error, info};
use softbuffer::{Context, Surface};
use std::num::NonZeroU32;
use std::rc::Rc;
use std::sync::Arc;
use winit::event::{ElementState, Event, KeyEvent, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::keyboard::PhysicalKey;
use winit::window::WindowBuilder;

use smithay::input::keyboard::FilterResult;
use smithay::input::pointer::{ButtonEvent, MotionEvent};
use smithay::input::{Seat, SeatHandler, SeatState};
use smithay::reexports::wayland_server::protocol::{wl_compositor, wl_shm};
use smithay::reexports::wayland_server::Resource;
use smithay::reexports::wayland_server::{Display, ListeningSocket};
use smithay::utils::{Serial, SERIAL_COUNTER};

mod handlers;
mod keymap;
mod messages;
mod render;
mod state;
use messages::CompositorMessage;

use crate::state::AppState;

 

fn main() {
    if let Ok(env) = std::env::var("RUST_LOG") {
        tracing_subscriber::fmt().with_env_filter(env).init();
    } else {
        tracing_subscriber::fmt().init();
    }

     
    let event_loop = EventLoop::new().unwrap();

    // Load Icon
    println!("Attempting to load icon from assets/icon.png...");
    let icon = if let Ok(img) = image::open("assets/icon.png") {
        println!("Icon file opened successfully. Dimensions: {:?}", img.dimensions());
        use image::GenericImageView;
        let (width, height) = img.dimensions();
        let rgba = img.into_rgba8().into_raw();
        let icon_result = winit::window::Icon::from_rgba(rgba, width, height);
        match icon_result {
            Ok(icon) => {
                println!("Winit Icon created successfully.");
                Some(icon)
            },
            Err(e) => {
                println!("Failed to create winit Icon: {:?}", e);
                None
            }
        }
    } else {
        println!("Failed to find or open assets/icon.png");
        log::warn!("Failed to load assets/icon.png");
        None
    };

    let mut window_builder = WindowBuilder::new()
            .with_title("Wayland on macOS (Smithay)")
            .with_inner_size(winit::dpi::LogicalSize::new(800.0, 600.0));

    if let Some(i) = icon {
        window_builder = window_builder.with_window_icon(Some(i));
    }

    let window = Rc::new(
        window_builder
            .build(&event_loop)
            .unwrap(),
    );

     
     
    let context = Context::new(window.clone()).unwrap();
     
    let window_ref = Box::leak(Box::new(window.clone()));
    let mut surface = unsafe { softbuffer::Surface::new(&context, window_ref) }.unwrap();

     
    let mut display = Display::<AppState>::new().unwrap();
    let display_handle = display.handle();

     
    let (loop_signal, loop_receiver) = std::sync::mpsc::channel::<CompositorMessage>();
    let scale_factor = window.scale_factor();
    let mut state = AppState::new(
        &display_handle,
        window.scale_factor(),
        loop_signal,
        window.inner_size().width,
        window.inner_size().height,
    );

     
    let initial_size = window.inner_size();
    let initial_mode = smithay::output::Mode {
        size: (initial_size.width as i32, initial_size.height as i32).into(),
        refresh: 60_000,
    };
    state.output.change_current_state(
        Some(initial_mode),
        Some(smithay::utils::Transform::Normal),
        Some(smithay::output::Scale::Fractional(window.scale_factor())),
        Some((0, 0).into()),
    );
    state.output.set_preferred(initial_mode);

     
     
     
    let runtime_dir = std::env::temp_dir().join("cocoa-way");
    if !runtime_dir.exists() {
        std::fs::create_dir_all(&runtime_dir).unwrap();
    }
    unsafe { std::env::set_var("XDG_RUNTIME_DIR", &runtime_dir); }

     
    let listener = ListeningSocket::bind_auto("wayland", 1..10).unwrap();
    let socket_name = listener
        .socket_name()
        .unwrap()
        .to_string_lossy()
        .into_owned();
    let socket_path = runtime_dir.join(&socket_name);

    info!("Wayland socket created: {:?}", socket_name);
    info!("XDG_RUNTIME_DIR set to: {:?}", runtime_dir);
    info!(
        "To run clients: export XDG_RUNTIME_DIR={:?} WAYLAND_DISPLAY={}",
        runtime_dir, socket_name
    );

    unsafe { std::env::set_var("WAYLAND_DISPLAY", &socket_name); }

     
    let mut loop_handle = display_handle.clone();
    std::thread::spawn(move || loop {
        match listener.accept() {
            Ok(Some(stream)) => {
                use crate::state::ClientState;
                use smithay::wayland::compositor::CompositorClientState;

                info!("New client connected");
                loop_handle
                    .insert_client(
                        stream,
                        Arc::new(ClientState {
                            compositor_state: Default::default(),
                        }),
                    )
                    .unwrap();
            }
            Ok(None) => {}
            Err(_) => {
                std::thread::sleep(std::time::Duration::from_millis(10));
            }
        }
    });

     
    let mut last_mouse_pos =
        smithay::utils::Point::<f64, smithay::utils::Logical>::from((0.0, 0.0));

     
    let start_time = std::time::Instant::now();

    event_loop.run(move |event, target| {
         
        target.set_control_flow(ControlFlow::WaitUntil(std::time::Instant::now() + std::time::Duration::from_millis(16))); 
        
         
         if std::time::Instant::now().elapsed().as_millis() % 5000 < 10 {
              
             static mut LAST_PRINT: u64 = 0;
             unsafe {
                 let now = std::time::Instant::now().elapsed().as_secs();
                 if now > LAST_PRINT {
                    println!("HEARTBEAT: width={}, height={}, scale={}", state.width, state.height, state.scale_factor);
                    LAST_PRINT = now;
                 }
             }
         } 

         
        while let Ok(msg) = loop_receiver.try_recv() {
            match msg {
                CompositorMessage::Maximize(max) => {
                    log::info!("Handling Maximize: {}", max);
                    window.set_maximized(max);
                },
                CompositorMessage::Fullscreen(full) => {
                    log::info!("Handling Fullscreen: {}", full);
                     if full {
                        window.set_fullscreen(Some(winit::window::Fullscreen::Borderless(None)));
                    } else {
                        window.set_fullscreen(None);
                    }
                }
            }
        }

        match event {
            Event::WindowEvent { window_id, event } if window_id == window.id() => {
                match event {
                    WindowEvent::Resized(size) => {
                         println!("*** HIT RESIZED EVENT: {}x{} ***", size.width, size.height);
                         let width = size.width as i32;
                         let height = size.height as i32;
                         log::info!("Window Resized to {}x{}", width, height);
                         
                          
                         if let Some(w) = std::num::NonZeroU32::new(size.width) {
                             if let Some(h) = std::num::NonZeroU32::new(size.height) {
                                 let _ = surface.resize(w, h);
                             }
                         }

                          
                         state.width = size.width;
                         state.height = size.height;
                         state.scale_factor = window.scale_factor();
                         log::info!("DEBUG RESIZED: width={}, height={}, scale_factor={}", state.width, state.height, state.scale_factor);
                         
                          
                         let mode = smithay::output::Mode {
                             size: (width, height).into(),
                             refresh: 60_000,
                         };
                         state.output.change_current_state(
                             Some(mode),
                             Some(smithay::utils::Transform::Normal),
                             Some(smithay::output::Scale::Fractional(state.scale_factor)),
                             Some((0,0).into())
                         );

                          
                         let logical_width = (width as f64 / state.scale_factor) as i32;
                         let logical_height = (height as f64 / state.scale_factor) as i32;
                         
                         for toplevel in state.toplevels.iter() {
                             toplevel.with_pending_state(|state| {
                                 state.size = Some((logical_width, logical_height).into());
                             });
                             toplevel.send_configure();
                         }
                    },
                    WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                        log::info!("ScaleFactorChanged: {}", scale_factor);
                        state.update_scale_factor(scale_factor);
                    },
                    WindowEvent::CloseRequested => target.exit(),
                     

                    WindowEvent::KeyboardInput { event: KeyEvent { state: el_state, physical_key, .. }, .. } => {
                        if let winit::keyboard::PhysicalKey::Code(key_code) = physical_key {
                             
                            use winit::keyboard::KeyCode;
                            
                            match key_code {
                                KeyCode::Escape => target.exit(),
                                _ => {
                                     use smithay::backend::input::KeyState;
                                     use smithay::input::keyboard::Keycode;  

                                     let serial = SERIAL_COUNTER.next_serial();
                                     let time = start_time.elapsed().as_millis() as u32;
                                     if let Some(keyboard) = state.seat.get_keyboard() {
                                         if let Some(scancode) = crate::keymap::map_key(physical_key) {
                                             let key_state = match el_state {
                                                 ElementState::Pressed => KeyState::Pressed,
                                                 ElementState::Released => KeyState::Released,
                                             };
                                             let keycode = Keycode::from(scancode + 8);
                                             keyboard.input(&mut state, keycode, key_state, serial, time, |_, _, _| FilterResult::<()>::Forward);
                                         }
                                     }
                                }
                            }
                        }
                    },
                    WindowEvent::CursorMoved { position, .. } => {
                        let scale = window.scale_factor();
                        let logical_pos = position.to_logical::<f64>(scale);
                        log::info!("CursorMoved: Physical({:?}) -> Logical({:?})", position, logical_pos);
                        
                        let serial = SERIAL_COUNTER.next_serial();
                        let pointer = state.seat.get_pointer().unwrap();
                        let position_f64 = smithay::utils::Point::<f64, smithay::utils::Logical>::from((logical_pos.x, logical_pos.y));
                        last_mouse_pos = position_f64;

                         
                         if let Some(target_id) = state.start_drag_request.take() {
                              let (cur_x, cur_y) = *state.surface_positions.get(&target_id).unwrap_or(&(0,0));
                              let offset_x = logical_pos.x - cur_x as f64;
                              let offset_y = logical_pos.y - cur_y as f64;
                              state.drag_state = Some((target_id.clone(), (offset_x, offset_y)));
                              log::info!("Drag Started for {:?}", target_id);
                         }

                         
                        if let Some((target_id, (offset_x, offset_y))) = state.drag_state.clone() {
                            let new_x = logical_pos.x - offset_x;
                            let new_y = logical_pos.y - offset_y;
                            state.surface_positions.insert(target_id, (new_x as i32, new_y as i32));
                            window.request_redraw();  
                        }

                        let mut focus = None;
                         
                          
                         'outer: for (i, toplevel) in state.toplevels.iter().rev().enumerate() {
                             let wl_surface = toplevel.wl_surface();
                             let id = wl_surface.id();
                                  
                              
                             let (x_offset, y_offset) = *state.surface_positions.entry(id.clone()).or_insert_with(|| {
                                   
                                  let idx = state.toplevels.iter().position(|s| s.wl_surface().id() == id).unwrap_or(0);
                                  ((idx * 50) as i32, (idx * 50) as i32)
                             });
                             
                              
                             smithay::wayland::compositor::with_surface_tree_downward(
                                wl_surface,
                                (x_offset, y_offset),
                                |_, _, &loc| {
                                    smithay::wayland::compositor::TraversalAction::DoChildren(loc)
                                },
                                |surface, states, &loc| {
                                    let mut guard = states.cached_state.get::<smithay::wayland::compositor::SurfaceAttributes>();
                                     
                                     
                                     
                                    let current_attrs = guard.current();
                                    let scale = current_attrs.buffer_scale;
                                    let (width, height) = if let Some(buffer) = &current_attrs.buffer {
                                      match buffer {
                                          smithay::wayland::compositor::BufferAssignment::NewBuffer(b) => {
                                                if let Ok((w, h)) = smithay::wayland::shm::with_buffer_contents(b, |_, _, spec| (spec.width, spec.height)) {
                                                   ((w as f64) / (scale as f64), (h as f64) / (scale as f64))
                                                } else { 
                                                    
                                                   log::warn!("HitTest: Failed to get SHM dimensions for {:?}.", surface.id());
                                                   (0.0, 0.0) 
                                                }
                                          },
                                          _ => {
                                              (0.0, 0.0)
                                          }
                                      }    
                                    } else { 
                                        (0.0, 0.0) 
                                    };

                                    if width > 0.0 && height > 0.0 {
                                        if logical_pos.x >= loc.0 as f64 && logical_pos.x <= loc.0 as f64 + width &&
                                           logical_pos.y >= loc.1 as f64 && logical_pos.y <= loc.1 as f64 + height {
                                            
                                             
                                             
                                            let surface_location = smithay::utils::Point::<f64, smithay::utils::Logical>::from((loc.0 as f64, loc.1 as f64));
                                            
                                            log::info!("HitTest: Focused {:?} at Surface Origin {:?}", surface.id(), surface_location);
                                            focus = Some((surface.clone(), surface_location));
                                        }
                                    }
                                },
                                |_, _, _| true
                             );
                             
                             if focus.is_some() {
                                 break 'outer;
                             }
                        }


                         
                        let time = start_time.elapsed().as_millis() as u32;

                        let event = MotionEvent {
                            location: position_f64,
                            serial,
                            time,
                        };
                        log::info!("Motion Focus: {:?}", focus.as_ref().map(|(s,_)| s.id()));
                        pointer.motion(
                            &mut state,
                            focus, 
                            &event,
                        );
                        pointer.frame(&mut state);
                    },
                    WindowEvent::MouseInput { state: el_state, button, .. } => {
                        log::info!("MouseInput: {:?} {:?}", button, el_state);
                        let serial = SERIAL_COUNTER.next_serial();
                        let pointer = state.seat.get_pointer().unwrap();
                        let keyboard = state.seat.get_keyboard().unwrap();
                        
                        let button_code = match button {
                            winit::event::MouseButton::Left => 0x110,  
                            winit::event::MouseButton::Right => 0x111,
                            winit::event::MouseButton::Middle => 0x112,
                            _ => 0x110,
                        };
                         let p_state = match el_state {
                            ElementState::Pressed => smithay::backend::input::ButtonState::Pressed,
                            ElementState::Released => smithay::backend::input::ButtonState::Released,
                        };
                        
                        let time = start_time.elapsed().as_millis() as u32;
                        
                         
                        if p_state == smithay::backend::input::ButtonState::Pressed && button == winit::event::MouseButton::Left {
                             
                            let mut focus_surface = None;
                            if let Some(pointer_state) = state.seat.get_pointer() {
                                if let Some(surface) = pointer_state.current_focus() {
                                    focus_surface = Some(surface);
                                }
                            }

                            if let Some(surface) = focus_surface {
                                log::info!("Click-Focus: Setting keyboard focus to {:?}", surface.id());
                                keyboard.set_focus(&mut state, Some(surface.clone()), serial);
                                
                                 
                                if let Some(toplevel) = state.toplevels.iter().find(|t| t.wl_surface() == &surface) {
                                     toplevel.with_pending_state(|state| {
                                        state.states.set(smithay::reexports::wayland_protocols::xdg::shell::server::xdg_toplevel::State::Activated);
                                    });
                                    toplevel.send_configure();
                                }
                            } else {
                                keyboard.set_focus(&mut state, None, serial);
                            }
                        }
                        
                         
                         if p_state == smithay::backend::input::ButtonState::Pressed && button == winit::event::MouseButton::Left {
                             if let Some(target_id) = state.start_drag_request.take() {
                                 let (cur_x, cur_y) = *state.surface_positions.get(&target_id).unwrap_or(&(0,0));
                                 let offset_x = last_mouse_pos.x - cur_x as f64;
                                 let offset_y = last_mouse_pos.y - cur_y as f64;
                                 state.drag_state = Some((target_id, (offset_x, offset_y)));
                             }
                         }
                        
                         
                        if p_state == smithay::backend::input::ButtonState::Released && button == winit::event::MouseButton::Left {
                            state.drag_state = None;
                        }

                        let event = ButtonEvent {
                            button: button_code,
                            state: p_state,
                            serial,
                            time,
                        };
                        pointer.button(&mut state, &event);
                        pointer.frame(&mut state);
                    },
                    WindowEvent::MouseWheel { delta, phase, .. } => {
                        let pointer = state.seat.get_pointer().unwrap();
                        let time = start_time.elapsed().as_millis() as u32;
                        
                        let (idx, amount, source) = match delta {
                           winit::event::MouseScrollDelta::LineDelta(x, y) => {
                               if x != 0.0 {
                                   (smithay::backend::input::Axis::Horizontal, -x as f64 * 10.0, smithay::backend::input::AxisSource::Wheel)
                               } else {
                                   (smithay::backend::input::Axis::Vertical, -y as f64 * 10.0, smithay::backend::input::AxisSource::Wheel)
                               }
                           },
                           winit::event::MouseScrollDelta::PixelDelta(pos) => {
                               let scale = window.scale_factor();
                               let logical_pos = pos.to_logical::<f64>(scale);
                               if logical_pos.x != 0.0 {
                                   (smithay::backend::input::Axis::Horizontal, -logical_pos.x, smithay::backend::input::AxisSource::Finger)
                               } else {
                                   (smithay::backend::input::Axis::Vertical, -logical_pos.y, smithay::backend::input::AxisSource::Finger)
                               }
                           }
                        };
                        
                         
                        if amount != 0.0 {
                             let (h, v) = if idx == smithay::backend::input::Axis::Horizontal { (amount, 0.0) } else { (0.0, amount) };
                             let stop_tuple = if phase == winit::event::TouchPhase::Ended {
                                 if idx == smithay::backend::input::Axis::Horizontal { (true, false) } else { (false, true) }
                             } else { (false, false) };

                             let details = smithay::input::pointer::AxisFrame {
                                 source: Some(source),
                                 time,
                                 axis: (h, v),
                                 stop: stop_tuple,
                                 v120: Some((0, 0)),
                                 relative_direction: (smithay::backend::input::AxisRelativeDirection::Identical, smithay::backend::input::AxisRelativeDirection::Identical),
                             };
                             pointer.axis(&mut state, details);
                             pointer.frame(&mut state);
                        }
                    },
                    WindowEvent::RedrawRequested => {
                          
                          
                         let (width, height) = {
                            let size = window.inner_size();
                            (size.width, size.height)
                        };
                        
                        if width > 0 && height > 0 {
                            surface.resize(
                                NonZeroU32::new(width).unwrap(),
                                NonZeroU32::new(height).unwrap(),
                            ).unwrap();

                            let mut sb_buffer = surface.buffer_mut().unwrap();
                            sb_buffer.fill(0);

                            let mut rendered_surfaces = 0;  
                            use smithay::reexports::wayland_server::Resource;
                            let mut rendered_count = 0;
                             
                            for (i, toplevel) in state.toplevels.iter().enumerate() {
                                let wl_surface = toplevel.wl_surface();
                                let id = wl_surface.id();
                                
                                 
                                let (x_offset, y_offset) = *state.surface_positions.entry(id.clone()).or_insert_with(|| {
                                     let idx = i;
                                     ((idx * 50) as i32, (idx * 50) as i32)
                                });
                                
                                 
                                smithay::wayland::compositor::with_surface_tree_downward(
                                    wl_surface,
                                    (x_offset, y_offset),
                                    |_, _, &loc| {
                                        smithay::wayland::compositor::TraversalAction::DoChildren(loc)
                                    },
                                    |surface, states, &loc| {
                                        let mut guard = states.cached_state.get::<smithay::wayland::compositor::SurfaceAttributes>();
                                         
                                        let current = guard.current();
                                     
                                        
                                        if let Some(buffer) = &current.buffer {
                                              
                                             if let smithay::wayland::compositor::BufferAssignment::NewBuffer(b) = buffer {
                                                  
                                                 let scale = window.scale_factor();
                        
                        crate::render::render_surface(
                                                    b,
                                                    &mut sb_buffer,
                                                    width,
                                                    height,
                                                    (loc.0 as f64 * scale) as i32,
                                                    (loc.1 as f64 * scale) as i32
                                                );
                                                rendered_count += 1;
                                             }
                                        }
                                    },
                                    |_, _, _| true
                                );
                                
                                 
                                send_frames_surface_tree(
                                    wl_surface, 
                                    std::time::Instant::now().elapsed().as_millis() as u32
                                );
                            }
                            if rendered_count > 0 {
                                 
                            }
                            
                            


                             sb_buffer.present().unwrap();
                        }
                    }
                    _ => {}
                }
            }
            Event::AboutToWait => {
                  
                match display.dispatch_clients(&mut state) {
                    Ok(_) => {
                        display.flush_clients().unwrap();
                    }
                    Err(_) => {}
                }
                window.request_redraw();
            }
            _ => {}
        }
    }).unwrap();
}

fn send_frames_surface_tree(
    surface: &smithay::reexports::wayland_server::protocol::wl_surface::WlSurface,
    time: u32,
) {
    smithay::wayland::compositor::with_surface_tree_downward(
        surface,
        (),
        |_, _, _| smithay::wayland::compositor::TraversalAction::DoChildren(()),
        |surface, states, _| {
            let mut guard = states
                .cached_state
                .get::<smithay::wayland::compositor::SurfaceAttributes>();
             
            for callback in guard.current().frame_callbacks.drain(..) {
                callback.done(time);
            }
        },
        |_, _, _| true,
    );
}
