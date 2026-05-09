pub mod keys;
pub mod keyboard;
pub mod mouse;
pub mod touch;
pub mod gamepad;

use crate::math::Vec2;
use keys::{KeyCode, MouseButton};
use keyboard::KeyboardState;
use mouse::MouseState;
use touch::TouchState;
use gamepad::GamepadManager;

use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

type Closures = Vec<Closure<dyn FnMut(web_sys::Event)>>;

pub struct InputManager {
    pub keyboard: KeyboardState,
    pub mouse: MouseState,
    pub touch: TouchState,
    pub gamepad: GamepadManager,
    _closures: Closures,
    _keyboard_ref: Rc<RefCell<KeyboardState>>,
    _mouse_ref: Rc<RefCell<MouseState>>,
    _touch_ref: Rc<RefCell<TouchState>>,
}

fn add_listener(
    target: &web_sys::EventTarget,
    event_name: &str,
    closures: &mut Closures,
    callback: impl FnMut(web_sys::Event) + 'static,
) {
    let closure = Closure::wrap(Box::new(callback) as Box<dyn FnMut(web_sys::Event)>);
    target
        .add_event_listener_with_callback(event_name, closure.as_ref().unchecked_ref())
        .unwrap();
    closures.push(closure);
}

impl InputManager {
    pub fn new() -> Result<Self, String> {
        let window = web_sys::window().ok_or("No window")?;
        let document = window.document().ok_or("No document")?;
        let navigator = window.navigator();

        let keyboard = Rc::new(RefCell::new(KeyboardState::new()));
        let mouse = Rc::new(RefCell::new(MouseState::new()));
        let touch = Rc::new(RefCell::new(TouchState::new()));

        let mut closures: Closures = Vec::new();

        {
            let kb = keyboard.clone();
            add_listener(&document, "keydown", &mut closures, move |event| {
                let event = event.unchecked_ref::<web_sys::KeyboardEvent>();
                let key = KeyCode::from_code(&event.code());
                kb.borrow_mut().on_key_down(key);
            });
        }

        {
            let kb = keyboard.clone();
            add_listener(&document, "keyup", &mut closures, move |event| {
                let event = event.unchecked_ref::<web_sys::KeyboardEvent>();
                let key = KeyCode::from_code(&event.code());
                kb.borrow_mut().on_key_up(key);
            });
        }

        {
            let m = mouse.clone();
            add_listener(&document, "mousemove", &mut closures, move |event| {
                let event = event.unchecked_ref::<web_sys::MouseEvent>();
                m.borrow_mut().on_move(event.offset_x() as f32, event.offset_y() as f32);
                m.borrow_mut().on_move_delta(event.movement_x() as f32, event.movement_y() as f32);
            });
        }

        {
            let m = mouse.clone();
            add_listener(&document, "mousedown", &mut closures, move |event| {
                let event = event.unchecked_ref::<web_sys::MouseEvent>();
                let button = MouseButton::from_index(event.button());
                m.borrow_mut().on_button_down(button);
            });
        }

        {
            let m = mouse.clone();
            add_listener(&document, "mouseup", &mut closures, move |event| {
                let event = event.unchecked_ref::<web_sys::MouseEvent>();
                let button = MouseButton::from_index(event.button());
                m.borrow_mut().on_button_up(button);
            });
        }

        {
            let m = mouse.clone();
            add_listener(&document, "wheel", &mut closures, move |event| {
                let event = event.unchecked_ref::<web_sys::WheelEvent>();
                m.borrow_mut().on_wheel(event.delta_y() as f32);
            });
        }

        {
            let t = touch.clone();
            add_listener(&document, "touchstart", &mut closures, move |event| {
                event.prevent_default();
                let event = event.unchecked_ref::<web_sys::TouchEvent>();
                let touches = event.changed_touches();
                for i in 0..touches.length() {
                    if let Some(touch_point) = touches.get(i) {
                        t.borrow_mut().on_touch_start(
                            touch_point.identifier(),
                            touch_point.client_x() as f32,
                            touch_point.client_y() as f32,
                            touch_point.force(),
                        );
                    }
                }
            });
        }

        {
            let t = touch.clone();
            add_listener(&document, "touchmove", &mut closures, move |event| {
                event.prevent_default();
                let event = event.unchecked_ref::<web_sys::TouchEvent>();
                let touches = event.changed_touches();
                for i in 0..touches.length() {
                    if let Some(touch_point) = touches.get(i) {
                        t.borrow_mut().on_touch_move(
                            touch_point.identifier(),
                            touch_point.client_x() as f32,
                            touch_point.client_y() as f32,
                            touch_point.force(),
                        );
                    }
                }
            });
        }

        {
            let t = touch.clone();
            add_listener(&document, "touchend", &mut closures, move |event| {
                let event = event.unchecked_ref::<web_sys::TouchEvent>();
                let touches = event.changed_touches();
                for i in 0..touches.length() {
                    if let Some(touch_point) = touches.get(i) {
                        t.borrow_mut().on_touch_end(touch_point.identifier());
                    }
                }
            });
        }

        let gamepad = GamepadManager::new(navigator);

        Ok(Self {
            keyboard: KeyboardState::new(),
            mouse: MouseState::new(),
            touch: TouchState::new(),
            gamepad,
            _closures: closures,
            _keyboard_ref: keyboard,
            _mouse_ref: mouse,
            _touch_ref: touch,
        })
    }

    pub fn update(&mut self) {
        if let Ok(kb) = self._keyboard_ref.try_borrow() {
            self.keyboard.keys_down = kb.keys_down.clone();
            self.keyboard.keys_pressed = kb.keys_pressed.clone();
            self.keyboard.keys_released = kb.keys_released.clone();
        }
        self.keyboard.update();

        if let Ok(m) = self._mouse_ref.try_borrow() {
            self.mouse.position = m.position;
            self.mouse.buttons_down = m.buttons_down.clone();
            self.mouse.buttons_pressed = m.buttons_pressed.clone();
            self.mouse.buttons_released = m.buttons_released.clone();
        }
        self.mouse.update();

        if let Ok(t) = self._touch_ref.try_borrow() {
            self.touch.touches = t.touches.clone();
        }
        self.touch.update();

        self.gamepad.update();
    }

    pub fn mouse_position(&self) -> Vec2 {
        self.mouse.position()
    }

    pub fn is_key_down(&self, key: KeyCode) -> bool {
        self.keyboard.is_key_down(key)
    }

    pub fn is_key_pressed(&self, key: KeyCode) -> bool {
        self.keyboard.is_key_pressed(key)
    }

    pub fn is_key_released(&self, key: KeyCode) -> bool {
        self.keyboard.is_key_released(key)
    }

    pub fn is_mouse_down(&self, button: MouseButton) -> bool {
        self.mouse.is_button_down(button)
    }

    pub fn is_mouse_pressed(&self, button: MouseButton) -> bool {
        self.mouse.is_button_pressed(button)
    }

    pub fn mouse_wheel(&self) -> f32 {
        self.mouse.wheel_delta()
    }
}
