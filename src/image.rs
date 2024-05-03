use std::ptr::{null, null_mut};

use core_foundation::{
    base::{CFTypeID, TCFType},
    string::{CFString, CFStringRef},
};
use libc::{c_void, size_t};
#[cfg(feature = "objc")]
use objc2::encode::{Encoding, RefEncode};

use crate::{
    base::CGFloat,
    color_space::{CGColorRenderingIntent, CGColorSpace, CGColorSpaceRef},
    data_provider::{CGDataProvider, CGDataProviderRef},
    geometry::CGRect,
};

#[repr(C)]
pub struct __CGImage(c_void);

pub type CGImageRef = *mut __CGImage;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CGImageAlphaInfo {
    #[doc(alias = "kCGImageAlphaNone")]
    AlphaNone,
    #[doc(alias = "kCGImageAlphaPremultipliedLast")]
    AlphaPremultipliedLast,
    #[doc(alias = "kCGImageAlphaPremultipliedFirst")]
    AlphaPremultipliedFirst,
    #[doc(alias = "kCGImageAlphaLast")]
    AlphaLast,
    #[doc(alias = "kCGImageAlphaFirst")]
    AlphaFirst,
    #[doc(alias = "kCGImageAlphaNoneSkipLast")]
    AlphaNoneSkipLast,
    #[doc(alias = "kCGImageAlphaNoneSkipFirst")]
    AlphaNoneSkipFirst,
    #[doc(alias = "kCGImageAlphaOnly")]
    AlphaOnly,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CGImageByteOrderInfo {
    #[doc(alias = "kCGImageByteOrderMask")]
    ByteOrderMask     = 0x7000,
    #[doc(alias = "kCGImageByteOrderDefault")]
    ByteOrderDefault  = 0 << 12,
    #[doc(alias = "kCGImageByteOrder16Little")]
    ByteOrder16Little = 1 << 12,
    #[doc(alias = "kCGImageByteOrder32Little")]
    ByteOrder32Little = 2 << 12,
    #[doc(alias = "kCGImageByteOrder16Big")]
    ByteOrder16Big    = 3 << 12,
    #[doc(alias = "kCGImageByteOrder32Big")]
    ByteOrder32Big    = 4 << 12,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CGImagePixelFormatInfo {
    #[doc(alias = "kCGImagePixelFormatMask")]
    PixelFormatMask      = 0xF000,
    #[doc(alias = "kCGImagePixelFormatPacked")]
    PixelFormatPacked    = 0 << 16,
    #[doc(alias = "kCGImagePixelFormatRGB")]
    PixelFormatRGB555    = 1 << 16,
    #[doc(alias = "kCGImagePixelFormatRGB")]
    PixelFormatRGB565    = 2 << 16,
    #[doc(alias = "kCGImagePixelFormatRGB")]
    PixelFormatRGB101010 = 3 << 16,
    #[doc(alias = "kCGImagePixelFormatRGB")]
    PixelFormatRGBCIF10  = 4 << 16,
}

bitflags! {
    #[repr(C)]
    #[derive(Clone, Copy, Debug, Default, PartialEq)]
    pub struct CGBitmapInfo: u32 {
        #[doc(alias = "kCGBitmapAlphaInfoMask")]
        const AlphaInfoMask = 0x1F;
        #[doc(alias = "kCGBitmapFloatInfoMask")]
        const FloatInfoMask = 0xF00;
        #[doc(alias = "kCGBitmapFloatComponents")]
        const FloatComponents = 1 << 8;
        #[doc(alias = "kCGBitmapByteOrderMask")]
        const ByteOrderMask = CGImageByteOrderInfo::ByteOrderMask as u32;
        #[doc(alias = "kCGBitmapByteOrderDefault")]
        const ByteOrderDefault = CGImageByteOrderInfo::ByteOrderDefault as u32;
        #[doc(alias = "kCGBitmapByteOrder16Little")]
        const ByteOrder16Little = CGImageByteOrderInfo::ByteOrder16Little as u32;
        #[doc(alias = "kCGBitmapByteOrder32Little")]
        const ByteOrder32Little = CGImageByteOrderInfo::ByteOrder32Little as u32;
        #[doc(alias = "kCGBitmapByteOrder16Big")]
        const ByteOrder16Big = CGImageByteOrderInfo::ByteOrder16Big as u32;
        #[doc(alias = "kCGBitmapByteOrder32Big")]
        const ByteOrder32Big = CGImageByteOrderInfo::ByteOrder32Big as u32;
        #[doc(alias = "kCGBitmapByteOrder16Host")]
        const ByteOrder16Host = kCGBitmapByteOrder16Host;
        #[doc(alias = "kCGBitmapByteOrder32Host")]
        const ByteOrder32Host = kCGBitmapByteOrder32Host;
    }
}

cfg_if! {
    if #[cfg(target_endian = "big")] {
        pub const kCGBitmapByteOrder16Host: u32 = CGImageByteOrderInfo::ByteOrder16Big as u32;
        pub const kCGBitmapByteOrder32Host: u32 = CGImageByteOrderInfo::ByteOrder32Big as u32;
    } else {
        pub const kCGBitmapByteOrder16Host: u32 = CGImageByteOrderInfo::ByteOrder16Little as u32;
        pub const kCGBitmapByteOrder32Host: u32 = CGImageByteOrderInfo::ByteOrder32Little as u32;
    }
}

extern "C" {
    pub fn CGImageGetTypeID() -> CFTypeID;
    pub fn CGImageCreate(
        width: size_t,
        height: size_t,
        bitsPerComponent: size_t,
        bitsPerPixel: size_t,
        bytesPerRow: size_t,
        space: CGColorSpaceRef,
        bitmapInfo: u32,
        provider: CGDataProviderRef,
        decode: *const CGFloat,
        shouldInterpolate: bool,
        intent: CGColorRenderingIntent,
    ) -> CGImageRef;
    pub fn CGImageMaskCreate(
        width: size_t,
        height: size_t,
        bitsPerComponent: size_t,
        bitsPerPixel: size_t,
        bytesPerRow: size_t,
        provider: CGDataProviderRef,
        decode: *const CGFloat,
        shouldInterpolate: bool,
    ) -> CGImageRef;
    pub fn CGImageCreateCopy(image: CGImageRef) -> CGImageRef;
    pub fn CGImageCreateWithJPEGDataProvider(
        provider: CGDataProviderRef,
        decode: *const CGFloat,
        shouldInterpolate: bool,
        intent: CGColorRenderingIntent,
    ) -> CGImageRef;
    pub fn CGImageCreateWithPNGDataProvider(
        provider: CGDataProviderRef,
        decode: *const CGFloat,
        shouldInterpolate: bool,
        intent: CGColorRenderingIntent,
    ) -> CGImageRef;
    pub fn CGImageCreateWithImageInRect(image: CGImageRef, rect: CGRect) -> CGImageRef;
    pub fn CGImageCreateWithMask(image: CGImageRef, mask: CGImageRef) -> CGImageRef;
    pub fn CGImageCreateWithMaskingColors(image: CGImageRef, components: *const CGFloat) -> CGImageRef;
    pub fn CGImageCreateCopyWithColorSpace(image: CGImageRef, space: CGColorSpaceRef) -> CGImageRef;
    pub fn CGImageRetain(image: CGImageRef) -> CGImageRef;
    pub fn CGImageRelease(image: CGImageRef);
    pub fn CGImageIsMask(image: CGImageRef) -> bool;
    pub fn CGImageGetWidth(image: CGImageRef) -> size_t;
    pub fn CGImageGetHeight(image: CGImageRef) -> size_t;
    pub fn CGImageGetBitsPerComponent(image: CGImageRef) -> size_t;
    pub fn CGImageGetBitsPerPixel(image: CGImageRef) -> size_t;
    pub fn CGImageGetBytesPerRow(image: CGImageRef) -> size_t;
    pub fn CGImageGetColorSpace(image: CGImageRef) -> CGColorSpaceRef;
    pub fn CGImageGetAlphaInfo(image: CGImageRef) -> CGImageAlphaInfo;
    pub fn CGImageGetDataProvider(image: CGImageRef) -> CGDataProviderRef;
    pub fn CGImageGetDecode(image: CGImageRef) -> *const CGFloat;
    pub fn CGImageGetShouldInterpolate(image: CGImageRef) -> bool;
    pub fn CGImageGetRenderingIntent(image: CGImageRef) -> CGColorRenderingIntent;
    pub fn CGImageGetBitmapInfo(image: CGImageRef) -> CGBitmapInfo;
    pub fn CGImageGetByteOrderInfo(image: CGImageRef) -> CGImageByteOrderInfo;
    pub fn CGImageGetPixelFormatInfo(image: CGImageRef) -> CGImagePixelFormatInfo;
    pub fn CGImageGetUTType(image: CGImageRef) -> CFStringRef;
}

pub struct CGImage(CGImageRef);

impl Drop for CGImage {
    fn drop(&mut self) {
        unsafe { CGImageRelease(self.0) }
    }
}

impl_TCFType!(CGImage, CGImageRef, CGImageGetTypeID);
impl_CFTypeDescription!(CGImage);

impl CGImage {
    pub fn new(
        width: size_t,
        height: size_t,
        bits_per_component: size_t,
        bits_per_pixel: size_t,
        bytes_per_row: size_t,
        space: Option<&CGColorSpace>,
        bitmap_info: u32,
        provider: Option<&CGDataProvider>,
        decode: Option<&[CGFloat]>,
        should_interpolate: bool,
        intent: CGColorRenderingIntent,
    ) -> Option<Self> {
        unsafe {
            let image = CGImageCreate(
                width,
                height,
                bits_per_component,
                bits_per_pixel,
                bytes_per_row,
                space.map_or(null_mut(), |s| s.as_concrete_TypeRef()),
                bitmap_info,
                provider.map_or(null(), |p| p.as_concrete_TypeRef()),
                decode.map_or(null(), |d| d.as_ptr()),
                should_interpolate,
                intent,
            );
            if image.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(image))
            }
        }
    }

    pub fn new_mask(
        width: size_t,
        height: size_t,
        bits_per_component: size_t,
        bits_per_pixel: size_t,
        bytes_per_row: size_t,
        provider: Option<&CGDataProvider>,
        decode: Option<&[CGFloat]>,
        should_interpolate: bool,
    ) -> Option<Self> {
        unsafe {
            let image = CGImageMaskCreate(
                width,
                height,
                bits_per_component,
                bits_per_pixel,
                bytes_per_row,
                provider.map_or(null(), |p| p.as_concrete_TypeRef()),
                decode.map_or(null(), |d| d.as_ptr()),
                should_interpolate,
            );
            if image.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(image))
            }
        }
    }

    pub fn new_copy(image: &CGImage) -> Option<Self> {
        unsafe {
            let image = CGImageCreateCopy(image.as_concrete_TypeRef());
            if image.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(image))
            }
        }
    }

    pub fn new_copy_with_color_space(image: &CGImage, space: &CGColorSpace) -> Option<Self> {
        unsafe {
            let image = CGImageCreateCopyWithColorSpace(image.as_concrete_TypeRef(), space.as_concrete_TypeRef());
            if image.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(image))
            }
        }
    }

    pub fn from_jpeg_data_provider(
        provider: &CGDataProvider,
        decode: Option<&[CGFloat]>,
        should_interpolate: bool,
        intent: CGColorRenderingIntent,
    ) -> Option<Self> {
        unsafe {
            let image =
                CGImageCreateWithJPEGDataProvider(provider.as_concrete_TypeRef(), decode.map_or(null(), |d| d.as_ptr()), should_interpolate, intent);
            if image.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(image))
            }
        }
    }

    pub fn from_png_data_provider(
        provider: &CGDataProvider,
        decode: Option<&[CGFloat]>,
        should_interpolate: bool,
        intent: CGColorRenderingIntent,
    ) -> Option<Self> {
        unsafe {
            let image =
                CGImageCreateWithPNGDataProvider(provider.as_concrete_TypeRef(), decode.map_or(null(), |d| d.as_ptr()), should_interpolate, intent);
            if image.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(image))
            }
        }
    }

    pub fn from_mask(image: Option<&CGImage>, mask: Option<&CGImage>) -> Option<Self> {
        unsafe {
            let image =
                CGImageCreateWithMask(image.map_or(null_mut(), |i| i.as_concrete_TypeRef()), mask.map_or(null_mut(), |m| m.as_concrete_TypeRef()));
            if image.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(image))
            }
        }
    }

    pub fn from_masking_colors(image: &CGImage, components: &[CGFloat]) -> Option<Self> {
        unsafe {
            let color_space = image.color_space()?;
            let num_components = color_space.number_of_components();
            if components.len() != num_components * 2 {
                return None;
            }
            let image = CGImageCreateWithMaskingColors(image.as_concrete_TypeRef(), components.as_ptr());
            if image.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(image))
            }
        }
    }

    pub fn cropped(image: &CGImage, rect: CGRect) -> Option<Self> {
        unsafe {
            let image = CGImageCreateWithImageInRect(image.as_concrete_TypeRef(), rect);
            if image.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(image))
            }
        }
    }

    pub fn is_mask(&self) -> bool {
        unsafe { CGImageIsMask(self.as_concrete_TypeRef()) }
    }

    pub fn width(&self) -> size_t {
        unsafe { CGImageGetWidth(self.as_concrete_TypeRef()) }
    }

    pub fn height(&self) -> size_t {
        unsafe { CGImageGetHeight(self.as_concrete_TypeRef()) }
    }

    pub fn bits_per_component(&self) -> size_t {
        unsafe { CGImageGetBitsPerComponent(self.as_concrete_TypeRef()) }
    }

    pub fn bits_per_pixel(&self) -> size_t {
        unsafe { CGImageGetBitsPerPixel(self.as_concrete_TypeRef()) }
    }

    pub fn bytes_per_row(&self) -> size_t {
        unsafe { CGImageGetBytesPerRow(self.as_concrete_TypeRef()) }
    }

    pub fn color_space(&self) -> Option<CGColorSpace> {
        unsafe {
            let space = CGImageGetColorSpace(self.as_concrete_TypeRef());
            if space.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_get_rule(space))
            }
        }
    }

    pub fn alpha_info(&self) -> CGImageAlphaInfo {
        unsafe { CGImageGetAlphaInfo(self.as_concrete_TypeRef()) }
    }

    pub fn data_provider(&self) -> Option<CGDataProvider> {
        unsafe {
            let provider = CGImageGetDataProvider(self.as_concrete_TypeRef());
            if provider.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_get_rule(provider))
            }
        }
    }

    pub fn should_interpolate(&self) -> bool {
        unsafe { CGImageGetShouldInterpolate(self.as_concrete_TypeRef()) }
    }

    pub fn rendering_intent(&self) -> CGColorRenderingIntent {
        unsafe { CGImageGetRenderingIntent(self.as_concrete_TypeRef()) }
    }

    pub fn bitmap_info(&self) -> CGBitmapInfo {
        unsafe { CGImageGetBitmapInfo(self.as_concrete_TypeRef()) }
    }

    pub fn byte_order_info(&self) -> CGImageByteOrderInfo {
        unsafe { CGImageGetByteOrderInfo(self.as_concrete_TypeRef()) }
    }

    pub fn pixel_format_info(&self) -> CGImagePixelFormatInfo {
        unsafe { CGImageGetPixelFormatInfo(self.as_concrete_TypeRef()) }
    }

    pub fn ut_type(&self) -> Option<CFString> {
        unsafe {
            let ut_type = CGImageGetUTType(self.as_concrete_TypeRef());
            if ut_type.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_get_rule(ut_type))
            }
        }
    }
}

#[cfg(feature = "objc")]
unsafe impl RefEncode for __CGImage {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Encoding::Struct("CGImage", &[]));
}
