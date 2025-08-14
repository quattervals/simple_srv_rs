import re

def parse_cargo_lock(input_path, output_path):
    """
    Parse Cargo.lock and convert it into the desired .inc file format for Yocto.

    :param input_path: Path to the Cargo.lock file
    :param output_path: Path to the output .inc file
    """
    with open(input_path, "r") as f:
        cargo_contents = f.read()

    # Parse individual package blocks using regex that captures until the end of the file or double newlines
    package_blocks = re.findall(r'\[\[package\]\](.*?)(?:\n\n|\Z)', cargo_contents, re.DOTALL)

    # Extract relevant fields (name, version, checksum) from each block
    packages = []
    for block in package_blocks:
        name_match = re.search(r'name = "([^"]+)"', block)
        version_match = re.search(r'version = "([^"]+)"', block)
        checksum_match = re.search(r'checksum = "([^"]+)"', block)

        if name_match and version_match and checksum_match:
            name = name_match.group(1)
            version = version_match.group(1)
            checksum = checksum_match.group(1)
            packages.append((name, version, checksum))

    # Create the SRC_URI section
    src_uri_lines = ["SRC_URI += \" \\"]
    src_uri_lines += [f'    crate://crates.io/{name}/{version} \\' for name, version, _ in packages]
    # src_uri_lines[-1] = src_uri_lines[-1][:-2]  # Remove trailing " \\"
    src_uri_lines.append('"')  # Close the SRC_URI line

    # Create the checksum mappings
    checksum_lines = [
        f'SRC_URI[{name}-{version}.sha256sum] = "{checksum}"'
        for name, version, checksum in packages
    ]

    # Write the output .inc file
    with open(output_path, "w") as f:
        f.write("\n".join(src_uri_lines))
        f.write("\n\n")
        f.write("\n".join(checksum_lines))
        f.write("\n")

if __name__ == "__main__":
    # Replace with your input and output file paths
    input_cargo_lock = "Cargo.lock"
    output_inc_file = "output.inc"

    parse_cargo_lock(input_cargo_lock, output_inc_file)
    print(f"Converted {input_cargo_lock} to {output_inc_file}")
