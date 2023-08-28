use sfml::{
    graphics::RenderWindow,
    window::{ContextSettings, Style, VideoMode},
};

static SETTINGS: ContextSettings = ContextSettings {
    depth_bits: 0,
    stencil_bits: 0,
    antialiasing_level: 0,
    major_version: 3,
    minor_version: 2,
    attribute_flags: ContextSettings::ATTRIB_DEFAULT,
    srgb_capable: false,
};

pub fn build_window(title: &str, width: u32, height: u32) -> RenderWindow {
    return RenderWindow::new(
        VideoMode::new(width, height, 32),
        title,
        Style::CLOSE,
        &SETTINGS,
    );
}
