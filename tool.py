#!/usr/bin/env python3

import os
import subprocess
import sys
from pathlib import Path
import shutil
import signal

BENCH_ROOT = Path("benchmarks")
CONFIG_ROOT = BENCH_ROOT / "configs"
BIN_TEST_DIR = BENCH_ROOT / "bin-tests"
SCRIPT_TEST_DIR = BENCH_ROOT / "script-tests"
RS_ORIG = BENCH_ROOT / "rs"
TRANSFORM_DIRS = {
    "resolve": BENCH_ROOT / "rs-resolved",
    "union": BENCH_ROOT / "rs-union",
    "io": BENCH_ROOT / "rs-io",
    "union-io": BENCH_ROOT / "rs-union-io",
    "io-union": BENCH_ROOT / "rs-io-union",
}
TRANSFORM_ORDER = {
    "resolve": None,
    "union": "resolve",
    "io": "resolve",
    "union-io": "union",
    "io-union": "io",
}
TRANSFORM_PASS = {
    "resolve": "assert,extern,bin",
    "union": "union",
    "io": "io",
    "union-io": "io",
    "io-union": "union",
}
TRANSFORM_CONFIG = {
    "resolve": CONFIG_ROOT / "resolve",
    "union": CONFIG_ROOT / "union",
    "io": CONFIG_ROOT / "io",
    "union-io": CONFIG_ROOT / "io",
    "io-union": CONFIG_ROOT / "union",
}

current_dest = None

def handle_interrupt(signum, frame):
    global current_dest
    print("\n[Interrupt] Caught Ctrl+C")
    if current_dest and current_dest.exists():
        print(f"[Remove] {current_dest}")
        shutil.rmtree(current_dest)
    sys.exit(1)

signal.signal(signal.SIGINT, handle_interrupt)

def run_cargo(source_dir, dest_dir, passes, config_path=None):
    global current_dest
    cmd = ["cargo", "run", "--release", "--", "-o", str(dest_dir), "--pass", passes]
    if config_path and config_path.exists():
        cmd.extend(["--config", str(config_path)])
    cmd.append(str(source_dir))
    print("[Running]", " ".join(cmd))
    current_dest = dest_dir / source_dir.name
    try:
        subprocess.run(cmd, check=True)
    except subprocess.CalledProcessError:
        print(f"[Error] Transformation failed: {source_dir} -> {dest_dir}")
        if current_dest.exists():
            print(f"[Remove] {current_dest}")
            shutil.rmtree(current_dest)
        sys.exit(1)
    finally:
        current_dest = None

def transform_one(stage, bench, force=False):
    dest = TRANSFORM_DIRS[stage] / bench
    if dest.exists() and not force:
        print(f"[Skip] {dest} already exists (use --force to override)")
        return

    src_stage = TRANSFORM_ORDER[stage]
    src = RS_ORIG / bench if src_stage is None else TRANSFORM_DIRS[src_stage] / bench

    if src_stage and not src.exists():
        transform_one(src_stage, bench, force)

    config_dir = TRANSFORM_CONFIG[stage]
    config_file = config_dir / f"{bench}.toml"
    config = config_file if config_file.exists() else None

    if not dest.parent.exists():
        dest.parent.mkdir(parents=True)

    run_cargo(src, dest.parent, TRANSFORM_PASS[stage], config)

def list_benchmarks():
    return sorted(p.name for p in RS_ORIG.iterdir() if p.is_dir())

def clean(stage, bench=None):
    path = TRANSFORM_DIRS[stage]
    if bench:
        target = path / bench
        if target.exists():
            print(f"[Remove] {target}")
            shutil.rmtree(target)
    else:
        if path.exists():
            print(f"[Remove] {path}")
            shutil.rmtree(path)

def build(stage, bench=None, release=False):
    def run_build(path):
        env = os.environ.copy()
        env["RUSTFLAGS"] = "-Awarnings"
        cmd = ["cargo", "build"]
        if release:
            cmd.append("--release")
        print(f"[Building] {path}")
        try:
            subprocess.run(cmd, cwd=path, env=env, check=True)
        except subprocess.CalledProcessError as e:
            print(f"[Error] Build failed: {path}")
            sys.exit(1)

    dest_root = TRANSFORM_DIRS[stage]
    if bench:
        dest_path = dest_root / bench
        if not dest_path.exists():
            transform_one(stage, bench)
        run_build(dest_path)
    else:
        for bench in list_benchmarks():
            dest_path = dest_root / bench
            if not dest_path.exists():
                transform_one(stage, bench)
            run_build(dest_path)

def test_one(stage, bench, release=False):
    build(stage, bench, release)

    print(f"[Test] {bench}")

    test_file = BIN_TEST_DIR / f"{bench}.txt"
    script_file = SCRIPT_TEST_DIR / f"{bench}.sh"

    if test_file.exists():
        bench_dir = TRANSFORM_DIRS[stage] / bench
        bin_subdir = "release" if release else "debug"
        with open(test_file) as f:
            for line in f:
                line = line.strip()
                if not line or line.startswith("#"):
                    continue

                expected_failure = line.startswith("!")
                name = line[1:] if expected_failure else line
                bin_path = Path("target") / bin_subdir / name

                print(f"[Exec] {bin_path}")
                result = subprocess.run([str(bin_path)], cwd=bench_dir)
                success = (result.returncode == 0 and not expected_failure) or \
                          (result.returncode != 0 and expected_failure)
                status = "ok" if success else "FAIL"
                print(f"  => {status}")

                if not success:
                    sys.exit(1)

    elif script_file.exists():
        tmp_dir = BENCH_ROOT / "tmp"
        tmp_dir.mkdir(exist_ok=True)

        cmd = [str(script_file), str(tmp_dir), str(TRANSFORM_DIRS[stage])]
        if release:
            cmd.append("--release")

        print(f"[Exec] {' '.join(cmd)}")
        try:
            subprocess.run(cmd, check=True)
            print("  => ok")
        except subprocess.CalledProcessError:
            print("  => FAIL")
            sys.exit(1)

    else:
        print(f"[Skip] no test: {bench}")

def test(stage, bench=None, release=False):
    if bench:
        test_one(stage, bench, release)
    else:
        for b in list_benchmarks():
            test_one(stage, b, release)

def main():
    if len(sys.argv) < 3:
        print("Usage: tool.py <transform|build|test|clean> <stage> [BENCHMARK] [--force|--release]")
        sys.exit(1)

    mode = sys.argv[1]
    stage = sys.argv[2]

    if stage not in TRANSFORM_DIRS:
        print(f"Unknown stage: {stage}")
        sys.exit(1)

    force = "--force" in sys.argv
    release = "--release" in sys.argv
    args = [arg for arg in sys.argv[3:] if arg not in ("--force", "--release")]

    if mode == "transform":
        if args:
            transform_one(stage, args[0], force)
        else:
            for bench in list_benchmarks():
                transform_one(stage, bench, force)
    elif mode == "build":
        if args:
            build(stage, args[0], release)
        else:
            build(stage, release=release)
    elif mode == "test":
        if args:
            test(stage, args[0], release)
        else:
            test(stage, release=release)
    elif mode == "clean":
        if args:
            clean(stage, args[0])
        else:
            clean(stage)
    else:
        print(f"Unknown mode: {mode}")
        sys.exit(1)

if __name__ == "__main__":
    main()

