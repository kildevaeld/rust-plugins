use types::{Plugin, PluginLoader};
use uuid;

struct Instance<T> {
    id: uuid::Uuid,
    instance: T,
}

impl<T> Plugin<T> for Instance<T> {
    fn id(&self) -> &uuid::Uuid {
        &self.id
    }

    fn instance(&self) -> &T {
        &self.instance
    }
}

pub trait PluginManager<T> {
    plugins: Vec<Box<dyn Plugin<T>>>,
    loaders: Vec<Box<dyn PluginLoader<Item = T>>>,
}

impl<T> PluginManager<T> {
    pub fn new() -> PluginManager<T> {
        PluginManager {
            plugins: vec![],
            loaders: vec![],
        }
    }

    pub fn add_plugin(&mut self, plugin: T) {
        let p = Box::new(Instance {
            id: uuid::Uuid::new_v4(),
            instance: plugin,
        });
        self.plugins.push(p);
    }
}

