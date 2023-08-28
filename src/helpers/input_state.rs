use sfml::window::{
    mouse::{Button as MouseButton, Wheel},
    Event, Scancode,
};

const MAX_KEY_STATE: usize = 8;
const MAX_SCAN_CODE_MAPPING: usize = Scancode::ScancodeCount as usize;
const MAX_MOUSE_BUTTON: usize = 5;

/// Structure responsive of holding the input state and sfml input to virtual key mapping
pub struct InputState {
    key_state: [bool; MAX_KEY_STATE],
    scan_code_mapping: [usize; MAX_SCAN_CODE_MAPPING],
    mouse_x: i32,
    mouse_y: i32,
    wheel_delta: f32,
    mouse_button_mapping: [usize; MAX_MOUSE_BUTTON],
    last_input_char: Vec<char>,
}

impl InputState {
    pub fn new() -> Self {
        return Self {
            key_state: [false; MAX_KEY_STATE],
            scan_code_mapping: [MAX_KEY_STATE; MAX_SCAN_CODE_MAPPING],
            mouse_x: 0,
            mouse_y: 0,
            wheel_delta: 0.0,
            mouse_button_mapping: [MAX_KEY_STATE; MAX_MOUSE_BUTTON],
            last_input_char: Vec::new(),
        };
    }

    pub fn load_scan_code_table(&mut self, table: [usize; MAX_SCAN_CODE_MAPPING]) {
        self.scan_code_mapping = table.clamp(
            [0; MAX_SCAN_CODE_MAPPING],
            [MAX_KEY_STATE; MAX_SCAN_CODE_MAPPING],
        );
    }

    pub fn load_scan_code_mapping(&mut self, scancode: Scancode, key_index: usize) {
        if scancode == Scancode::Unknown
            || scancode == Scancode::ScancodeCount
            || key_index >= MAX_KEY_STATE
        {
            return;
        }

        self.scan_code_mapping[scancode as usize] = key_index;
    }

    pub fn get_key_index_from_scan_code(&self, scancode: Scancode) -> usize {
        if scancode == Scancode::Unknown || scancode == Scancode::ScancodeCount {
            return MAX_KEY_STATE;
        }

        return self.scan_code_mapping[scancode as usize];
    }

    fn on_scancode_pressed(&mut self, scancode: Scancode) {
        let key_index = self.get_key_index_from_scan_code(scancode);
        if key_index < MAX_KEY_STATE {
            self.key_state[key_index] = true;
        }
    }

    fn on_scancode_released(&mut self, scancode: Scancode) {
        let key_index = self.get_key_index_from_scan_code(scancode);
        if key_index < MAX_KEY_STATE {
            self.key_state[key_index] = false;
        }
    }

    pub fn get_key_state(&self, key_index: usize) -> bool {
        if key_index < MAX_KEY_STATE {
            return self.key_state[key_index];
        }

        return false;
    }

    pub fn load_mouse_button_table(&mut self, table: [usize; MAX_MOUSE_BUTTON]) {
        self.mouse_button_mapping =
            table.clamp([0; MAX_MOUSE_BUTTON], [MAX_KEY_STATE; MAX_MOUSE_BUTTON]);
    }

    pub fn load_mouse_button_mapping(&mut self, button: MouseButton, key_index: usize) {
        self.mouse_button_mapping[button as usize] = key_index;
    }

    pub fn get_mouse_x(&self) -> i32 {
        return self.mouse_x;
    }

    pub fn get_mouse_y(&self) -> i32 {
        return self.mouse_y;
    }

    pub fn get_wheel_delta(&self) -> f32 {
        return self.wheel_delta;
    }

    fn on_wheel_scrolled(&mut self, delta: f32) {
        self.wheel_delta = delta;
    }

    fn on_mouse_moved(&mut self, x: i32, y: i32) {
        self.mouse_x = x;
        self.mouse_y = y;
    }

    pub fn get_key_index_from_mouse_button(&self, button: MouseButton) -> usize {
        return self.mouse_button_mapping[button as usize];
    }

    fn on_mouse_button_pressed(&mut self, button: MouseButton) {
        let key_index = self.get_key_index_from_mouse_button(button);
        if key_index < MAX_KEY_STATE {
            self.key_state[key_index] = true;
        }
    }

    fn on_mouse_button_released(&mut self, button: MouseButton) {
        let key_index = self.get_key_index_from_mouse_button(button);
        if key_index < MAX_KEY_STATE {
            self.key_state[key_index] = false;
        }
    }

    /// Function responsive of matching relevant SFML events related to input state
    ///
    /// Returns false if none were matched
    pub fn on_sfml_event(&mut self, event: Event) -> bool {
        match event {
            Event::KeyReleased { scan, .. } => self.on_scancode_released(scan),
            Event::KeyPressed { scan, .. } => self.on_scancode_pressed(scan),
            // Event::MouseLeft => self.on_mouse_moved(std::i32::MIN, std::i32::MIN),
            Event::MouseMoved { x, y } => self.on_mouse_moved(x, y),
            Event::MouseWheelScrolled { wheel, delta, .. } => {
                if wheel != Wheel::VerticalWheel {
                    return false;
                }
                self.on_wheel_scrolled(delta);
            }
            Event::MouseButtonPressed { button, .. } => {
                self.on_mouse_button_pressed(button);
            }
            Event::MouseButtonReleased { button, .. } => {
                self.on_mouse_button_released(button);
            }
            Event::TextEntered { unicode } => {
                self.last_input_char.push(unicode);
            }
            _ => {
                return false;
            }
        }
        return true;
    }

    /// Function that resets the wheel delta & unicode char since events only comes when they happen
    pub fn clear_state(&mut self) {
        self.wheel_delta = 0.0;
        self.last_input_char.clear();
    }
}
