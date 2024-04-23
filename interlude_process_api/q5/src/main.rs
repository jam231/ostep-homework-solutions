use rusl::platform::WaitPidFlags;
use rusl::process::fork;
use rusl::process::wait_pid;

fn main() {
    let pid = unsafe { fork().unwrap() };

    // Per https://man7.org/linux/man-pages/man2/waitpid.2.html
    // `waitpid` is equivalent to `waitpid(-1, &wstatus, 0);`
    if pid == 0 {
        // This would return ECHILD: 10 - No child processes
        //wait_pid(-1, WaitPidFlags::empty()).unwrap();
        println!("hello!");
    } else if pid > 0 {
        wait_pid(-1, WaitPidFlags::empty()).unwrap();
        println!("goodbye!");
    } else {
        panic!("Negative pid={pid}");
    }
}
