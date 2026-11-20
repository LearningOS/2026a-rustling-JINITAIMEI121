//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

fn main() {
    // In tests7, we should set up an environment variable
    // called `TEST_FOO`. Print in the standard output to let
    // Cargo do it.
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs(); // The timestamp must be fresh for every build, so the
                     // emitted value has to be recomputed each time Cargo runs
                     // this build script (see `cargo:rerun-if-changed` below).
    let your_command = format!("rustc-env=TEST_FOO={}", timestamp);
    println!("cargo:{}", your_command);
    // Make Cargo re-run this build script on every build so that `TEST_FOO` is
    // never served from a cached (and therefore stale) value. Without this the
    // tests7 assertion `timestamp >= e && timestamp < e + 10` would fail on any
    // repeated grading run that reuses the target directory.
    println!("cargo:rerun-if-changed=build.rs");

    // In tests8, we should enable "pass" feature to make the
    // testcase return early. Fill in the command to tell
    // Cargo about that.
    let your_command = "rustc-cfg=feature=\"pass\"";
    println!("cargo:{}", your_command);
}
