#[macro_use]
extern crate plugins;
use plugins::{PluginManager, Result, ResultExt};

plugin_manager!{
    manager_name = MyManager;
    pub trait Plugin {
        fn hello(&self) -> Result<()>;
    }
}

#[derive(Default)]
struct TestPlugin;

impl Plugin for TestPlugin {
    fn hello(&self) -> Result<()> {
        println!("Hello, orld");
        Ok(())
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
        let plugin = manager.add_plugin(Box::new(TestPlugin {}));
        plugin.instance().hello().unwrap();
        plugin.id().clone()
    };
    manager.unload_plugin(&id);

    println!("Hello, world!");
}
