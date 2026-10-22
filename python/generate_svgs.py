import sys
import pathlib

def main():
    if len(sys.argv) < 2:
        print("Usage: python generate_svgs.py <path_to_svg_directory>")
        sys.exit(1)

    svg_dir = pathlib.Path(sys.argv[1])
    if not svg_dir.exists():
        print(f"Error: Directory not found -> {svg_dir}")
        sys.exit(1)

    output_lines = []

    for svg_file in sorted(svg_dir.glob("*.svg")):
        var_name = svg_file.stem.upper().replace("-","_")
        file_name = svg_file.name

        byte_const = f'const {var_name}_BYTES: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/svg/{file_name}"));'

        output_lines.append(byte_const)

    print("\n".join(output_lines))

if __name__ == "__main__":
    main()