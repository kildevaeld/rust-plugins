
Usage
```rust

extern crate plugins;

pub trait Plugin {
    fn register(&self, build: Builder) -> Result<Builder>;
    fn name(&self) -> &'static str;
}

build_plugin_manager!(Plugin, MyManager);

#[derive(Default)]
struct MyPlugin;

impl Plugin for MyPlugin {
    fn register(b:Builder) -> Result<Builder> {
        Ok(b)
    }
    fn name(&self) -> &'static str {
        "MyPlugin"
    }
}

declare_plugin!(Plugin, MyPlugin, MyPlugin::default);

fn main() {

    let mut m = MyManager::new();
    m.add_plugin(Box::new(MyPlugin::default()));
    m.load_plugin("path/to/plugin");

    let id = {
        let plugin = m.plugins().into_iter().find(|m| m.name == "MyPlugin");
        plugin.id
    };

    m.plugin(&id).instance().register(...)?;


}


```rust


struct Loader;

impl PluginLoader for Loader {
    type PluginType = Plugin;
    fn can(&self, path: &Path) -> bool {
        true
    }

    fn load(&self, path: &Path) -> Result<Box<dyn plugins::Plugin<Self::Item>>> {

    }
    
}


```