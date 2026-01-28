// src/main.rs
use std::backtrace::{Backtrace, BacktraceStatus};

// 递归函数，用于生成多层调用栈
fn function_d(n: u32) -> Backtrace {
    println!("Function D called with n = {}", n);
    if n == 0 {
        // 捕获当前调用栈
        let bt = Backtrace::capture();
        println!("Capturing stack trace at base case:");
        bt
    } else {
        function_d(n - 1)
    }
}

fn function_c() {
    println!("Function C called");
    let _ = function_d(2);
}

fn function_b() {
    println!("Function B called");
    function_c();
}

fn function_a() {
    println!("Function A called");
    function_b();
}

// 自定义panic钩子，在panic时打印栈跟踪
fn set_panic_hook() {
    std::panic::set_hook(Box::new(|panic_info| {
        println!("\n=== PANIC OCCURRED ===");
        println!("Panic info: {}", panic_info);

        // 强制捕获栈跟踪（即使没有设置RUST_BACKTRACE环境变量）
        let bt = Backtrace::force_capture();
        if bt.status() == BacktraceStatus::Captured {
            println!("\n=== STACK TRACE ===");
            println!("{}", bt);
        } else {
            println!("Failed to capture backtrace");
        }
    }));
}

fn main() {
    println!("=== Rust Stack Trace Program ===");

    // 设置自定义panic钩子
    set_panic_hook();

    // 正常情况下的栈跟踪
    println!("\n--- Normal Stack Trace ---");
    function_a();

    // 演示panic时的栈跟踪
    println!("\n--- Testing Panic Handler ---");
    println!("Enter 'panic' to trigger a panic with stack trace, or anything else to exit:");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    if input.trim() == "panic" {
        // 触发panic，自定义钩子会捕获并打印栈跟踪
        panic!("This is a test panic!");
    }

    println!("Program completed normally.");
}
