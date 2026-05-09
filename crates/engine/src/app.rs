use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::ecs::world::World;
use crate::ecs::system::SystemScheduler;
use crate::renderer::{GlBackend, Camera2D, SpriteRenderer, ShapeRenderer, TextureManager};
use crate::input::InputManager;
use crate::audio::AudioEngine;
use crate::time::Time;
use crate::event::EventBus;
use crate::scene::{SceneManager, SceneContext};
use crate::color::Color;

pub struct App {
    pub world: World,
    scheduler: SystemScheduler,
    pub renderer: Renderer,
    pub input: InputManager,
    pub audio: AudioEngine,
    pub time: Time,
    pub events: EventBus,
    pub scene_manager: SceneManager,
    pub scene_context: SceneContext,
    clear_color: Color,
    #[allow(dead_code)]
    canvas_id: String,
}

pub struct Renderer {
    pub gl: GlBackend,
    pub camera: Camera2D,
    pub sprite_renderer: SpriteRenderer,
    pub shape_renderer: ShapeRenderer,
    pub texture_manager: TextureManager,
}

impl App {
    pub fn new(canvas_id: &str) -> Result<Self, String> {
        crate::log::init();

        let window = web_sys::window().ok_or("No window")?;
        let document = window.document().ok_or("No document")?;

        let canvas = document
            .get_element_by_id(canvas_id)
            .ok_or(format!("Canvas '{}' not found", canvas_id))?
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| "Element is not a canvas")?;

        canvas.set_width(canvas.client_width() as u32);
        canvas.set_height(canvas.client_height() as u32);

        let gl_backend = GlBackend::new(canvas_id)?;
        let camera = Camera2D::new(gl_backend.width() as f32, gl_backend.height() as f32);
        let sprite_renderer = SpriteRenderer::new(gl_backend.gl())?;
        let shape_renderer = ShapeRenderer::new(gl_backend.gl())?;

        let mut texture_manager = TextureManager::new();
        texture_manager.init(gl_backend.gl());

        let renderer = Renderer {
            gl: gl_backend,
            camera,
            sprite_renderer,
            shape_renderer,
            texture_manager,
        };

        let performance = window.performance().ok_or("No performance API")?;
        let time = Time::new(performance);
        let input = InputManager::new()?;
        let audio = AudioEngine::new()?;
        let events = EventBus::new();
        let scene_manager = SceneManager::new();
        let scene_context = SceneContext::new();
        let world = World::new();
        let scheduler = SystemScheduler::new();

        Ok(Self {
            world,
            scheduler,
            renderer,
            input,
            audio,
            time,
            events,
            scene_manager,
            scene_context,
            clear_color: Color::BLACK,
            canvas_id: canvas_id.to_string(),
        })
    }

    pub fn set_clear_color(&mut self, color: Color) {
        self.clear_color = color;
    }

    pub fn add_startup_system(&mut self, system: impl FnMut(&mut World) + 'static) {
        self.scheduler.add_startup_system(system);
    }

    pub fn add_system(&mut self, system: impl crate::ecs::system::System + 'static) {
        self.scheduler.add_system(system);
    }

    pub fn add_render_system(&mut self, system: impl FnMut(&mut World, f32) + 'static) {
        self.scheduler.add_render_system(system);
    }

    fn tick_update(&mut self, dt: f32) {
        self.scheduler.run_update(&mut self.world, dt);
        self.scene_manager.update(&mut self.scene_context, dt);
    }

    fn tick_render(&mut self, alpha: f32) {
        self.scheduler.run_render(&mut self.world, alpha);
        self.scene_manager.render(&mut self.scene_context, alpha);
    }

    pub fn run(mut self) {
        self.scheduler.run_startup(&mut self.world);
        self.time.init();

        let app = Rc::new(RefCell::new(self));
        let f: Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>> = Rc::new(RefCell::new(None));
        let g = f.clone();
        let app_clone = app.clone();

        let mut last_time = 0.0_f64;

        *g.borrow_mut() = Some(Closure::new(move |timestamp: f64| {
            let _dt = if last_time == 0.0 {
                1.0 / 60.0
            } else {
                ((timestamp - last_time) / 1000.0).min(0.25) as f32
            };
            last_time = timestamp;

            let mut app = app_clone.borrow_mut();

            app.time.update();
            app.input.update();

            let fixed_dt = app.time.fixed_timestep();
            while app.time.consume_fixed_step() {
                app.tick_update(fixed_dt);
            }

            let alpha = app.time.alpha();
            let (cr, cg, cb, ca) = (app.clear_color.r, app.clear_color.g, app.clear_color.b, app.clear_color.a);

            app.renderer.gl.resize();
            let width = app.renderer.gl.width() as f32;
            let height = app.renderer.gl.height() as f32;
            app.renderer.camera.set_viewport(width, height);

            app.renderer.gl.clear(cr, cg, cb, ca);
            app.renderer.gl.enable_blend();

            app.tick_render(alpha);

            drop(app);

            request_animation_frame(f.borrow().as_ref().unwrap());
        }));

        request_animation_frame(g.borrow().as_ref().unwrap());
    }
}

fn request_animation_frame(f: &Closure<dyn FnMut(f64)>) {
    web_sys::window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .unwrap();
}

pub struct Commands<'a> {
    world: &'a mut World,
}

impl<'a> Commands<'a> {
    pub fn new(world: &'a mut World) -> Self {
        Self { world }
    }

    pub fn spawn(&mut self) -> crate::ecs::builder::EntityBuilder<'_> {
        self.world.spawn()
    }

    pub fn despawn(&mut self, entity: crate::ecs::Entity) -> bool {
        self.world.despawn(entity)
    }

    pub fn insert_resource<T: 'static>(&mut self, resource: T) {
        self.world.insert_resource(resource);
    }
}
