// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Buildable;
use Container;
use Orientable;
use Orientation;
use ScrollType;
use Widget;
use ffi;
use gdk;
use glib;
use glib::StaticType;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Paned(Object<ffi::GtkPaned, ffi::GtkPanedClass>): Container, Widget, Buildable, Orientable;

    match fn {
        get_type => || ffi::gtk_paned_get_type(),
    }
}

impl Paned {
    pub fn new(orientation: Orientation) -> Paned {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_paned_new(orientation.to_glib())).downcast_unchecked()
        }
    }
}

pub trait PanedExt {
    fn add1<P: IsA<Widget>>(&self, child: &P);

    fn add2<P: IsA<Widget>>(&self, child: &P);

    fn get_child1(&self) -> Option<Widget>;

    fn get_child2(&self) -> Option<Widget>;

    fn get_handle_window(&self) -> Option<gdk::Window>;

    fn get_position(&self) -> i32;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn get_wide_handle(&self) -> bool;

    fn pack1<P: IsA<Widget>>(&self, child: &P, resize: bool, shrink: bool);

    fn pack2<P: IsA<Widget>>(&self, child: &P, resize: bool, shrink: bool);

    fn set_position(&self, position: i32);

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn set_wide_handle(&self, wide: bool);

    fn get_property_max_position(&self) -> i32;

    fn get_property_min_position(&self) -> i32;

    fn get_property_position_set(&self) -> bool;

    fn set_property_position_set(&self, position_set: bool);

    fn get_child_resize<T: IsA<Widget>>(&self, item: &T) -> bool;

    fn set_child_resize<T: IsA<Widget>>(&self, item: &T, resize: bool);

    fn get_child_shrink<T: IsA<Widget>>(&self, item: &T) -> bool;

    fn set_child_shrink<T: IsA<Widget>>(&self, item: &T, shrink: bool);

    fn connect_accept_position<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_accept_position(&self) -> bool;

    fn connect_cancel_position<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_cancel_position(&self) -> bool;

    fn connect_cycle_child_focus<F: Fn(&Self, bool) -> bool + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_cycle_child_focus(&self, reversed: bool) -> bool;

    fn connect_cycle_handle_focus<F: Fn(&Self, bool) -> bool + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_cycle_handle_focus(&self, reversed: bool) -> bool;

    fn connect_move_handle<F: Fn(&Self, ScrollType) -> bool + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_move_handle(&self, scroll_type: ScrollType) -> bool;

    fn connect_toggle_handle_focus<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_toggle_handle_focus(&self) -> bool;

