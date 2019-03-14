use bindgen;
use cc;
use std::env;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    cc::Build::new()
        .file("ili9225spi_rpi/ili9225.c")
        .file("ili9225spi_rpi/fontx.c")
        .warnings(false)
        .compile("libili9225.a");
    println!("cargo:rustc-link-lib=bcm2835");
    println!("cargo:rustc-link-lib=m");

    let bindings = bindgen::Builder::default()
        .header_contents(
            "wrapper.h",
            "\
            #include <stdio.h>
            #include <stdint.h>
            #include <stdbool.h>
            #include \"ili9225spi_rpi/ili9225.h\"\
        ",
        )
        .generate()
        .expect("Unable to generate bindings");

    let bindings_file = Path::new(&out_dir).join("ili9225.rs");
    bindings
        .write_to_file(bindings_file)
        .expect("Couldn't write to bindings output file");

    println!("cargo:rerun-if-changed=ili9225spi_rpi/ili9225.c");
    println!("cargo:rerun-if-changed=ili9225spi_rpi/ili9225.h");
    println!("cargo:rerun-if-changed=ili9225spi_rpi/fontx.h");
    println!("cargo:rerun-if-changed=ili9225spi_rpi/fontx.c");
}
