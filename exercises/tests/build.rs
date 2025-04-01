//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

// build.rs
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // 1. 设置环境变量 TEST_FOO（用于 tests7）
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // 启用 "pass" 特性
    println!("cargo:rerun-if-changed=build.rs"); // 确保每次修改 build.rs 文件时重新运行构建脚本
    println!("cargo:feature=pass"); // 启用 "pass" 特性
}