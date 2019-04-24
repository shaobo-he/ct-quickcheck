# ct-quickcheck
QuickCheck for Contant-Time Implementations  
~~This project is infinitely suspended due to Rust's quicheckcheck crate's lack of quality~~  
We will use proptest instead which is much proper than Rust's quickcheck.

## Overview
This project is intended to be a practice of Rust programming language and an experiment that addresses drawbacks of ct-fuzz (at the expense of losing some advantages).

The issues of ct-fuzz that we want to address are,
1. Precondition satisfaction by input generation as oppsed to bypassing unsatisfying inputs
2. Type-safe and precise annotations

