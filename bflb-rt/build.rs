use std::path::PathBuf;
use std::{env, fs};


fn main() {

    let chips = [
        (env::var("CARGO_FEATURE_BL808_M0").is_ok(), "bl808_m0"),
        (env::var("CARGO_FEATURE_BL808_D0").is_ok(), "bl808_d0"),
        (env::var("CARGO_FEATURE_BL808_LP").is_ok(), "bl808_lp"),
    ];

    let mut chip_selected = None;

    for (enable, chip) in chips {
        if enable {
            if chip_selected.is_none() {
                chip_selected = Some(chip);
            } else {
                eprintln!("Chip feature {chip} will be ignored because a multiple ones are enabled.");
                std::process::exit(1);
            }
        }
    }

    let out_dir: PathBuf = env::var("OUT_DIR").unwrap().into();
    let link_file = out_dir.join("link.x");

    if let Some(chip_selected) = chip_selected {

        let mut in_link_file: PathBuf = env::var("CARGO_MANIFEST_DIR").unwrap().into();
        in_link_file.push("src");
        in_link_file.push("link");
        in_link_file.push(format!("{chip_selected}.x"));

        fs::copy(in_link_file, link_file).unwrap();
        println!("cargo:rustc-cfg=rt_chip=\"{chip_selected}\"");
        println!("cargo:rustc-cfg=rt_chip_ok");
        println!("cargo:rustc-link-search={}", out_dir.display());

    } else if link_file.is_file() {
        fs::remove_file(link_file).unwrap();
    }

}
