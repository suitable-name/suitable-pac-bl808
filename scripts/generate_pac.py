#!/usr/bin/env python3
"""
BL808 PAC Generator Script
Automates XInclude expansion, svd2rust code generation, module splitting (form),
and cleanup for all BL808 cores (M0, D0, LP).
"""

import argparse
import os
import shutil
import subprocess
import sys
import xml.etree.ElementTree as ET

REPO_ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))

CORES = {
    "m0": {
        "xml": "svd/bl808_m0.xml",
        "svd": "bl808_m0.svd",
        "crate": "crates/bl808-m0",
        "settings": "riscv_bl808_settings.yaml",
        "desc": "M0 (E907, 32-bit RISC-V)",
    },
    "d0": {
        "xml": "svd/bl808_d0.xml",
        "svd": "bl808_d0.svd",
        "crate": "crates/bl808-d0",
        "settings": "riscv_bl808_d0_settings.yaml",
        "desc": "D0 (C906, 64-bit RISC-V)",
    },
    "lp": {
        "xml": "svd/bl808_lp.xml",
        "svd": "bl808_lp.svd",
        "crate": "crates/bl808-lp",
        "settings": "riscv_bl808_lp_settings.yaml",
        "desc": "LP (E902, 32-bit RISC-V)",
    },
}

def expand_xinclude(elem, base_dir):
    ns = "{http://www.w3.org/2001/XInclude}"
    for child in list(elem):
        if child.tag == f"{ns}include":
            href = child.attrib.get("href")
            inc_path = os.path.normpath(os.path.join(base_dir, href))
            inc_tree = ET.parse(inc_path)
            inc_root = inc_tree.getroot()
            expand_xinclude(inc_root, os.path.dirname(inc_path))
            idx = list(elem).index(child)
            elem.remove(child)
            elem.insert(idx, inc_root)
        else:
            expand_xinclude(child, base_dir)

def generate_core(core_name):
    cfg = CORES[core_name]
    print(f"=== Generating PAC for {core_name.upper()} ({cfg['desc']}) ===")
    
    xml_path = os.path.join(REPO_ROOT, cfg["xml"])
    svd_path = os.path.join(REPO_ROOT, cfg["svd"])
    crate_dir = os.path.join(REPO_ROOT, cfg["crate"])
    src_dir = os.path.join(crate_dir, "src")
    settings_path = os.path.join(REPO_ROOT, cfg["settings"])
    
    # 1. Expand XInclude
    print(f"  [1/4] Expanding XInclude: {cfg['xml']} -> {cfg['svd']}")
    tree = ET.parse(xml_path)
    expand_xinclude(tree.getroot(), os.path.dirname(xml_path))
    tree.write(svd_path, encoding="utf-8", xml_declaration=True)
    
    # 2. Run svd2rust
    print(f"  [2/4] Running svd2rust with {cfg['settings']}...")
    cmd = [
        "svd2rust",
        "--target", "riscv",
        "--settings", settings_path,
        "-i", svd_path,
    ]
    res = subprocess.run(cmd, cwd=REPO_ROOT, capture_output=True, text=True)
    if res.returncode != 0:
        print(f"Error running svd2rust:\n{res.stderr}", file=sys.stderr)
        sys.exit(1)
        
    # 3. Modularize with form
    print(f"  [3/4] Modularizing into {cfg['crate']}/src/ using form...")
    os.makedirs(src_dir, exist_ok=True)
    lib_rs = os.path.join(REPO_ROOT, "lib.rs")
    device_x = os.path.join(REPO_ROOT, "device.x")
    root_build_rs = os.path.join(REPO_ROOT, "build.rs")
    
    cmd = ["form", "-i", lib_rs, "-o", src_dir]
    res = subprocess.run(cmd, cwd=REPO_ROOT, capture_output=True, text=True)
    if res.returncode != 0:
        print(f"Error running form:\n{res.stderr}", file=sys.stderr)
        sys.exit(1)
        
    # Move device.x into crate directory
    if os.path.exists(device_x):
        dest_device_x = os.path.join(crate_dir, "device.x")
        shutil.move(device_x, dest_device_x)
        
    # 4. Clean up root temporary files
    print("  [4/4] Cleaning temporary root files...")
    for temp in [svd_path, lib_rs, root_build_rs, device_x]:
        if os.path.exists(temp):
            os.remove(temp)
            
    print(f"[OK] {core_name.upper()} PAC generation complete.\n")

def main():
    parser = argparse.ArgumentParser(description="Generate PAC crates for BL808")
    parser.add_argument(
        "--core",
        choices=["m0", "d0", "lp", "all"],
        default="all",
        help="Core to generate (default: all)",
    )
    args = parser.parse_args()
    
    if args.core == "all":
        for c in ["m0", "d0", "lp"]:
            generate_core(c)
    else:
        generate_core(args.core)
        
    print("Running cargo fmt...")
    subprocess.run(["cargo", "fmt"], cwd=REPO_ROOT)
    print("All done!")

if __name__ == "__main__":
    main()
