pub mod helpers;
pub mod scenes;

use helpers::build_window::build_window;
use helpers::font_loader;
use helpers::input_state::InputState;
use helpers::scene::PreInitializedScene;
use scenes::test_scene::PreInitializedTestScene;
use sfml::system::Clock;
use sfml::window::mouse::Button as MouseButton;
use sfml::window::{Event, Scancode};

fn main() {
    let mut window = build_window("test", 640, 360);
    let fonts = font_loader::load_fonts();
    let mut input_state = InputState::new();
    let preinit_scene = PreInitializedTestScene {};
    let mut clock = Clock::start();
    let mut last_delta_time: f32 = 0.0;

    window.set_vertical_sync_enabled(true);
    input_state.load_scan_code_mapping(Scancode::Enter, 0);
    input_state.load_mouse_button_mapping(MouseButton::Left, 0);

    let mut scene = preinit_scene.init_graphics(&fonts);

    while window.is_open() {
        scene.draw(&mut window);
        window.display();
        let delta_time = clock.elapsed_time().as_seconds();
        clock.restart();
        input_state.clear_state();
        while let Some(event) = window.poll_event() {
            let matched_event = input_state.on_sfml_event(event);
            if !matched_event && event == Event::Closed {
                window.close();
            }
        }
        scene.update_state(&input_state, delta_time, last_delta_time);
        last_delta_time = delta_time;
    }
}
