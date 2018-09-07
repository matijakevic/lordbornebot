use std::path::Path;

use libloading;
use lordbornebot_core::Module;
use std::boxed::Box;
use std::collections::HashMap;
use std::io::Result;

pub fn load_module(
    libraries: &mut HashMap<String, libloading::Library>,
    modules: &mut HashMap<String, Box<Module>>,
    path: &Path,
    name: &str,
) -> Result<()> {
    let library = libloading::Library::new(path.join(name))?;
    libraries.insert(name.to_string(), library);

    unsafe {
        let create_module: libloading::Symbol<extern "C" fn() -> *mut Module> =
            libraries[name].get(b"_create_module")?;
        let module_ptr = create_module();
        let module = Box::from_raw(module_ptr);
        modules.insert(name.to_string(), module);

        Ok(())
    }
}
