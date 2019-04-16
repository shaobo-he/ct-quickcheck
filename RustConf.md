# ct-quickcheck: QuickChecking Contant-Time Implmentations

## Abstract
Attacks based on timing side-channels have been proven practical.
Constant-time implementations are a counter-measure to timing side-channel attacks by enforcing programs not to leak any timing information.
A verification tool and a fuzzer to check constant-time implementations have been proposed but suffer from practical issues compared to random testing.
We propose ct-quickcheck, a tool based on Rust's implmentation of Haskell's QuickCheck to test lack of timing information leaks.
To be more specific, we leverage Rust's advanced macro system to translate a Rust-like language to a driver program.
Such a transformation is concise while flexible as oppsed to the existing transformation implemented in C++.
Moreover, we also use Rust's FFI to interact C libraries that monitor timing information.
In summary, we would like to show that Rust's language features allow fast yet efficient tool prototying.
