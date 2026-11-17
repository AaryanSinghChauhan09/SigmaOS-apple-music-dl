// Integration test for SigmaOS-apple-music-dl
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use apple_music_dl as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
