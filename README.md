# Crat

**C**-to-**R**ust **A**utomatic **T**ranslator

```bash
# Run extern-resolving transformation for all benchmarks
./tool.py transform resolve

# Run extern-resolving transformation for a specific benchmark
./tool.py transform resolve avl

# Run extern-resolving transformation for a specific benchmark
# even if it already has been transformed
./tool.py transform resolve avl --force

# Build all transformed benchmarks
./tool.py build resolve

# Build a specific transformed benchmark
./tool.py build resolve avl

# Build a specific transformed benchmark in a release mode
./tool.py build resolve avl --release

# Clean the transformed benchmarks
./tool.py clean resolve

# Clean a specific transformed benchmark
./tool.py clean resolve avl
```
