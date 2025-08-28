#!/usr/bin/env python3

import argparse
import os
from pathlib import Path
import subprocess

def find_benchmarks(root: Path):
    for p in sorted(root.iterdir()):
        if p.is_dir():
            yield p

def run_one(bench_path: Path, out_dir: Path, bin_name: str, subcmd: str, cargo_extra: list[str]) -> tuple[str, int]:
    name = bench_path.name
    out_file = out_dir / f"{name}.txt"
    err_file = out_dir / f"{name}.err"

    cmd = [
        "cargo", "run", "--bin", bin_name, *cargo_extra, "--", subcmd, str(bench_path)
    ]

    out_dir.mkdir(parents=True, exist_ok=True)

    with out_file.open("wb") as out_f, err_file.open("wb") as err_f:
        proc = subprocess.run(
            cmd,
            stdout=out_f,
            stderr=err_f,
            cwd=None,
            env={**os.environ, "CARGO_TERM_COLOR": "never"}
        )
        return name, proc.returncode

def main():
    parser = argparse.ArgumentParser(description="Run crat-finder over all benchmarks in benchmarks/rs.")
    parser.add_argument("--bench-root", default="benchmarks/rs", type=Path, help="where benchmarks directories are")
    parser.add_argument("--out-dir", default="experiment-results", type=Path, help="where to save output files")
    parser.add_argument("--bin", default="crat-finder", dest="bin_name", help="cargo binary name to run")
    parser.add_argument("--subcmd", default="global", help="which finder to run (e.g., global, example)")
    parser.add_argument("--include", nargs="*", default=None,
                        help="if given, only included names are run")
    parser.add_argument("--exclude", nargs="*", default=None,
                        help="if given, excluded names are skipped")
    parser.add_argument("--cargo-extra", nargs="*", default=[],
                        help="extra args to pass to cargo (e.g., --release)")
    args = parser.parse_args()

    bench_paths = list(find_benchmarks(args.bench_root))

    def match_filters(p: Path) -> bool:
        n = p.name
        if args.include and not any(s in n for s in args.include):
            return False
        if args.exclude and any(s in n for s in args.exclude):
            return False
        return True

    targets = [p for p in bench_paths if match_filters(p)]
    if not targets:
        print("no target benchmarks to run.")
        return

    print(f"total of {len(targets)} benchmarks to run:")
    for t in targets:
        print(f"  - {t.name}")

    results = []
    for p in targets:
        print(f"[RUN] {p.name} ...")
        name, code = run_one(p, args.out_dir, args.bin_name, args.subcmd, args.cargo_extra)
        status = "OK" if code == 0 else f"FAIL({code})"
        print(f"[DONE] {name}: {status}")
        results.append((name, code))

    ok = sum(1 for _, c in results if c == 0)
    fail = [(n, c) for n, c in results if c != 0]
    print("\n=== Summary ===")
    print(f"Success: {ok} / {len(results)}")
    if fail:
        print("Fail:")
        for n, c in fail:
            print(f"  - {n}: exit {c} (log: {args.out_dir / (n + '.err')})")

if __name__ == "__main__":
    main()