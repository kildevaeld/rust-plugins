
Usage
```rust

extern crate plugins;

plugin_manager!{
    manager_name = MyManager; // Defaults to PluginManager
    pub trait Plugin {
        fn register(&self, build: Builder) -> Result<Builder>;
        fn name(&self) -> &'static str;
    }
}

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

    m.plugin(id).instance().register(...)?;


}


```