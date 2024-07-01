fn main() {
    // Compile-time build information.
    built::write_built_file().expect("Failed to write build information");
}
