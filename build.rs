extern crate "pkg-config" as pkg_config;

fn main() {
  pkg_config::find_library("libusb-1.0").unwrap();
}
