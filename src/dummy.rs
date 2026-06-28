pub mod dummy {
    use std::{path::PathBuf, thread, time};

    pub fn base(path: &PathBuf, mut args: Vec<String>) {
        args.drain(0..2);
        println!("{path:?}");
        println!(
            "Running dummy command with the following args: {}",
            args.len()
        );
        for arg in args {
            println!("{arg}");
        }

        println!("Dummy command doing dummy things...");
        thread::sleep(time::Duration::from_secs(5));
        println!("Dummy command has finished doing things")
    }
}
