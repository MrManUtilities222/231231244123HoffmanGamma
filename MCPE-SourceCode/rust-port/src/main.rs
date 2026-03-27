use rust_port::ninecraft_app::NinecraftApp;
use rust_port::app::App;
use rust_port::renderer::GameRenderer;
use rust_port::screen::{GameState, Screen, PauseScreen};
use rust_port::font::Font;
use std::time::{Duration, Instant};
use std::thread;
use std::env;
use std::collections::HashSet;
use rust_port::local_player::LocalPlayer;

fn run_headless(mut app: NinecraftApp) {
    println!("No display detected — running headless for 5 seconds.");
    let start = Instant::now();
    while start.elapsed().as_secs() < 5 {
        app.update();
        thread::sleep(Duration::from_millis(50));
    }
    println!("Headless run finished.");
}

fn perspective_matrix(fovy_deg: f32, aspect: f32, near: f32, far: f32) -> [f32; 16] {
    let fovy = fovy_deg.to_radians();
    let f = 1.0 / (fovy / 2.0).tan();
    let nf = 1.0 / (near - far);

    [
        f / aspect, 0.0, 0.0, 0.0,
        0.0, f, 0.0, 0.0,
        0.0, 0.0, (far + near) * nf, -1.0,
        0.0, 0.0, (2.0 * far * near) * nf, 0.0,
    ]
}

