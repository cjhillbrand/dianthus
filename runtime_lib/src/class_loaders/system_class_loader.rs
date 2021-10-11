use std::env;
use std::path::PathBuf;

use crate::class_loaders::class_loader::ClassLoader;
use crate::class_loaders::class_loader_container::ClassLoaderContainer;
use crate::class_loaders::extension_class_loader::ExtensionClassLoader;

pub struct SystemClassLoader {}

impl ClassLoader for SystemClassLoader {
	fn path_buf(&self) -> PathBuf { get_current_directory() }

	fn parent(&self) -> ClassLoaderContainer { ClassLoaderContainer::Extension(ExtensionClassLoader {}) }
}

fn get_current_directory() -> PathBuf {
	match env::current_dir() {
		Ok(value) => value,
		Err(err) => panic!("Could not resolve current directory. With err: {}", err)
	}
}
