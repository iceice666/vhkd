use vhkd_daemon::macos;

fn main() {
    #[cfg(target_os = "macos")]
    macos::mainloop();
}