fn ortho_matrix(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> [f32; 16] {
    let rml = right - left;
    let tmb = top - bottom;
    let fmn = far - near;

    [
        2.0 / rml, 0.0, 0.0, 0.0,
        0.0, 2.0 / tmb, 0.0, 0.0,
        0.0, 0.0, -2.0 / fmn, 0.0,
        -(right + left) / rml, -(top + bottom) / tmb, -(far + near) / fmn, 1.0,
    ]
}

fn main() {
    // Detect display availability (avoid winit backend init in headless containers)
    let has_display = env::var("DISPLAY").is_ok() || env::var("WAYLAND_DISPLAY").is_ok();

    // Initialize app state early so both headless and GUI paths run the same logic
    let mut app = NinecraftApp::new();
    app.init();

    if !has_display {
        // Headless mode for CI/container environments without X/Wayland
        run_headless(app);
        return;
    }

    // GUI path (display present)
    use winit::{
        event::{Event, WindowEvent, DeviceEvent, KeyboardInput, ElementState, VirtualKeyCode, MouseButton},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
    };

    let event_loop = EventLoop::new();
    let wb = WindowBuilder::new()
        .with_title("RustCraft")
        .with_inner_size(winit::dpi::LogicalSize::new(800.0, 600.0));

    // Build a GL context with glutin + winit
    let windowed_context = {
        use glutin::ContextBuilder;
        let ctx = ContextBuilder::new()
            .with_vsync(true)
            .build_windowed(wb, &event_loop)
            .expect("Failed to create windowed GL context");
        unsafe { ctx.make_current().expect("Failed to make GL context current") }
    };
    windowed_context.window().set_cursor_grab(true).ok();
    windowed_context.window().set_cursor_visible(false);

    let gl = unsafe { glow::Context::from_loader_function(|s| windowed_context.get_proc_address(s) as *const _) };
    let gl = std::rc::Rc::new(gl);

    // Generate merged mesh vertices from level (y = 64 layer)
    let mesh_vertices: Vec<f32> = if let Some(level) = app.minecraft_mut().level_ref() {
        // generate mesh across a small chunk radius around origin
        rust_port::mesh::generate_chunked_mesh_vertices(level, 2)
    } else {
        Vec::new()
    };

    let mut renderer = GameRenderer::new(Some(gl.clone()));
    let mut gui = rust_port::gui::Gui::new();
    let font = Font::new("../data/images/font/default8.png");
    let mut pause_screen = PauseScreen::new();
    let mut options_screen = rust_port::screen::OptionsScreen::new();
    let mut game_state = GameState::Playing;
    let mut mx = 0.0f32;
    let mut my = 0.0f32;

    // Upload generated mesh to the GPU
    if !mesh_vertices.is_empty() {
        renderer.upload_mesh(&mesh_vertices);
    }

    // Setup player state
    let mut player = LocalPlayer::new(false);
    player.player.mob.entity.core.set_pos(0.0, 70.0, 5.0);
    
    let mut keys: HashSet<VirtualKeyCode> = HashSet::new();
    let mut last_frame = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                app.quit();
                *control_flow = ControlFlow::Exit;
            }
            Event::WindowEvent { event: WindowEvent::KeyboardInput { input: KeyboardInput { virtual_keycode: Some(key), state, .. }, .. }, .. } => {
                match state {
                    ElementState::Pressed => {
                        if key == VirtualKeyCode::Escape {
                            game_state = match game_state {
                                GameState::Playing => {
                                    windowed_context.window().set_cursor_grab(false).ok();
                                    windowed_context.window().set_cursor_visible(true);
                                    let size = windowed_context.window().inner_size();
                                    pause_screen.init(size.width as f32, size.height as f32);
                                    options_screen.init(size.width as f32, size.height as f32);
                                    GameState::Paused
                                }
                                GameState::Paused | GameState::Options => {
                                    windowed_context.window().set_cursor_grab(true).ok();
                                    windowed_context.window().set_cursor_visible(false);
                                    GameState::Playing
                                }
                                _ => GameState::Playing,
                            };
                        }
                        keys.insert(key);
                        match key {
                            VirtualKeyCode::Key1 => player.player.inventory.select_slot(0),
                            VirtualKeyCode::Key2 => player.player.inventory.select_slot(1),
                            VirtualKeyCode::Key3 => player.player.inventory.select_slot(2),
                            VirtualKeyCode::Key4 => player.player.inventory.select_slot(3),
                            VirtualKeyCode::Key5 => player.player.inventory.select_slot(4),
                            VirtualKeyCode::Key6 => player.player.inventory.select_slot(5),
                            VirtualKeyCode::Key7 => player.player.inventory.select_slot(6),
                            VirtualKeyCode::Key8 => player.player.inventory.select_slot(7),
                            VirtualKeyCode::Key9 => player.player.inventory.select_slot(8),
                            _ => {}
                        }
                    }
                    ElementState::Released => { keys.remove(&key); }
                }
            }
            Event::DeviceEvent { event: DeviceEvent::MouseMotion { delta }, .. } => {
                if game_state == GameState::Playing {
                    let dx = delta.0 as f32;
                    let dy = delta.1 as f32;
                    player.player.mob.entity.core.y_rot -= dx * 0.2;
                    player.player.mob.entity.core.x_rot -= dy * 0.2;
                    if player.player.mob.entity.core.x_rot > 89.0 { player.player.mob.entity.core.x_rot = 89.0; }
                    if player.player.mob.entity.core.x_rot < -89.0 { player.player.mob.entity.core.x_rot = -89.0; }
                }
            }
            Event::WindowEvent { event: WindowEvent::CursorMoved { position, .. }, .. } => {
                mx = position.x as f32;
                my = position.y as f32;
                let ev = rust_port::screen::InputEvent::MouseMove { x: mx, y: my };
                if game_state == GameState::Paused {
                    if let Some(rust_port::screen::ScreenAction::ChangeState(new_state)) = pause_screen.handle_input(ev) {
                        game_state = new_state;
                    }
                } else if game_state == GameState::Options {
                    if let Some(rust_port::screen::ScreenAction::ChangeState(new_state)) = options_screen.handle_input(ev) {
                        game_state = new_state;
                    }
                }
            }
            Event::WindowEvent { event: WindowEvent::MouseInput { state, button, .. }, .. } => {
                if game_state != GameState::Playing {
                    let b = match button { MouseButton::Left => 0, MouseButton::Right => 1, _ => 2 };
                    let ev = match state {
                        ElementState::Pressed => rust_port::screen::InputEvent::MouseClick { x: mx, y: my, button: b },
                        ElementState::Released => rust_port::screen::InputEvent::MouseRelease { x: mx, y: my, button: b },
                    };
                    
                    let action = if game_state == GameState::Paused {
                        pause_screen.handle_input(ev)
                    } else if game_state == GameState::Options {
                        options_screen.handle_input(ev)
                    } else { None };

                    if let Some(act) = action {
                        match act {
                            rust_port::screen::ScreenAction::CloseScreen => {
                                windowed_context.window().set_cursor_grab(true).ok();
                                windowed_context.window().set_cursor_visible(false);
                                game_state = GameState::Playing;
                            }
                            rust_port::screen::ScreenAction::ChangeState(new_state) => {
                                game_state = new_state;
                                if new_state == GameState::Playing {
                                    windowed_context.window().set_cursor_grab(true).ok();
                                    windowed_context.window().set_cursor_visible(false);
                                }
                            }
                            _ => {}
                        }
                    }
                } else if state == ElementState::Pressed {
                    let cx = player.player.mob.entity.core.x;
                    let cy = player.player.mob.entity.core.y + 1.62;
                    let cz = player.player.mob.entity.core.z;
                    
                    let yaw_rad = player.player.mob.entity.core.y_rot.to_radians();
                    let pitch_rad = player.player.mob.entity.core.x_rot.to_radians();
                    let dir_x = -yaw_rad.sin() * pitch_rad.cos();
                    let dir_y = -pitch_rad.sin();
                    let dir_z = yaw_rad.cos() * pitch_rad.cos();
                    
                    if let Some(level) = app.minecraft_mut().level_mut() {
                        let p1 = rust_port::vec3::Vec3::new(cx, cy, cz);
                        let p2 = rust_port::vec3::Vec3::new(cx + dir_x * 5.0, cy + dir_y * 5.0, cz + dir_z * 5.0);
                        if let Some(hit) = level.clip(p1, p2) {
                            if button == MouseButton::Left {
                                level.set_tile(hit.x, hit.y, hit.z, rust_port::tile::AIR.id);
                            } else if button == MouseButton::Right {
                                let mut px = hit.x;
                                let mut py = hit.y;
                                let mut pz = hit.z;
                                match hit.f {
                                    0 => py -= 1,
                                    1 => py += 1,
                                    2 => pz -= 1,
                                    3 => pz += 1,
                                    4 => px -= 1,
                                    5 => px += 1,
                                    _ => {}
                                }
                                if let Some(selected_item) = player.player.inventory.get_selected() {
                                    if !selected_item.is_null() && selected_item.id < 256 {
                                        level.set_tile(px, py, pz, selected_item.id);
                                    }
                                }
                            }
                            
                            // Regenerate mesh since we modified a block
                            let mesh_vertices = rust_port::mesh::generate_chunked_mesh_vertices(level, 2);
                            renderer.upload_mesh(&mesh_vertices);
                        }
                    }
                }
            }
            Event::MainEventsCleared => {
                let now = Instant::now();
                let _dt = (now - last_frame).as_secs_f32();
                last_frame = now;

                // Only update game logic when playing (freeze on pause)
                if game_state == GameState::Playing {
                    let mut forward = 0.0;
                    let mut strafe = 0.0;
                    if keys.contains(&VirtualKeyCode::W) { forward += 1.0; }
                    if keys.contains(&VirtualKeyCode::S) { forward -= 1.0; }
                    if keys.contains(&VirtualKeyCode::A) { strafe += 1.0; }
                    if keys.contains(&VirtualKeyCode::D) { strafe -= 1.0; }
                    
                    player.move_relative(forward, strafe, 0.2);
                    
                    if keys.contains(&VirtualKeyCode::Space) {
                        player.jump();
                    }

                    // Apply physics and collision
                    player.tick(app.minecraft_mut().level_ref());
                }

                app.update();
                windowed_context.window().request_redraw();
            }
            Event::RedrawRequested(_) => {
                let size = windowed_context.window().inner_size();
                let width = size.width as i32;
                let height = size.height as i32;
                renderer.set_viewport(width, height);
                let proj = perspective_matrix(60.0, size.width as f32 / size.height as f32, 0.1, 100.0);
                
                // Use player eye-height for rendering
                let cx = player.player.mob.entity.core.x;
                let cy = player.player.mob.entity.core.y + 1.62;
                let cz = player.player.mob.entity.core.z;
                
                // Use player rotation for rendering
                let yaw_rad = player.player.mob.entity.core.y_rot.to_radians();
                let pitch_rad = player.player.mob.entity.core.x_rot.to_radians();
                
                // Draw merged mesh
                renderer.render_scene(&proj, [cx, cy, cz], [yaw_rad, pitch_rad]);

                // Draw GUI
                gui.render(width as f32, height as f32, &player.player.inventory);
                let ortho = ortho_matrix(0.0, width as f32, height as f32, 0.0, -1.0, 1.0);
                renderer.render_gui(&ortho, &gui.gui_mesh, true);  // Use gui.png
                renderer.render_gui(&ortho, &gui.item_mesh, false); // Use terrain.png

                // Draw pause overlay if paused
                if game_state == GameState::Paused {
                    let (overlay_verts, text_verts) = pause_screen.render(&font, width as f32, height as f32, mx, my);
                    renderer.render_gui(&ortho, &overlay_verts, true);
                    renderer.render_gui_tex(&ortho, &text_verts, renderer.font_texture);
                } else if game_state == GameState::Options {
                    let (overlay_verts, text_verts) = options_screen.render(&font, width as f32, height as f32, mx, my);
                    renderer.render_gui(&ortho, &overlay_verts, true);
                    renderer.render_gui_tex(&ortho, &text_verts, renderer.font_texture);
                }

                windowed_context.swap_buffers().ok();
                app.draw();
            }
            _ => {}
        }
    });
}
