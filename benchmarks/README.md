# Benchmarks

Run the full benchmark suite (including the C reference and query-only run):

```
cargo run -p benchmarks
```

Run a subset of benchmarks:

```
BOLT_BENCH_FILTER=bolt,quadtree_rs cargo run -p benchmarks
```

The C benchmark is compiled in headless mode with the upstream `test.c` settings.

Environment variables:
- `BOLT_BENCH_ENTITIES`: set the entity count for all benchmarks (including c-quadtree).
- `BOLT_BENCH_TICKS`: set the tick count for all benchmarks (including c-quadtree).
- `BOLT_C_QUADTREE_DIR`: use an existing checkout instead of cloning.
- `BOLT_C_QUADTREE_REPO`: override the clone URL (default: `https://github.com/supahero1/c-quadtree`).
- `BOLT_C_QUADTREE_UPDATE`: set to `1` to `git pull` the repo before building.
