use error::Result;
use std::path::Path;
use uuid::Uuid;

pub trait Plugin<T> {
    fn id(&self) -> &Uuid;
    fn instance(&self) -> &T;
}

pub trait PluginLoader {
    type Item;
    fn load(&self, path: &Path) -> Result<Box<dyn Plugin<Self::Item>>>;
    fn can(&self, path: &Path) -> bool;
}

pub type PluginBoxed<T> = Box<dyn Plugin<T>>;

pub trait PluginManager {
    type PluginType;

    fn plugins(&self) -> &Vec<PluginBoxed<Self::PluginType>>;
    fn add_plugin(&mut self, plugin: Self::PluginType) -> &PluginBoxed<Self::PluginType>;
    fn add_loader(&mut self, loader: Box<dyn PluginLoader<Item = Self::PluginType>>);
    fn load_plugin(&mut self, path: &Path) -> Result<&PluginBoxed<Self::PluginType>>;
    fn unload_plugin(&mut self, id: &Uuid) -> bool;
    fn plugin(&self, id: &Uuid) -> Option<&PluginBoxed<Self::PluginType>> {
        self.plugins().into_iter().find(|m| m.id() == id)
    }
}
