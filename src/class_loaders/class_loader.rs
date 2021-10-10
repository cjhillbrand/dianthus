use crate::class_loaders::class_loader_container::ClassLoaderContainer;
use crate::entities::class_struct::ClassStruct;
use std::collections::VecDeque;
use std::iter::FromIterator;
use std::fs;
use std::env;
use std::string::String;
use std::path::PathBuf;

pub trait ClassLoader {
    fn path_buf(&self) -> PathBuf;
    fn parent(&self) -> ClassLoaderContainer;
    fn load_class(&self, file: &str) -> ClassStruct
    {
        let mut path: PathBuf = self.path_buf();
        path.push(file);
        path.set_extension(".class");

        match fs::read(&path) {
            Ok(data_vec) =>
                {
                    let mut data: VecDeque<u8> = VecDeque::from_iter(data_vec);
                    ClassStruct::new(&mut data)
                },
            Err(_err) => self.parent().load_class(file),
        }
    }
}

const JAVA_HOME: &str = "JAVA_HOME";
const CLASS_PATH: &str = "CLASSPATH";

pub fn get_java_home() -> String {
    match env::var(JAVA_HOME) {
        Ok(result) => result,
        Err(err) => panic!(
            "Can not find \"{}\" environment variable. With err: {}",
            JAVA_HOME, err
        ),
    }
}

pub fn get_class_path() -> String {
    match env::var(CLASS_PATH) {
        Ok(result) => result,
        Err(err) => panic!(
            "Can not find \"{}\" environment variable. With err: {}",
            CLASS_PATH, err
        ),
    }
}
