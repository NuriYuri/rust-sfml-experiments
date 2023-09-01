use std::{cell::RefCell, rc::Rc};

use sfml::graphics::RcFont;

pub struct Fonts {
    pub main_font: RcFont,
}

pub type LoadedFonts = Rc<RefCell<Fonts>>;

pub fn load_fonts() -> LoadedFonts {
    let main_font = RcFont::from_file("assets/fonts/PublicSans-Regular.ttf")
        .unwrap_or_else(|| panic!("Failed to load PublicSans-Regular Font"));
    return Rc::new(RefCell::new(Fonts { main_font }));
}
