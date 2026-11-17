//! Integration tests for SigmaOS-apple-music-dl.

use sigmaos_apple_music_dl_lib::Component;

#[test]
fn component_initialises_and_runs() {
    let mut c = Component::new();
    c.init().expect("init should succeed");
    c.run().expect("run should succeed after init");
}
