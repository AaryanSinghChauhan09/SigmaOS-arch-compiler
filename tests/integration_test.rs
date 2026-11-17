// Integration test for SigmaOS-arch-compiler
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use arch_compiler as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
