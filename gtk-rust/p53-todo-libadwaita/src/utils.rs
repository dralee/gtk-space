use std::{path::PathBuf, fs};
use gtk::glib;

use crate::APP_ID;

pub fn data_path() -> PathBuf {
    let mut path = glib::user_data_dir();
    path.push(APP_ID);
    fs::create_dir_all(&path).expect("could not create directory");
    path.push("data.json");
    path
}