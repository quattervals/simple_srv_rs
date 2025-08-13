use std::fs;
use std::io::Write;

pub fn generate_inc_from_cargolock(lockfile_path: &str, output_path: &str) -> std::io::Result<()> {
    let cargo_contents = fs::read_to_string(lockfile_path)?;

    let mut src_uri_lines = Vec::new();
    let mut checksum_lines = Vec::new();

    src_uri_lines.push("SRC_URI += \" \\".to_string());

    // Split the file by `[[package]]`, which serves as the delimiter for blocks
    for block in cargo_contents.split("\n\n") {
        if block.starts_with("[[package]]") {
            // Extract the name, version, and checksum from the block
            let name = block
                .lines()
                .find(|line| line.starts_with("name = "))
                .map(|line| line.split('=').nth(1).unwrap().trim().replace('"', ""));
            let version = block
                .lines()
                .find(|line| line.starts_with("version = "))
                .map(|line| line.split('=').nth(1).unwrap().trim().replace('"', ""));
            let checksum = block
                .lines()
                .find(|line| line.starts_with("checksum = "))
                .map(|line| line.split('=').nth(1).unwrap().trim().replace('"', ""));

            // Ensure all values are present
            if let (Some(name), Some(version), Some(checksum)) = (name, version, checksum) {
                src_uri_lines.push(format!("    crate://crates.io/{}/{} \\", name, version));
                checksum_lines.push(format!(
                    "SRC_URI[{}-{}.sha256sum] = \"{}\"",
                    name, version, checksum
                ));
            } else {
                eprintln!("Warning: Skipping a package block due to missing fields");
            }
        }
    }

    // Remove trailing backslash from the last SRC_URI line
    if let Some(last_line) = src_uri_lines.last_mut() {
        last_line.truncate(last_line.len() - 2);
    }
    src_uri_lines.push("\"".to_string());

    // Write to output file
    let mut output = fs::File::create(output_path)?;
    writeln!(output, "{}", src_uri_lines.join("\n"))?;
    writeln!(output)?; // Blank line between sections
    writeln!(output, "{}", checksum_lines.join("\n"))?;

    Ok(())
}
