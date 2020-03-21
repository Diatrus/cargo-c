use std::path::PathBuf;

use crate::target::Target;

#[derive(Debug)]
pub struct BuildTargets {
    pub include: PathBuf,
    pub static_lib: PathBuf,
    pub shared_lib: PathBuf,
    pub impl_lib: Option<PathBuf>,
    pub def: Option<PathBuf>,
    pub pc: PathBuf,
}

impl BuildTargets {
    pub fn new(name: &str, target: &Target, targetdir: &PathBuf) -> BuildTargets {
        let pc = targetdir.join(&format!("{}.pc", name));
        let include = targetdir.join(&format!("{}.h", name));

        let os = &target.os;
        let env = &target.env;

        let (shared_lib, static_lib, impl_lib, def) = match (os.as_str(), env.as_str()) {
            ("linux", _) | ("freebsd", _) | ("dragonfly", _) | ("netbsd", _) => {
                let static_lib = targetdir.join(&format!("lib{}.a", name));
                let shared_lib = targetdir.join(&format!("lib{}.so", name));
                (shared_lib, static_lib, None, None)
            }
            ("macos", _) => {
                let static_lib = targetdir.join(&format!("lib{}.a", name));
                let shared_lib = targetdir.join(&format!("lib{}.dylib", name));
                (shared_lib, static_lib, None, None)
            }
            ("windows", "gnu") => {
                let static_lib = targetdir.join(&format!("{}.lib", name));
                let shared_lib = targetdir.join(&format!("{}.dll", name));
                let impl_lib = targetdir.join(&format!("{}.dll.a", name));
                let def = targetdir.join(&format!("{}.def", name));
                (shared_lib, static_lib, Some(impl_lib), Some(def))
            }
            _ => unimplemented!("The target {}-{} is not supported yet", os, env),
        };

        BuildTargets {
            pc,
            include,
            static_lib,
            shared_lib,
            impl_lib,
            def,
        }
    }
}
