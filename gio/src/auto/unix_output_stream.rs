// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::OutputStream;
use crate::PollableOutputStream;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    pub struct UnixOutputStream(Object<ffi::GUnixOutputStream, ffi::GUnixOutputStreamClass>) @extends OutputStream, @implements PollableOutputStream;

    match fn {
        get_type => || ffi::g_unix_output_stream_get_type(),
    }
}

pub const NONE_UNIX_OUTPUT_STREAM: Option<&UnixOutputStream> = None;

pub trait UnixOutputStreamExt: 'static {
    #[doc(alias = "g_unix_output_stream_get_close_fd")]
    fn get_close_fd(&self) -> bool;
}

impl<O: IsA<UnixOutputStream>> UnixOutputStreamExt for O {
    fn get_close_fd(&self) -> bool {
        unsafe {
            from_glib(ffi::g_unix_output_stream_get_close_fd(
                self.as_ref().to_glib_none().0,
            ))
        }
    }
}

impl fmt::Display for UnixOutputStream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("UnixOutputStream")
    }
}
