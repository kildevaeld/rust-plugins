#[macro_use]
extern crate plugins;
use plugins::PluginManager;

pub trait Plugin {
    fn hello(&self);
}

build_plugin_manager!(Plugin, MyManager);

#[derive(Default)]
struct TestPlugin;

impl Plugin for TestPlugin {
    fn hello(&self) {
        println!("Hello, orld");
    }
}

impl Drop for TestPlugin {
    fn drop(&mut self) {
        println!("drop");
    }
}

declare_plugin!(Plugin, TestPlugin, TestPlugin::default);

fn main() {
    let mut manager = MyManager::new();
    let id = {
        let plugin = manager.add_plugin(Box::new(TestPlugin::default()));
        plugin.instance().hello();
        plugin.id().clone()
    };
    manager.unload_plugin(&id);

    println!("Hello, world!");
}
