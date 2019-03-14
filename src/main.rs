use std::io::{Read, Result};

#[allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]
mod ffi {
    include!(concat!(env!("OUT_DIR"), "/ili9225.rs"));
}

const XMAX: u16 = 176;
const YMAX: u16 = 220;

const XMAX2: u16 = 175;
const YMAX2: u16 = 219;

struct Font {
    fonts: [ffi::FontxFile; 2],
    paths: [std::ffi::CString; 2],
}

impl Font {
    fn new<T1: Into<String>, T2: Into<String>>(path1: T1, path2: T2) -> Result<Self> {
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

struct Lcd();

impl Lcd {
    fn new(model: u16, width: u16, height: u16) -> Self {
        unsafe {
            ffi::lcdInit(model, width, height);
            ffi::lcdReset();
            ffi::lcdSetup();

            ffi::lcdFillScreen(ffi::WHITE as u16);
            ffi::lcdSetFontDirection(ffi::DIRECTION90 as u16);
        }
        Lcd()
    }

    fn draw_utf8_string<T: Into<String>>(
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

fn main() -> Result<()> {
    let lcd = Lcd::new(0x9225, XMAX, YMAX);
    let fxg16 = Font::new(
        "ili9225spi_rpi/fontx/ILGH16XB.FNT",
        "ili9225spi_rpi//fontx/ILGZ16XB.FNT",
    )?;
    let fxm16 = Font::new(
        "ili9225spi_rpi/fontx/ILMH16XB.FNT",
        "ili9225spi_rpi//fontx/ILMZ16XF.FNT",
    )?;
    let fxg24 = Font::new(
        "ili9225spi_rpi/fontx/ILGH24XB.FNT",
        "ili9225spi_rpi//fontx/ILGZ24XB.FNT",
    )?;
    let fxm24 = Font::new(
        "ili9225spi_rpi/fontx/ILMH24XF.FNT",
        "ili9225spi_rpi//fontx/ILMZ24XF.FNT",
    )?;

    let mut xpos;
    let mut ypos;
    let color;

    xpos = XMAX2 - (16 * 1);
    ypos = YMAX2;
    color = ffi::BLACK as u16;
    lcd.draw_utf8_string(&fxg16, xpos, ypos, "16Dot Gothic", color);

    xpos = XMAX2 - (16 * 2);
    lcd.draw_utf8_string(&fxg16, xpos, ypos, "ABCDEFGabcdefg", color);

    xpos = XMAX2 - (16 * 3);
    ypos = YMAX2;
    lcd.draw_utf8_string(&fxm16, xpos, ypos, "16Dot Mincho", color);

    xpos = XMAX2 - (16 * 4);
    lcd.draw_utf8_string(&fxm16, xpos, ypos, "ABCDEFGabcdefg", color);

    xpos = XMAX2 - (16 * 6) - (24 * 0);
    ypos = YMAX2;
    lcd.draw_utf8_string(&fxg24, xpos, ypos, "24Dot Gothic", color);

    xpos = XMAX2 - (16 * 6) - (24 * 1);
    lcd.draw_utf8_string(&fxg24, xpos, ypos, "ABCDEFGabcdefg", color);

    xpos = XMAX2 - (16 * 6) - (24 * 2);
    ypos = YMAX2;
    lcd.draw_utf8_string(&fxm24, xpos, ypos, "24Dot Mincho", color);

    xpos = XMAX2 - (16 * 6) - (24 * 3);
    lcd.draw_utf8_string(&fxm24, xpos, ypos, "ABCDEFGabcdefg", color);

    println!("Hit any key");
    let mut b = [0u8; 1];
    std::io::stdin().read(&mut b)?;

    Ok(())
}
