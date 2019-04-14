use nix::unistd::{fork, ForkResult}
mod ct_quickcheck {
    pub fn main() -> bool {
        // set up args
        initialize_states();
        // check spec
        // start ob
        for x in 0..1 {
           match fork() {
            Ok(ForkResult::Parent { child, ...}) => {
                // wait
            }
            Ok(ForkResult::Child) => {
                // call the function to test
            }
            Err(_) => {
                println!("Oops");
                // probably crashes
            }
           } 
        }
    }
}
