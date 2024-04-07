use include_dir::include_dir;

fn add_sdl2_libs() {
    // Include the directory containing SDL2 library files
    let sdl2_libs_dir = include_dir!("sdl2_libs");

    // Emit the directory as a Rust module
    sdl2_libs_dir
        .as_files()
        .iter()
        .for_each(|file| println!("cargo:rerun-if-changed={}", file.path().display()));
    sdl2_libs_dir
        .as_files()
        .iter()
        .for_each(|file| println!("cargo:warning={}", file.path().display()));

    // For each file, emit it as a byte array in a Rust module
    sdl2_libs_dir
        .as_files()
        .iter()
        .for_each(|file| println!("cargo:rustc-env={}={}", file.path().display(), include_str!(file)));
}
