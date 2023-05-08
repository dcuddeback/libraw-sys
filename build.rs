fn main() {
    pkg_config::Config::new()
        .atleast_version("0.21.0")
        .probe("libraw_r")
        .unwrap();
}
