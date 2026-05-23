use super::transition::Transition;
use std::any::{Any, TypeId};
use std::collections::HashMap;

pub trait Scene: Any {
    fn on_enter(&mut self, _ctx: &mut SceneContext) {}
    fn on_exit(&mut self, _ctx: &mut SceneContext) {}
    fn update(&mut self, ctx: &mut SceneContext, dt: f32);
    fn render(&mut self, ctx: &mut SceneContext, alpha: f32);
}

pub struct SceneContext {
    pub data: HashMap<TypeId, Box<dyn Any>>,
}

impl Default for SceneContext {
    fn default() -> Self {
        Self::new()
    }
}

impl SceneContext {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn set<T: 'static>(&mut self, value: T) {
        self.data.insert(TypeId::of::<T>(), Box::new(value));
    }

    pub fn get<T: 'static>(&self) -> Option<&T> {
        self.data.get(&TypeId::of::<T>())?.downcast_ref()
    }

    pub fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.data.get_mut(&TypeId::of::<T>())?.downcast_mut()
    }
}

pub struct SceneManager {
    scenes: Vec<Box<dyn Scene>>,
    transition: Option<Transition>,
    pending_action: Option<SceneAction>,
}

enum SceneAction {
    Push(Box<dyn Scene>),
    Pop,
    Switch(Box<dyn Scene>),
}

impl SceneManager {
    pub fn new() -> Self {
        Self {
            scenes: Vec::new(),
            transition: None,
            pending_action: None,
        }
    }

    pub fn push(&mut self, scene: impl Scene + 'static) {
        self.pending_action = Some(SceneAction::Push(Box::new(scene)));
    }

    pub fn pop(&mut self) {
        self.pending_action = Some(SceneAction::Pop);
    }

    pub fn switch(&mut self, scene: impl Scene + 'static) {
        self.pending_action = Some(SceneAction::Switch(Box::new(scene)));
    }

    pub fn current(&self) -> Option<&dyn Scene> {
        self.scenes.last().map(|s| s.as_ref())
    }

    pub fn current_mut(&mut self) -> Option<&mut dyn Scene> {
        self.scenes.last_mut().map(|s| s.as_mut())
    }

    pub fn depth(&self) -> usize {
        self.scenes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.scenes.is_empty()
    }

    pub fn update_transition(&mut self) -> bool {
        if let Some(transition) = &mut self.transition {
            if transition.is_finished() {
                self.transition = None;
                return true;
            }
        }
        false
    }

    pub fn update(&mut self, ctx: &mut SceneContext, dt: f32) {
        if let Some(action) = self.pending_action.take() {
            match action {
                SceneAction::Push(mut scene) => {
                    scene.on_enter(ctx);
                    self.scenes.push(scene);
                }
                SceneAction::Pop => {
                    if let Some(mut scene) = self.scenes.pop() {
                        scene.on_exit(ctx);
                    }
                }
                SceneAction::Switch(mut new_scene) => {
                    if let Some(mut old_scene) = self.scenes.pop() {
                        old_scene.on_exit(ctx);
                    }
                    new_scene.on_enter(ctx);
                    self.scenes.push(new_scene);
                }
            }
        }

        if let Some(scene) = self.scenes.last_mut() {
            scene.update(ctx, dt);
        }
    }

    pub fn render(&mut self, ctx: &mut SceneContext, alpha: f32) {
        if let Some(scene) = self.scenes.last_mut() {
            scene.render(ctx, alpha);
        }
    }
}

impl Default for SceneManager {
    fn default() -> Self {
        Self::new()
    }
}
