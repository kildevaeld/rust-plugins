#[cfg(feature = "dll")]
#[macro_export]
macro_rules! native_loader {
    ($plugin_type:ident) => {
        struct NativePlugin {
            id: $crate::uuid::Uuid,
            plugin: Option<Box<dyn $plugin_type + Sync + Send>>,
            library: $crate::libloading::Library,
        }
        unsafe impl Sync for NativePlugin {}
        impl Drop for NativePlugin {
            fn drop(&mut self) {
                self.plugin = None;
                drop(&self.library);
            }
        }

        impl $crate::Plugin<Box<dyn $plugin_type + Sync + Send>> for NativePlugin {
            fn id(&self) -> &$crate::uuid::Uuid {
                &self.id
            }
            fn instance(&self) -> Option<&Box<dyn $plugin_type + Sync + Send>> {
                match self.plugin {
                    Some(ref e) => Some(&e),
                    None => None,
                }
            }
        }

        #[cfg(target_os = "windows")]
        pub static LIB_EXT: &'static str = "dll";
        #[cfg(target_os = "macos")]
        pub static LIB_EXT: &'static str = "dylib";
        #[cfg(target_os = "linux")]
        pub static LIB_EXT: &'static str = "so";

        struct NativeLoader;

        impl NativeLoader {
            pub fn new() -> NativeLoader {
                NativeLoader {}
            }

            unsafe fn load_plugin<P: AsRef<std::ffi::OsStr>>(
                &self,
                filename: P,
            ) -> $crate::Result<Box<dyn $crate::Plugin<Box<dyn $plugin_type + Sync + Send>>>> {
                type PluginCreate = unsafe fn() -> *mut $plugin_type;

                let lib = if cfg!(unix) {
                    let os_lib = $crate::libloading::os::unix::Library::open(
                        Some(filename.as_ref()),
                        $crate::libc::RTLD_NODELETE | $crate::libc::RTLD_NOW,
                    )?;
                    $crate::libloading::Library::from(os_lib)
                } else {
                    $crate::libloading::Library::new(filename.as_ref())?
                };

                //let lib = $crate::libloading::Library::new(filename.as_ref())?;

                let id = $crate::uuid::Uuid::new_v4();

                let plugin: Box<dyn $plugin_type + Sync + Send>;

                {
                    let constructor: $crate::libloading::Symbol<PluginCreate> =
                        lib.get(b"_plugin_create")?;

                    let boxed_raw = constructor();

                    plugin = Box::from_raw(boxed_raw);
                }

                Ok(Box::new(NativePlugin {
                    id: id,
                    library: lib,
                    plugin: Some(plugin),
                }))
            }
        }

        impl $crate::PluginLoader for NativeLoader {
            type Item = Box<dyn $plugin_type + Sync + Send>;

            fn load(
                &self,
                path: &std::path::Path,
            ) -> $crate::Result<Box<dyn $crate::Plugin<Self::Item>>> {
                let plugin = unsafe { self.load_plugin(path)? };
                Ok(plugin)
            }

            fn can(&self, path: &std::path::Path) -> bool {
                if let Some(ext) = path.extension() {
                    return ext.to_str().unwrap_or("") == LIB_EXT;
                }

                false
            }
        }
    };
}

#[cfg(not(feature = "dll"))]
#[macro_export]
macro_rules! native_loader {
    ($plugin_type:ident) => {};
}

#[cfg(not(feature = "dll"))]
#[macro_export]
macro_rules! push_native {
    ($manager: ident) => {};
}

#[cfg(feature = "dll")]
#[macro_export]
macro_rules! push_native {
    ($manager: ident) => {
        $manager.loaders.push(Box::new(NativeLoader::new()));
    };
}

#[macro_export]
macro_rules! build_plugin_manager {
    ($plugin_type:ident) => {
        build_plugin_manager!($plugin_type, PluginManager);
    };
    ($plugin_type:ident, $manager_name:ident) => {
        struct Instance {
            id: $crate::uuid::Uuid,
            instance: Box<dyn $plugin_type + Sync + Send>,
        }

        impl $crate::Plugin<Box<dyn $plugin_type + Sync + Send>> for Instance {
            fn id(&self) -> &$crate::uuid::Uuid {
                &self.id
            }

            fn instance(&self) -> Option<&Box<dyn $plugin_type + Sync + Send>> {
                Some(&self.instance)
            }
        }

        pub struct $manager_name {
            plugins:
                Vec<Box<dyn $crate::Plugin<Box<dyn $plugin_type + Send + Sync>> + Sync + Send>>,
            loaders: Vec<Box<dyn $crate::PluginLoader<Item = Box<dyn $plugin_type + Sync + Send>>>>,
        }

        native_loader!($plugin_type);

        impl $manager_name {
            pub fn new() -> $manager_name {
                let mut out = $manager_name {
                    plugins: vec![],
                    loaders: vec![],
                };

                // #[cfg(feature = "dll")]
                // out.loaders.push(Box::new(NativeLoader::new()));
                push_native!(out);

                out
            }
        }

        impl $crate::PluginManager for $manager_name {
            type PluginType = Box<dyn $plugin_type + Sync + Send>;

            fn plugins(&self) -> &Vec<Box<dyn $crate::Plugin<Self::PluginType> + Sync + Send>> {
                &self.plugins
            }

            fn add_plugin(
                &mut self,
                plugin: Self::PluginType,
            ) -> &Box<dyn $crate::Plugin<Self::PluginType> + Sync + Send> {
                self.plugins.push(Box::new(Instance {
                    id: $crate::uuid::Uuid::new_v4(),
                    instance: plugin,
                }));
                self.plugins.last().unwrap()
            }

            fn add_loader(
                &mut self,
                loader: Box<dyn $crate::PluginLoader<Item = Self::PluginType>>,
            ) {
                self.loaders.push(loader);
            }

            fn load_plugin(
                &mut self,
                path: &std::path::Path,
            ) -> $crate::Result<&Box<dyn $crate::Plugin<Self::PluginType> + Sync + Send>> {
                let loader = self.loaders.iter().find(|m| m.can(&path));
                if loader.is_none() {
                    return Err($crate::ErrorKind::Loader(path.to_path_buf()).into());
                }

                let plugin = loader.unwrap().load(path)?;

                self.plugins.push(plugin);

                Ok(self.plugins.last().unwrap())
            }

            fn unload_plugin(&mut self, id: &$crate::uuid::Uuid) -> bool {
                if let Some(found) = self.plugins.iter().position(|m| m.id() == id) {
                    self.plugins.remove(found);
                    return false;
                }

                true
            }
        }
    };
}

/// Declare a plugin type and its constructor.
///
/// # Notes
///
/// This works by automatically generating an `extern "C"` function with a
/// pre-defined signature and symbol name. Therefore you will only be able to
/// declare one plugin per library.
#[macro_export]
macro_rules! declare_plugin {
    ($plugin_trait:ident, $plugin_type:ty, $constructor:path) => {
        declare_plugin!($plugin_trait, $plugin_type, $constructor, _plugin_create);
    };

    ($plugin_trait:ident, $plugin_type:ty, $constructor:path, $fn: ident) => {
        #[no_mangle]
        pub extern "C" fn $fn() -> *mut $plugin_trait {
            // make sure the constructor is the correct type.
            let constructor: fn() -> $plugin_type = $constructor;

            let object = constructor();
            let boxed: Box<$plugin_trait + Sync + Send> = Box::new(object);
            Box::into_raw(boxed)
        }
    };
}
