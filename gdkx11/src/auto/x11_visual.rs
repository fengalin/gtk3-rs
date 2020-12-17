// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;
use std::fmt;

glib::wrapper! {
    pub struct X11Visual(Object<ffi::GdkX11Visual, ffi::GdkX11VisualClass>) @extends gdk::Visual;

    match fn {
        get_type => || ffi::gdk_x11_visual_get_type(),
    }
}

impl X11Visual {}

impl fmt::Display for X11Visual {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("X11Visual")
    }
}
