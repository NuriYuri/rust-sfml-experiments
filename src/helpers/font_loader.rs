use sfml::{graphics::Font, SfBox};

pub struct Fonts {
    pub main_font: SfBox<Font>,
}

pub fn load_fonts() -> Box<Fonts> {
    let main_font = Font::from_file("assets/fonts/PublicSans-Regular.ttf")
        .unwrap_or_else(|| panic!("Failed to load PublicSans-Regular Font"));
    return Box::new(Fonts { main_font });
}
