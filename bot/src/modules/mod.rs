pub mod afk;
pub mod gamble;
pub mod points;
pub mod rpg;
pub mod shapes;

use std::path::Path;

use libloading;
use lordbornebot_core::Module;
use std::boxed::Box;

pub fn load_module(libraries: &mut Vec<libloading::Library>, name: &str) -> Box<Module> {
    libraries.push(libloading::Library::new(name).unwrap());
    unsafe {
        let lib = &libraries.last().unwrap();
        let func: libloading::Symbol<unsafe fn() -> *mut Module> =
            lib.get(b"_create_module").unwrap();

        Box::from_raw(func())
    }
}
