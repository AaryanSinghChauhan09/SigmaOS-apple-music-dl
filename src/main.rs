//! SigmaOS-apple-music-dl — thin CLI entry point.
//!
//! The library lives in `src/lib.rs`; this binary only wires argv/stdout.

use sigmaos_apple_music_dl_lib::Component;

fn main() {
    let mut component = Component::new();
    if let Err(e) = component.init().and_then(|_| component.run()) {
        eprintln!("SigmaOS-apple-music-dl: {e}");
        std::process::exit(1);
    }
    println!("SigmaOS-apple-music-dl: ok");
}
