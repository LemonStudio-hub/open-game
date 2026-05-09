use wasm_bindgen::JsCast;

#[derive(Debug, Clone)]
pub struct GamepadState {
    pub id: String,
    pub index: u32,
    pub connected: bool,
    pub buttons: Vec<bool>,
    pub axes: Vec<f32>,
}

impl GamepadState {
    pub fn new(id: String, index: u32) -> Self {
        Self {
            id,
            index,
            connected: true,
            buttons: Vec::new(),
            axes: Vec::new(),
        }
    }
}

pub struct GamepadManager {
    gamepads: Vec<Option<GamepadState>>,
    navigator: web_sys::Navigator,
}

impl GamepadManager {
    pub fn new(navigator: web_sys::Navigator) -> Self {
        Self {
            gamepads: vec![None; 4],
            navigator,
        }
    }

    pub fn update(&mut self) {
        if let Ok(gamepads) = self.navigator.get_gamepads() {
            for i in 0..gamepads.length().min(4) {
                if let Ok(gp_obj) = gamepads.get(i).dyn_into::<web_sys::Gamepad>() {
                    let index = gp_obj.index() as usize;
                    if index >= self.gamepads.len() {
                        continue;
                    }

                    if gp_obj.connected() {
                        let mut state = GamepadState::new(gp_obj.id(), gp_obj.index());
                        state.connected = true;

                        let buttons = gp_obj.buttons();
                        state.buttons = Vec::with_capacity(buttons.length() as usize);
                        for j in 0..buttons.length() {
                            if let Ok(btn) = buttons.get(j).dyn_into::<web_sys::GamepadButton>() {
                                state.buttons.push(btn.pressed());
                            }
                        }

                        let axes = gp_obj.axes();
                        state.axes = Vec::with_capacity(axes.length() as usize);
                        for j in 0..axes.length() {
                            if let Some(val) = axes.get(j).as_f64() {
                                state.axes.push(val as f32);
                            }
                        }

                        self.gamepads[index] = Some(state);
                    } else {
                        self.gamepads[index] = None;
                    }
                }
            }
        }
    }

    pub fn get(&self, index: usize) -> Option<&GamepadState> {
        self.gamepads.get(index)?.as_ref()
    }

    pub fn is_button_down(&self, gamepad_index: usize, button_index: usize) -> bool {
        self.gamepads
            .get(gamepad_index)
            .and_then(|gp| gp.as_ref())
            .and_then(|gp| gp.buttons.get(button_index))
            .copied()
            .unwrap_or(false)
    }

    pub fn axis(&self, gamepad_index: usize, axis_index: usize) -> f32 {
        self.gamepads
            .get(gamepad_index)
            .and_then(|gp| gp.as_ref())
            .and_then(|gp| gp.axes.get(axis_index))
            .copied()
            .unwrap_or(0.0)
    }

    pub fn connected_count(&self) -> usize {
        self.gamepads.iter().filter_map(|gp| gp.as_ref()).filter(|gp| gp.connected).count()
    }
}
