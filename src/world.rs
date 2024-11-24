use bevy_ecs::bundle::Bundle;
use bevy_ecs::prelude;
use bevy_ecs::prelude::EntityWorldMut;
use bevy_ecs::system::{BoxedSystem, IntoSystem};

pub trait DataWorld {
    fn run<S, I, R, T>(&mut self, system: S, input: I) -> R
    where
        S: IntoSystem<I, R, T>,
        R: 'static,
        I: 'static;

    fn run_boxed<R: 'static, I: 'static>(&mut self, system: &mut BoxedSystem<I, R>, input: I) -> R;

    fn get_resource<R: prelude::Resource + Clone>(&self) -> Option<R>;

    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityWorldMut;
}
