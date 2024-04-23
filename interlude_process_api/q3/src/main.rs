use rusl::process::fork;
use rusl::unistd::pipe;

fn main() {
    let pipe = pipe().unwrap();
    let pid = unsafe { fork().unwrap() };

    if pid == 0 {
        println!("hello!");
        rusl::unistd::write(pipe.out_pipe, &[1u8; 1]).unwrap();
    } else if pid > 0 {
        let mut buf = [0u8; 1];
        rusl::unistd::read(pipe.in_pipe, &mut buf).unwrap();
        assert_eq!(1u8, buf[0]);
        println!("goodbye!");
    } else {
        panic!("Negative pid={pid}");
    }
}
