use std::io::Result;

#[allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]
mod ffi {
    include!(concat!(env!("OUT_DIR"), "/ili9225.rs"));
}

pub use ffi::BLACK;

pub struct Font {
    fonts: [ffi::FontxFile; 2],
    paths: [std::ffi::CString; 2],
}

impl Font {
    pub fn new<T1: Into<String>, T2: Into<String>>(path1: T1, path2: T2) -> Result<Self> {
        unsafe {
            let mut f = Font {
                fonts: [std::mem::uninitialized(); 2],
                paths: [
                    std::ffi::CString::new(path1.into()).unwrap(),
                    std::ffi::CString::new(path2.into()).unwrap(),
                ],
            };
            ffi::Fontx_init(&mut f.fonts[0], f.paths[0].as_ptr(), f.paths[1].as_ptr());

            for i in 0..2 {
                if !ffi::Fontx_openFontxFile(&mut f.fonts[i]) {
                    f.fonts[i].opened = false;
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::NotFound,
                        format!("{:?}", f.paths[i]),
                    ));
                }
            }

            Ok(f)
        }
    }

    fn as_ptr(&self) -> *const ffi::FontxFile {
        &self.fonts[0]
    }
}

#[test]
fn open_non_existent_font() {
    assert!(Font::new("not_there_at_all", "wow").is_err());
}

impl Drop for Font {
    fn drop(&mut self) {
        unsafe {
            ffi::Fontx_closeFontxFile(&mut self.fonts[0]);
            ffi::Fontx_closeFontxFile(&mut self.fonts[1]);
        }
    }
}

pub struct Lcd();

impl Lcd {
    pub fn new(model: u16, width: u16, height: u16) -> Self {
        unsafe {
            ffi::lcdInit(model, width, height);
            ffi::lcdReset();
            ffi::lcdSetup();

            ffi::lcdFillScreen(ffi::WHITE as u16);
            ffi::lcdSetFontDirection(ffi::DIRECTION90 as u16);
        }
        Lcd()
    }

    pub fn draw_utf8_string<T: Into<String>>(
        &self,
        font: &Font,
        x: u16,
        y: u16,
        s: T,
        color: u16,
    ) -> u16 {
        unsafe {
            let s = std::ffi::CString::new(s.into()).unwrap();
            ffi::lcdDrawUTF8String(font.as_ptr() as *mut _, x, y, s.as_ptr() as *mut _, color)
                as u16
        }
    }
}
