use ili9225::{Lcd, Font, BLACK};
use std::io::{Result, Read};

const XMAX: u16 = 176;
const YMAX: u16 = 220;

const XMAX2: u16 = 175;
const YMAX2: u16 = 219;

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
    color = BLACK as u16;
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
