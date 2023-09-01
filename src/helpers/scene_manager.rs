use std::rc::Rc;

use sfml::{
    graphics::{Color, RenderTarget, RenderWindow},
    system::Clock,
    window::Event,
    SfBox,
};

use crate::scenes::test_scene::PreInitializedTestScene;

use super::{
    build_window::build_window,
    font_loader::{load_fonts, LoadedFonts},
    input_state::InputState,
    scene::{PreInitializedScene, Scene},
};

pub struct SceneManager {
    window: RenderWindow,
    fonts: LoadedFonts,
    input_state: InputState,
    clock: SfBox<Clock>,
    last_delta_time: f32,
    current_scene: Box<dyn Scene>,
}

impl SceneManager {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        let fonts = load_fonts();
        let mut window = build_window(title, width, height);
        window.set_vertical_sync_enabled(true);
        return Self {
            window,
            fonts: Rc::clone(&fonts),
            input_state: InputState::new(),
            clock: Clock::start(),
            last_delta_time: 0.0,
            current_scene: (PreInitializedTestScene {}).init_graphics(Rc::clone(&fonts)),
        };
    }

    pub fn is_running(&self) -> bool {
        return self.window.is_open();
    }

    pub fn update(&mut self) -> () {
        if !self.is_running() {
            return;
        }
        self.draw();
        let delta_time = self.get_delta_time();
        self.update_input_state();
        if self.is_running() {
            self.update_scene(delta_time);
        }
    }

    fn draw(&mut self) -> () {
        self.window.clear(Color::BLACK);
        self.current_scene.draw(&mut self.window);
        self.window.display();
    }

    fn update_scene(&mut self, delta_time: f32) -> () {
        self.current_scene
            .update_state(&self.input_state, delta_time, self.last_delta_time);
        self.last_delta_time = delta_time;
    }

    fn update_input_state(&mut self) {
        self.input_state.clear_state();
        while let Some(event) = self.window.poll_event() {
            let matched_event = self.input_state.on_sfml_event(event);
            if !matched_event && event == Event::Closed {
                self.window.close();
            }
        }
    }

    fn get_delta_time(&mut self) -> f32 {
        let delta_time = self.clock.elapsed_time().as_seconds();
        self.clock.restart();
        return delta_time;
    }

    pub fn temp_get_input_state(&mut self) -> &mut InputState {
        return &mut self.input_state;
    }
}
