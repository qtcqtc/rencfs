fn main() {
    if std::env::var_os("CARGO_CFG_TARGET_OS").is_some_and(|target| target == "windows") {
        #[cfg(target_os = "windows")]
        winfsp_wrs_build::build();
    }
}
