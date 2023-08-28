use sfml::graphics::RenderTarget;

use super::{font_loader::Fonts, input_state::InputState};

/// Trait describing a scene that is schedule-able for as next scene  but needs its graphics to be initialized
///
/// The init graphics function must return the final Scene with all its graphics so it can run.
///
/// Note: PreInitializedScene can carry arguments for the scene (eg. which entity it is about)
pub trait PreInitializedScene {
    fn init_graphics<'lifetime: 'b, 'b>(
        &self,
        fonts: &'lifetime Box<Fonts>,
    ) -> Box<dyn Scene<'lifetime, 'lifetime> + 'lifetime>;
}

/// Trait describing a game scene.
///
/// A game scene is responsive of handling its own state and draw things to the screen.
///
/// Here's the life cycle of a scene:
/// 1. start_animations (to avoid any bad delta issues animation are started before first draw)
/// 2. draw (draw the sprites of the scene based on the state)
/// 3. loop
///    1. update_state (update the state of the scene)
///    2. draw (draw the sprites of the scene based on the state)
///
/// Note: the scene is responsive of calling clear on the surface
pub trait Scene<'lifetime: 'b, 'b> {
    fn start_animations(&mut self) -> ();
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
