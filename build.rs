mod build_tools;

fn main() {
    let output_path = "bb_includes_dirty.inc";
    let lockfile_path = "Cargo.lock";

    // Generate .inc file
    if let Err(err) =
        build_tools::bb_inc_generator::generate_inc_from_cargolock(lockfile_path, output_path)
    {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
    println!("cargo:rerun-if-changed=Cargo.lock");
    println!("cargo:rerun-if-changed={}", output_path);
}
