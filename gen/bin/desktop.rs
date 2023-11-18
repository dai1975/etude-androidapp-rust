
fn main() {
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    etude_androidapp_rust::main().unwrap();
}
