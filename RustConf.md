# ct-quickcheck: QuickChecking Contant-Time Implementations

## Abstract
Attacks based on timing side-channels have been proven practical.
Constant-time implementations are a counter-measure to timing side-channel attacks by enforcing programs not to leak any timing information.
Verification tools and fuzzers have been proposed to check constant-time implementations but they experience practical issues compared to random testing.
We propose ct-quickcheck, a tool based on Rust's implementation of Haskell's QuickCheck to test lack of timing information leaks.
We leverage Rust's advanced macro system to translate annotations in Rust-like syntax to a driver program.
Such a transformation is concise while flexible as opposed to the existing transformation implemented in C++.
Moreover, we use Rust's FFI to interact with C libraries that monitor timing information.
