#[macro_use]
extern crate plugins;
#[macro_use]
extern crate error_chain;
use plugins::PluginManager;

// mod error {
//     use plugins;
//     error_chain!{
//         links {
//             Plugins(plugins::Error, plugins::ErrorKind);
//         }
//     }
// }

//use error::{Result, ResultExt};

// plugin_manager!{
//     manager_name = MyManager;
//     pub trait Plugin {
//         fn hello(&self) -> Result<()>;
//     }
// }

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
