extern crate libc;

extern {
    fn __ct_fuzz_initialize_states();
    fn __ct_fuzz_test_observations() -> c_int;
}

mod ct_quickcheck {
    fn initialize_states() {
        unsafe {
            __ct_fuzz_initialize_states();
        }
    }
    fn test_observation() -> bool {
        unsafe {
            __ct_fuzz_test_observation() != 0
        }
    }
}
