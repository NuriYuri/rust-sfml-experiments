use sfml::{
    graphics::{RcText, Transformable},
    system::Vector2f,
};

use crate::helpers::{
    font_loader::LoadedFonts,
    scene::{PreInitializedScene, Scene, UpdateSceneResult, VoidScene},
};

pub struct PreInitializedTestSceneDeep {}

impl PreInitializedScene for PreInitializedTestSceneDeep {
    fn init_graphics(
        &self,
        fonts: LoadedFonts,
        previous_scene: Option<Box<dyn Scene>>,
    ) -> Box<dyn Scene> {
        let mut text = RcText::new("Deep Scene!", &(fonts.borrow_mut().main_font), 16);
        text.set_position(Vector2f::new(0.0, 32.0));
        return Box::new(TestSceneDeep {
            previous_scene: previous_scene.expect("Need a previous scene to work"),
            text,
        });
    }
}

struct TestSceneDeep {
    previous_scene: Box<dyn Scene>,
    text: RcText,
}

impl Scene for TestSceneDeep {
    fn start_animations(&mut self) -> () {}

    fn draw(&self, surface: &mut dyn sfml::graphics::RenderTarget) -> () {
        self.previous_scene.draw(surface);
        surface.draw(&self.text);
    }

    fn update_state(
        &mut self,
        input_state: &crate::helpers::input_state::InputState,
        delta_time: f32,
        last_delta_time: f32,
    ) -> UpdateSceneResult {
        self.previous_scene
            .update_state(input_state, delta_time, last_delta_time);
        if input_state.get_key_state(1) {
            let mut scene: Box<dyn Scene> = Box::new(VoidScene {});
            std::mem::swap(&mut scene, &mut self.previous_scene);
            return UpdateSceneResult::SwitchToExistingScene(scene);
        }
        return UpdateSceneResult::Continue;
    }
}
