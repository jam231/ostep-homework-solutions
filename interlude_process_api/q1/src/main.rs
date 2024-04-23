use rusl::process::fork;

fn main() {
    let mut x = 0;

    unsafe {
        let pid = fork();
        println!("x = {x} (initial x == 0)");
        x = match pid {
            Ok(v) if v == 0 => x,
            Ok(v) if v > 0 => v,
            Ok(v) => panic!("PID has negative value: {v}"),
            Err(err) => panic!("Error during forking: {err}"),
        };

        println!("x has value {x} (parent if x == 0)");
    }
}
