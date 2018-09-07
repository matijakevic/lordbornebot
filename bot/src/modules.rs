use std::path::Path;

use libloading;
use lordbornebot_core::{Config, Module};
use std::boxed::Box;
use std::collections::HashMap;
use std::io::Result;

pub fn load_module(
    libraries: &mut HashMap<String, libloading::Library>,
    modules: &mut HashMap<String, Box<Module>>,
    config: &Config,
    name: &str,
) -> Result<()> {
    let library = libloading::Library::new(config.modules_path.join(name))?;
    libraries.insert(name.to_string(), library);

    unsafe {
        let create_module: libloading::Symbol<
            extern "C" fn(config: &Config) -> *mut Module,
        > = libraries[name].get(b"_create_module")?;
        let module_ptr = create_module(config);
        let module = Box::from_raw(module_ptr);
        modules.insert(name.to_string(), module);

        Ok(())
    }
}
