# rust-goldenfile-bug

```sh
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/bug`
thread 'main' panicked at 'assertion failed: false', src/main.rs:17:5
note: Run with `RUST_BACKTRACE=1` for a backtrace.

Goldenfile diff for "test":
To regenerate the goldenfile, run
    env REGENERATE_GOLDENFILES=1 cargo test
------------------------------------------------------------

data

thread 'main' panicked at 'assertion failed: edit distance between "data" and "" is 2 and not 0, see diffset above', /Users/slava/.cargo/registry/src/github.com-1ecc6299db9ec823/difference-0.4.1/src/lib.rs:86:9
stack backtrace:
   0:        0x10c6aaed3 - std::sys::unix::backtrace::tracing::imp::unwind_backtrace::h0d82ff21dbaafa37
                               at libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1:        0x10c6a5cf2 - std::sys_common::backtrace::_print::h8d6c54c009c3f040
                               at libstd/sys_common/backtrace.rs:71
   2:        0x10c6aa29d - std::panicking::default_hook::{{closure}}::h9fa6009de31be346
                               at libstd/sys_common/backtrace.rs:59
                               at libstd/panicking.rs:380
   3:        0x10c6a9f35 - std::panicking::default_hook::h6cdceadee11175c8
                               at libstd/panicking.rs:396
   4:        0x10c6aa686 - std::panicking::begin_panic::he256843cb68b70a7
                               at libstd/panicking.rs:576
   5:        0x10c6aa4de - std::panicking::begin_panic::h083a5eed938a1a63
                               at libstd/panicking.rs:537
   6:        0x10c6aa432 - std::panicking::try::do_call::h050ffe02358f9942
                               at libstd/panicking.rs:521
   7:        0x10c69979b - alloc::slice::<impl [T]>::as_mut_ptr::hb89f852e3e2449f2
                               at /Users/slava/.cargo/registry/src/github.com-1ecc6299db9ec823/difference-0.4.1/src/lib.rs:86
   8:        0x10c6785e8 - <core::slice::Iter<'a, T> as core::iter::iterator::Iterator>::next::h97fa042f79fdc854
                               at /Users/slava/.cargo/registry/src/github.com-1ecc6299db9ec823/goldenfile-0.5.1/src/differs.rs:14
   9:        0x10c66ed70 - core::ops::function::Fn::call::h95e5d085642e01a4
                               at /Users/travis/build/rust-lang/rust/src/libcore/ops/function.rs:73
  10:        0x10c673b54 - <alloc::allocator::Layout as core::clone::Clone>::clone::ha60e40961dddb5a9
                               at /Users/slava/.cargo/registry/src/github.com-1ecc6299db9ec823/goldenfile-0.5.1/src/mint.rs:79
  11:        0x10c6741b0 - <alloc::allocator::Layout as core::clone::Clone>::clone::ha60e40961dddb5a9
                               at /Users/slava/.cargo/registry/src/github.com-1ecc6299db9ec823/goldenfile-0.5.1/src/mint.rs:109
  12:        0x10c66eea4 - core::ptr::drop_in_place::h16e62b6b03e70910
                               at /Users/travis/build/rust-lang/rust/src/libcore/ptr.rs:59
  13:        0x10c6725e8 - bug::main::h545cbf4573d1abf5
                               at src/main.rs:18
  14:        0x10c6715b1 - std::rt::lang_start::{{closure}}::hb4b31eddf4178fc7
                               at /Users/travis/build/rust-lang/rust/src/libstd/rt.rs:74
  15:        0x10c6aa357 - std::panicking::try::do_call::h050ffe02358f9942
                               at libstd/rt.rs:59
                               at libstd/panicking.rs:479
  16:        0x10c6b7d8e - panic_unwind::dwarf::eh::read_encoded_pointer::he5373975d0777777
                               at libpanic_unwind/lib.rs:102
  17:        0x10c6aa98d - rust_panic
                               at libstd/panicking.rs:458
                               at libstd/panic.rs:358
                               at libstd/rt.rs:58
  18:        0x10c671591 - std::rt::lang_start::h97626fa35df2fbee
                               at /Users/travis/build/rust-lang/rust/src/libstd/rt.rs:74
  19:        0x10c672684 - bug::main::h545cbf4573d1abf5
thread panicked while panicking. aborting.
Illegal instruction: 4
```

```
$ cargo test
   Compiling bug v0.1.0 (file:///private/tmp/bug)
    Finished dev [unoptimized + debuginfo] target(s) in 1.13 secs
     Running target/debug/deps/bug-ac9065eb809b91af

running 1 test
thread panicked while panicking. aborting.
error: process didn't exit successfully: `/private/tmp/bug/target/debug/deps/bug-ac9065eb809b91af` (signal: 4, SIGILL: illegal instruction)
```

```
$ rustc --version --verbose
rustc 1.25.0 (84203cac6 2018-03-25)
binary: rustc
commit-hash: 84203cac67e65ca8640b8392348411098c856985
commit-date: 2018-03-25
host: x86_64-apple-darwin
release: 1.25.0
LLVM version: 6.0
```
