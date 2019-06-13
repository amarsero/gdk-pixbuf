// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gdk_pixbuf_sys;
use gio;
use glib;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::translate::*;
use glib::GString;
use glib::StaticType;
use glib::Value;
use gobject_sys;
use std::fmt;
use std::ptr;
use Colorspace;
use Error;
use InterpType;
use PixbufFormat;
use PixbufRotation;

glib_wrapper! {
    pub struct Pixbuf(Object<gdk_pixbuf_sys::GdkPixbuf, PixbufClass>) @implements gio::Icon, gio::LoadableIcon;

    match fn {
        get_type => || gdk_pixbuf_sys::gdk_pixbuf_get_type(),
    }
}

impl Pixbuf {
    pub fn new(
        colorspace: Colorspace,
        has_alpha: bool,
        bits_per_sample: i32,
        width: i32,
        height: i32,
    ) -> Option<Pixbuf> {
        unsafe {
            from_glib_full(gdk_pixbuf_sys::gdk_pixbuf_new(
                colorspace.to_glib(),
                has_alpha.to_glib(),
                bits_per_sample,
                width,
                height,
            ))
        }
    }

    #[cfg(any(feature = "v2_32", feature = "dox"))]
    pub fn new_from_bytes(
        data: &glib::Bytes,
        colorspace: Colorspace,
        has_alpha: bool,
        bits_per_sample: i32,
        width: i32,
        height: i32,
        rowstride: i32,
    ) -> Pixbuf {
        unsafe {
            from_glib_full(gdk_pixbuf_sys::gdk_pixbuf_new_from_bytes(
                data.to_glib_none().0,
                colorspace.to_glib(),
                has_alpha.to_glib(),
                bits_per_sample,
                width,
                height,
                rowstride,
            ))
        }
    }

    //pub fn new_from_data(data: &[u8], colorspace: Colorspace, has_alpha: bool, bits_per_sample: i32, width: i32, height: i32, rowstride: i32, destroy_fn: Option<Box<dyn FnOnce(&Vec<u8>) + 'static>>) -> Pixbuf {
    //    unsafe { TODO: call gdk_pixbuf_sys:gdk_pixbuf_new_from_data() }
    //}

