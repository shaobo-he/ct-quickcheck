# ct-quickcheck: QuickChecking Contant-Time Implementations

## Abstract
Attacks based on timing side-channels have been proven practical.
Constant-time implementations are a counter-measure to timing side-channel attacks by enforcing programs not to leak any timing information.
Verification tools and fuzzers have been proposed to check constant-time implementations but they experience practical issues compared to random testing.
We propose ct-quickcheck, a tool based on Rust's implementation of Haskell's QuickCheck to test lack of timing information leaks.
We leverage Rust's advanced macro system to generate a driver program based on annotations in Rust-like syntax.
Such a program generation is concise while flexible as opposed to the existing work implemented in C++.
Moreover, we use Rust's FFI to interact with C libraries that monitor timing information.

# Additional things

## Details
Please refer to the abstract

## (High) Pitch
In this talk, we attempt to demonstrate the power of Rust's macro systems as well as FFI for fast tool protyping. Specifically, we show that Rust's procedural macros are powerful enough to enable a convenient input program syntax and concise light-weight program generation. Furthermore, Rust's FFI allows us to leverage C legacy code. In combination, these techniques allow us to construct a working prototype in a short time.