    fn connect_property_max_position_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_min_position_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_position_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_position_set_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_property_wide_handle_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Paned> + IsA<Container> + IsA<glib::object::Object> + glib::object::ObjectExt> PanedExt for O {
    fn add1<P: IsA<Widget>>(&self, child: &P) {
        unsafe {
            ffi::gtk_paned_add1(self.to_glib_none().0, child.to_glib_none().0);
        }
    }

    fn add2<P: IsA<Widget>>(&self, child: &P) {
        unsafe {
            ffi::gtk_paned_add2(self.to_glib_none().0, child.to_glib_none().0);
        }
    }

    fn get_child1(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_paned_get_child1(self.to_glib_none().0))
        }
    }

    fn get_child2(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_paned_get_child2(self.to_glib_none().0))
        }
    }

    fn get_handle_window(&self) -> Option<gdk::Window> {
        unsafe {
            from_glib_none(ffi::gtk_paned_get_handle_window(self.to_glib_none().0))
        }
    }

    fn get_position(&self) -> i32 {
        unsafe {
            ffi::gtk_paned_get_position(self.to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn get_wide_handle(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_paned_get_wide_handle(self.to_glib_none().0))
        }
    }

    fn pack1<P: IsA<Widget>>(&self, child: &P, resize: bool, shrink: bool) {
        unsafe {
            ffi::gtk_paned_pack1(self.to_glib_none().0, child.to_glib_none().0, resize.to_glib(), shrink.to_glib());
        }
    }

    fn pack2<P: IsA<Widget>>(&self, child: &P, resize: bool, shrink: bool) {
        unsafe {
            ffi::gtk_paned_pack2(self.to_glib_none().0, child.to_glib_none().0, resize.to_glib(), shrink.to_glib());
        }
    }

    fn set_position(&self, position: i32) {
        unsafe {
            ffi::gtk_paned_set_position(self.to_glib_none().0, position);
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn set_wide_handle(&self, wide: bool) {
        unsafe {
            ffi::gtk_paned_set_wide_handle(self.to_glib_none().0, wide.to_glib());
        }
    }

    fn get_property_max_position(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "max-position".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn get_property_min_position(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "min-position".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn get_property_position_set(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "position-set".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_position_set(&self, position_set: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "position-set".to_glib_none().0, Value::from(&position_set).to_glib_none().0);
        }
    }

    fn get_child_resize<T: IsA<Widget>>(&self, item: &T) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            ffi::gtk_container_child_get_property(self.to_glib_none().0, item.to_glib_none().0, "resize".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_child_resize<T: IsA<Widget>>(&self, item: &T, resize: bool) {
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0, item.to_glib_none().0, "resize".to_glib_none().0, Value::from(&resize).to_glib_none().0);
        }
    }

    fn get_child_shrink<T: IsA<Widget>>(&self, item: &T) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            ffi::gtk_container_child_get_property(self.to_glib_none().0, item.to_glib_none().0, "shrink".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_child_shrink<T: IsA<Widget>>(&self, item: &T, shrink: bool) {
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0, item.to_glib_none().0, "shrink".to_glib_none().0, Value::from(&shrink).to_glib_none().0);
        }
    }

    fn connect_accept_position<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "accept-position",
                transmute(accept_position_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn emit_accept_position(&self) -> bool {
        let res = self.emit("accept-position", &[]).unwrap();
        res.unwrap().get().unwrap()
    }

    fn connect_cancel_position<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "cancel-position",
                transmute(cancel_position_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn emit_cancel_position(&self) -> bool {
        let res = self.emit("cancel-position", &[]).unwrap();
        res.unwrap().get().unwrap()
    }

    fn connect_cycle_child_focus<F: Fn(&Self, bool) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, bool) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "cycle-child-focus",
                transmute(cycle_child_focus_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn emit_cycle_child_focus(&self, reversed: bool) -> bool {
        let res = self.emit("cycle-child-focus", &[&reversed]).unwrap();
        res.unwrap().get().unwrap()
    }

    fn connect_cycle_handle_focus<F: Fn(&Self, bool) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, bool) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "cycle-handle-focus",
                transmute(cycle_handle_focus_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn emit_cycle_handle_focus(&self, reversed: bool) -> bool {
        let res = self.emit("cycle-handle-focus", &[&reversed]).unwrap();
        res.unwrap().get().unwrap()
    }

    fn connect_move_handle<F: Fn(&Self, ScrollType) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, ScrollType) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "move-handle",
                transmute(move_handle_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn emit_move_handle(&self, scroll_type: ScrollType) -> bool {
        let res = self.emit("move-handle", &[&scroll_type]).unwrap();
        res.unwrap().get().unwrap()
    }

    fn connect_toggle_handle_focus<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "toggle-handle-focus",
                transmute(toggle_handle_focus_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn emit_toggle_handle_focus(&self) -> bool {
        let res = self.emit("toggle-handle-focus", &[]).unwrap();
        res.unwrap().get().unwrap()
    }

    fn connect_property_max_position_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::max-position",
                transmute(notify_max_position_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_min_position_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::min-position",
                transmute(notify_min_position_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_position_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::position",
                transmute(notify_position_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_position_set_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::position-set",
                transmute(notify_position_set_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_property_wide_handle_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::wide-handle",
                transmute(notify_wide_handle_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn accept_position_trampoline<P>(this: *mut ffi::GtkPaned, f: glib_ffi::gpointer) -> glib_ffi::gboolean
where P: IsA<Paned> {
    let f: &&(Fn(&P) -> bool + 'static) = transmute(f);
    f(&Paned::from_glib_borrow(this).downcast_unchecked()).to_glib()
}

unsafe extern "C" fn cancel_position_trampoline<P>(this: *mut ffi::GtkPaned, f: glib_ffi::gpointer) -> glib_ffi::gboolean
where P: IsA<Paned> {
    let f: &&(Fn(&P) -> bool + 'static) = transmute(f);
    f(&Paned::from_glib_borrow(this).downcast_unchecked()).to_glib()
}

unsafe extern "C" fn cycle_child_focus_trampoline<P>(this: *mut ffi::GtkPaned, reversed: glib_ffi::gboolean, f: glib_ffi::gpointer) -> glib_ffi::gboolean
where P: IsA<Paned> {
    let f: &&(Fn(&P, bool) -> bool + 'static) = transmute(f);
    f(&Paned::from_glib_borrow(this).downcast_unchecked(), from_glib(reversed)).to_glib()
}

unsafe extern "C" fn cycle_handle_focus_trampoline<P>(this: *mut ffi::GtkPaned, reversed: glib_ffi::gboolean, f: glib_ffi::gpointer) -> glib_ffi::gboolean
where P: IsA<Paned> {
    let f: &&(Fn(&P, bool) -> bool + 'static) = transmute(f);
    f(&Paned::from_glib_borrow(this).downcast_unchecked(), from_glib(reversed)).to_glib()
}

unsafe extern "C" fn move_handle_trampoline<P>(this: *mut ffi::GtkPaned, scroll_type: ffi::GtkScrollType, f: glib_ffi::gpointer) -> glib_ffi::gboolean
where P: IsA<Paned> {
    let f: &&(Fn(&P, ScrollType) -> bool + 'static) = transmute(f);
    f(&Paned::from_glib_borrow(this).downcast_unchecked(), from_glib(scroll_type)).to_glib()
}

unsafe extern "C" fn toggle_handle_focus_trampoline<P>(this: *mut ffi::GtkPaned, f: glib_ffi::gpointer) -> glib_ffi::gboolean
where P: IsA<Paned> {
    let f: &&(Fn(&P) -> bool + 'static) = transmute(f);
    f(&Paned::from_glib_borrow(this).downcast_unchecked()).to_glib()
}

unsafe extern "C" fn notify_max_position_trampoline<P>(this: *mut ffi::GtkPaned, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Paned> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Paned::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_min_position_trampoline<P>(this: *mut ffi::GtkPaned, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Paned> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Paned::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_position_trampoline<P>(this: *mut ffi::GtkPaned, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Paned> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Paned::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_position_set_trampoline<P>(this: *mut ffi::GtkPaned, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Paned> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Paned::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_16", feature = "dox"))]
unsafe extern "C" fn notify_wide_handle_trampoline<P>(this: *mut ffi::GtkPaned, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Paned> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Paned::from_glib_borrow(this).downcast_unchecked())
}

impl fmt::Display for Paned {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Paned")
    }
}
