pub mod helpers;
pub mod scenes;

use helpers::scene_manager::SceneManager;
use sfml::window::mouse::Button as MouseButton;
use sfml::window::Scancode;

fn main() {
    let mut scene_manager = SceneManager::new("test", 640, 360);

    scene_manager
        .temp_get_input_state()
        .load_scan_code_mapping(Scancode::Enter, 0);
    scene_manager
        .temp_get_input_state()
        .load_mouse_button_mapping(MouseButton::Left, 0);

    while scene_manager.is_running() {
        scene_manager.update();
    }
}