    #[cfg_attr(feature = "v2_32", deprecated)]
    pub fn new_from_inline(data: &[u8], copy_pixels: bool) -> Result<Pixbuf, Error> {
        let data_length = data.len() as i32;
        unsafe {
            let mut error = ptr::null_mut();
            let ret = gdk_pixbuf_sys::gdk_pixbuf_new_from_inline(
                data_length,
                data.to_glib_none().0,
                copy_pixels.to_glib(),
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn new_from_resource(resource_path: &str) -> Result<Pixbuf, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = gdk_pixbuf_sys::gdk_pixbuf_new_from_resource(
                resource_path.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn new_from_resource_at_scale(
        resource_path: &str,
        width: i32,
        height: i32,
        preserve_aspect_ratio: bool,
    ) -> Result<Pixbuf, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = gdk_pixbuf_sys::gdk_pixbuf_new_from_resource_at_scale(
                resource_path.to_glib_none().0,
                width,
                height,
                preserve_aspect_ratio.to_glib(),
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn new_from_stream<P: IsA<gio::InputStream>, Q: IsA<gio::Cancellable>>(
        stream: &P,
        cancellable: Option<&Q>,
    ) -> Result<Pixbuf, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = gdk_pixbuf_sys::gdk_pixbuf_new_from_stream(
                stream.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn new_from_stream_at_scale<P: IsA<gio::InputStream>, Q: IsA<gio::Cancellable>>(
        stream: &P,
        width: i32,
        height: i32,
        preserve_aspect_ratio: bool,
        cancellable: Option<&Q>,
    ) -> Result<Pixbuf, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = gdk_pixbuf_sys::gdk_pixbuf_new_from_stream_at_scale(
                stream.as_ref().to_glib_none().0,
                width,
                height,
                preserve_aspect_ratio.to_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn new_from_xpm_data(data: &[&str]) -> Pixbuf {
        unsafe {
            from_glib_full(gdk_pixbuf_sys::gdk_pixbuf_new_from_xpm_data(
                data.to_glib_none().0,
            ))
        }
    }

    pub fn add_alpha(&self, substitute_color: bool, r: u8, g: u8, b: u8) -> Option<Pixbuf> {
        unsafe {
            from_glib_full(gdk_pixbuf_sys::gdk_pixbuf_add_alpha(
                self.to_glib_none().0,
                substitute_color.to_glib(),
                r,
                g,
                b,
            ))
        }
    }

    pub fn apply_embedded_orientation(&self) -> Option<Pixbuf> {
        unsafe {
            from_glib_full(gdk_pixbuf_sys::gdk_pixbuf_apply_embedded_orientation(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn composite(
        &self,
        dest: &Pixbuf,
        dest_x: i32,
        dest_y: i32,
        dest_width: i32,
        dest_height: i32,
        offset_x: f64,
        offset_y: f64,
        scale_x: f64,
        scale_y: f64,
        interp_type: InterpType,
        overall_alpha: i32,
    ) {
        unsafe {
            gdk_pixbuf_sys::gdk_pixbuf_composite(
                self.to_glib_none().0,
                dest.to_glib_none().0,
                dest_x,
                dest_y,
                dest_width,
                dest_height,
                offset_x,
                offset_y,
                scale_x,
                scale_y,
                interp_type.to_glib(),
                overall_alpha,
            );
        }
    }

    pub fn composite_color(
        &self,
        dest: &Pixbuf,
        dest_x: i32,
        dest_y: i32,
        dest_width: i32,
        dest_height: i32,
        offset_x: f64,
        offset_y: f64,
        scale_x: f64,
        scale_y: f64,
        interp_type: InterpType,
        overall_alpha: i32,
        check_x: i32,
        check_y: i32,
        check_size: i32,
        color1: u32,
        color2: u32,
    ) {
        unsafe {
            gdk_pixbuf_sys::gdk_pixbuf_composite_color(
                self.to_glib_none().0,
                dest.to_glib_none().0,
                dest_x,
                dest_y,
                dest_width,
                dest_height,
                offset_x,
                offset_y,
                scale_x,
                scale_y,
                interp_type.to_glib(),
                overall_alpha,
                check_x,
                check_y,
                check_size,
                color1,
                color2,
            );
        }
    }

    pub fn composite_color_simple(
        &self,
        dest_width: i32,
        dest_height: i32,
        interp_type: InterpType,
        overall_alpha: i32,
        check_size: i32,
        color1: u32,
        color2: u32,
    ) -> Option<Pixbuf> {
        unsafe {
            from_glib_full(gdk_pixbuf_sys::gdk_pixbuf_composite_color_simple(
                self.to_glib_none().0,
                dest_width,
                dest_height,
                interp_type.to_glib(),
                overall_alpha,
                check_size,
                color1,
                color2,
            ))
        }
    }

    pub fn copy(&self) -> Option<Pixbuf> {
        unsafe { from_glib_full(gdk_pixbuf_sys::gdk_pixbuf_copy(self.to_glib_none().0)) }
    }

    pub fn copy_area(
        &self,
        src_x: i32,
        src_y: i32,
        width: i32,
        height: i32,
        dest_pixbuf: &Pixbuf,
        dest_x: i32,
        dest_y: i32,
    ) {
        unsafe {
            gdk_pixbuf_sys::gdk_pixbuf_copy_area(
                self.to_glib_none().0,
                src_x,
                src_y,
                width,
                height,
                dest_pixbuf.to_glib_none().0,
                dest_x,
                dest_y,
            );
        }
    }

    #[cfg(any(feature = "v2_36", feature = "dox"))]
    pub fn copy_options(&self, dest_pixbuf: &Pixbuf) -> bool {
        unsafe {
            from_glib(gdk_pixbuf_sys::gdk_pixbuf_copy_options(
                self.to_glib_none().0,
                dest_pixbuf.to_glib_none().0,
            ))
        }
    }

    pub fn fill(&self, pixel: u32) {
        unsafe {
            gdk_pixbuf_sys::gdk_pixbuf_fill(self.to_glib_none().0, pixel);
        }
    }

    pub fn flip(&self, horizontal: bool) -> Option<Pixbuf> {
        unsafe {
            from_glib_full(gdk_pixbuf_sys::gdk_pixbuf_flip(
                self.to_glib_none().0,
                horizontal.to_glib(),
            ))
        }
    }

    pub fn get_bits_per_sample(&self) -> i32 {
        unsafe { gdk_pixbuf_sys::gdk_pixbuf_get_bits_per_sample(self.to_glib_none().0) }
    }

    pub fn get_byte_length(&self) -> usize {
        unsafe { gdk_pixbuf_sys::gdk_pixbuf_get_byte_length(self.to_glib_none().0) }
    }

    pub fn get_colorspace(&self) -> Colorspace {
        unsafe {
            from_glib(gdk_pixbuf_sys::gdk_pixbuf_get_colorspace(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_has_alpha(&self) -> bool {
        unsafe {
            from_glib(gdk_pixbuf_sys::gdk_pixbuf_get_has_alpha(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_height(&self) -> i32 {
        unsafe { gdk_pixbuf_sys::gdk_pixbuf_get_height(self.to_glib_none().0) }
    }

    pub fn get_n_channels(&self) -> i32 {
        unsafe { gdk_pixbuf_sys::gdk_pixbuf_get_n_channels(self.to_glib_none().0) }
    }

    pub fn get_option(&self, key: &str) -> Option<GString> {
        unsafe {
            from_glib_none(gdk_pixbuf_sys::gdk_pixbuf_get_option(
                self.to_glib_none().0,
                key.to_glib_none().0,
            ))
        }
    }

    //#[cfg(any(feature = "v2_32", feature = "dox"))]
    //pub fn get_options(&self) -> /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 } {
    //    unsafe { TODO: call gdk_pixbuf_sys:gdk_pixbuf_get_options() }
    //}

    pub fn get_rowstride(&self) -> i32 {
        unsafe { gdk_pixbuf_sys::gdk_pixbuf_get_rowstride(self.to_glib_none().0) }
    }

    pub fn get_width(&self) -> i32 {
        unsafe { gdk_pixbuf_sys::gdk_pixbuf_get_width(self.to_glib_none().0) }
    }

    pub fn new_subpixbuf(&self, src_x: i32, src_y: i32, width: i32, height: i32) -> Option<Pixbuf> {
        unsafe {
            from_glib_full(gdk_pixbuf_sys::gdk_pixbuf_new_subpixbuf(
                self.to_glib_none().0,
                src_x,
                src_y,
                width,
                height,
            ))
        }
    }

    #[cfg(any(feature = "v2_32", feature = "dox"))]
    pub fn read_pixel_bytes(&self) -> Option<glib::Bytes> {
        unsafe {
            from_glib_full(gdk_pixbuf_sys::gdk_pixbuf_read_pixel_bytes(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v2_36", feature = "dox"))]
    pub fn remove_option(&self, key: &str) -> bool {
        unsafe {
            from_glib(gdk_pixbuf_sys::gdk_pixbuf_remove_option(
                self.to_glib_none().0,
                key.to_glib_none().0,
            ))
        }
    }

    pub fn rotate_simple(&self, angle: PixbufRotation) -> Option<Pixbuf> {
        unsafe {
            from_glib_full(gdk_pixbuf_sys::gdk_pixbuf_rotate_simple(
                self.to_glib_none().0,
                angle.to_glib(),
            ))
        }
    }

    pub fn saturate_and_pixelate(&self, dest: &Pixbuf, saturation: f32, pixelate: bool) {
        unsafe {
            gdk_pixbuf_sys::gdk_pixbuf_saturate_and_pixelate(
                self.to_glib_none().0,
                dest.to_glib_none().0,
                saturation,
                pixelate.to_glib(),
            );
        }
    }

    //pub fn save<P: AsRef<std::path::Path>>(&self, filename: P, type_: &str, error: Option<&mut Error>, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> bool {
    //    unsafe { TODO: call gdk_pixbuf_sys:gdk_pixbuf_save() }
    //}

    //pub fn save_to_buffer(&self, type_: &str, error: Option<&mut Error>, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Option<Vec<u8>> {
    //    unsafe { TODO: call gdk_pixbuf_sys:gdk_pixbuf_save_to_buffer() }
    //}

    //pub fn save_to_callback<P: FnMut(&Vec<u8>, usize, &Error) -> bool>(&self, save_func: P, type_: &str, error: Option<&mut Error>, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> bool {
    //    unsafe { TODO: call gdk_pixbuf_sys:gdk_pixbuf_save_to_callback() }
    //}

    //pub fn save_to_callbackv<P: FnMut(&Vec<u8>, usize, &Error) -> bool>(&self, save_func: P, type_: &str, option_keys: &[&str], option_values: &[&str]) -> Result<(), Error> {
    //    unsafe { TODO: call gdk_pixbuf_sys:gdk_pixbuf_save_to_callbackv() }
    //}

    //pub fn save_to_stream<P: IsA<gio::OutputStream>, Q: IsA<gio::Cancellable>>(&self, stream: &P, type_: &str, cancellable: Option<&Q>, error: Option<&mut Error>, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> bool {
    //    unsafe { TODO: call gdk_pixbuf_sys:gdk_pixbuf_save_to_stream() }
    //}

    //pub fn save_to_stream_async<P: IsA<gio::OutputStream>, Q: IsA<gio::Cancellable>, R: FnOnce(Result<(), Error>) + Send + 'static>(&self, stream: &P, type_: &str, cancellable: Option<&Q>, callback: R, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call gdk_pixbuf_sys:gdk_pixbuf_save_to_stream_async() }
    //}

    //#[cfg(feature = "futures")]
    //pub fn save_to_stream_async_future<P: IsA<gio::OutputStream> + Clone + 'static>(&self, stream: &P, type_: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Box_<dyn future::Future<Output = Result<(), Error>> + std::marker::Unpin> {
    //use gio::GioFuture;
    //use fragile::Fragile;

    //let stream = stream.clone();
    //let type_ = String::from(type_);
    //GioFuture::new(self, move |obj, send| {
    //    let cancellable = gio::Cancellable::new();
    //    let send = Fragile::new(send);
    //    obj.save_to_stream_async(
    //        &stream,
    //        &type_,
    //        Some(&cancellable),
    //        ,
    //        move |res| {
    //            let _ = send.into_inner().send(res);
    //        },
    //    );

    //    cancellable
    //})
    //}

    pub fn scale(
        &self,
        dest: &Pixbuf,
        dest_x: i32,
        dest_y: i32,
        dest_width: i32,
        dest_height: i32,
        offset_x: f64,
        offset_y: f64,
        scale_x: f64,
        scale_y: f64,
        interp_type: InterpType,
    ) {
        unsafe {
            gdk_pixbuf_sys::gdk_pixbuf_scale(
                self.to_glib_none().0,
                dest.to_glib_none().0,
                dest_x,
                dest_y,
                dest_width,
                dest_height,
                offset_x,
                offset_y,
                scale_x,
                scale_y,
                interp_type.to_glib(),
            );
        }
    }

    pub fn scale_simple(
        &self,
        dest_width: i32,
        dest_height: i32,
        interp_type: InterpType,
    ) -> Option<Pixbuf> {
        unsafe {
            from_glib_full(gdk_pixbuf_sys::gdk_pixbuf_scale_simple(
                self.to_glib_none().0,
                dest_width,
                dest_height,
                interp_type.to_glib(),
            ))
        }
    }

    pub fn set_option(&self, key: &str, value: &str) -> bool {
        unsafe {
            from_glib(gdk_pixbuf_sys::gdk_pixbuf_set_option(
                self.to_glib_none().0,
                key.to_glib_none().0,
                value.to_glib_none().0,
            ))
        }
    }

    pub fn get_property_pixel_bytes(&self) -> Option<glib::Bytes> {
        unsafe {
            let mut value = Value::from_type(<glib::Bytes as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"pixel-bytes\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value.get()
        }
    }

    //pub fn get_property_pixels(&self) -> /*Unimplemented*/Fundamental: Pointer {
    //    unsafe {
    //        let mut value = Value::from_type(</*Unknown type*/ as StaticType>::static_type());
    //        gobject_sys::g_object_get_property(self.as_ptr() as *mut gobject_sys::GObject, b"pixels\0".as_ptr() as *const _, value.to_glib_none_mut().0);
    //        value.get().unwrap()
    //    }
    //}

    #[cfg(any(feature = "v2_36_8", feature = "dox"))]
    pub fn calculate_rowstride(
        colorspace: Colorspace,
        has_alpha: bool,
        bits_per_sample: i32,
        width: i32,
        height: i32,
    ) -> i32 {
        unsafe {
            gdk_pixbuf_sys::gdk_pixbuf_calculate_rowstride(
                colorspace.to_glib(),
                has_alpha.to_glib(),
                bits_per_sample,
                width,
                height,
            )
        }
    }

    pub fn get_formats() -> Vec<PixbufFormat> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(gdk_pixbuf_sys::gdk_pixbuf_get_formats())
        }
    }
}

impl fmt::Display for Pixbuf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Pixbuf")
    }
}
