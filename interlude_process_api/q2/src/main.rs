use rusl::process::fork;
use std::env;
use std::process;
use std::{fs::File, io::Write};

fn main() {
    let file_name = env::args()
        .nth(1)
        .expect("Target filename for process to write to");
    let mut file = File::create(file_name).unwrap();
    unsafe {
        let _ = fork().unwrap();
    }
    let pid = process::id();
    // parent process has a head start and writes ~30 lines first liners in the file
    for i in 1..=1000 {
        file.write(format!("pid: {pid} > wrote {i} line.\n").as_bytes())
            .unwrap();
    }
}
