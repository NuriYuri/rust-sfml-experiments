use sfml::graphics::{Color, RcText};

use crate::helpers::{
    font_loader::LoadedFonts,
    scene::{PreInitializedScene, Scene},
};

pub struct PreInitializedTestScene {}

impl PreInitializedScene for PreInitializedTestScene {
    fn init_graphics(&self, fonts: LoadedFonts) -> Box<dyn Scene> {
        let text = RcText::new("No last press", &(fonts.borrow_mut().main_font), 16);
        return Box::new(TestScene {
            last_wheel: 0.0,
            frame_count: 0,
            time_accu: 0.0,
            fps: 0,
            last_main_input: false,
            graphics: TestSceneGraphics { text },
        });
    }
}

struct TestSceneGraphics {
    text: RcText,
}

pub struct TestScene {
    last_wheel: f32,
    frame_count: i32,
    time_accu: f32,
    fps: i32,
    last_main_input: bool,
    graphics: TestSceneGraphics,
}

impl TestScene {
    fn update_fps_state(&mut self) {
        if self.frame_count >= 60 {
            self.fps = (self.time_accu * self.frame_count as f32) as i32;
            self.frame_count = 0;
            self.time_accu = 0.0;
        }
    }

    fn update_text(&mut self, main_input: bool, wheel: f32) {
        self.graphics.text.set_string(
            format!(
                "main: {:?} wheel: {:?} fps: {:?}",
                main_input, wheel, self.fps
            )
            .as_str(),
        )
    }
}

impl Scene for TestScene {
    fn start_animations(&mut self) -> () {}

    fn draw(&self, surface: &mut dyn sfml::graphics::RenderTarget) -> () {
        surface.clear(Color::BLACK);
        surface.draw(&self.graphics.text);
    }

    fn update_state(
        &mut self,
        input_state: &crate::helpers::input_state::InputState,
        delta_time: f32,
        _: f32,
    ) -> () {
        let main_input = input_state.get_key_state(0);
        let wheel = input_state.get_wheel_delta();
        self.update_fps_state();
        if self.last_main_input != main_input || self.last_wheel != wheel || self.frame_count == 0 {
            self.update_text(main_input, wheel);
        }
        self.last_main_input = main_input;
        self.last_wheel = wheel;
        self.frame_count += 1;
        self.time_accu += delta_time;
    }
}
