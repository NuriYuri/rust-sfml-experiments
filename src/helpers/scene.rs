use sfml::graphics::RenderTarget;

use super::{font_loader::Fonts, input_state::InputState};

/// Trait describing a game scene.
///
/// A game scene is responsive of handling its own state and draw things to the screen.
///
/// Here's the life cycle of a scene:
/// 1. init_graphics (load all the resources and initialize all the sprites based on initial state)
/// 2. init_animation (init_graphics is expensive so animation must be initialized after)
/// 3. draw (draw the sprites of the scene based on the state)
/// 4. loop
///    1. update_state (update the state of the scene)
///    2. draw (draw the sprites of the scene based on the state)
///
/// Note: the scene is responsive of calling clear on the surface
pub trait Scene<'lifetime: 'b, 'b> {
    fn init_graphics(&mut self, fonts: &'lifetime Box<Fonts>) -> ();
    fn init_animation(&mut self) -> ();
    fn draw(&self, surface: &mut dyn RenderTarget) -> ();
    /// Update the state based on inputs
    /// ### Arguments
    /// - input_state: state of the user inputs
    /// - delta_time: Time elapsed for last frame in seconds
    /// - last_delta_time: Time elapsed for previous last frame in seconds
    fn update_state(
        &mut self,
        input_state: &InputState,
        delta_time: f32,
        last_delta_time: f32,
    ) -> ();
}
