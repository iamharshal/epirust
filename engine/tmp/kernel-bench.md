# EpiRust Kernel Benchmarks

Kernels are functions called in loop bodies. 
Hyperfine has executed these benchmarks.

## Without kernels

* Without native CPU optimizations
```
Benchmark #1: cargo run --release -- -c config/default_10K.json
  Time (mean ± σ):      5.937 s ±  3.181 s    [User: 5.889 s, System: 0.030 s]
  Range (min … max):    0.811 s …  9.002 s    10 runs

Benchmark #1: cargo run --release -- -c config/default_50K.json
  Time (mean ± σ):     47.488 s ± 21.299 s    [User: 47.279 s, System: 0.065 s]
  Range (min … max):    4.967 s … 62.427 s    10 runs

Benchmark #1: cargo run --release -- -c config/default_100K.json
  Time (mean ± σ):     137.361 s ± 80.061 s    [User: 136.761 s, System: 0.160 s]
  Range (min … max):   12.060 s … 224.681 s    10 runs
```

* With native CPU optimizations

```
Benchmark #1: RUSTFLAGS='-C target-cpu=native -C target-feature=+sse -C target-feature=+sse2 -C target-feature=+sse3 -C target-feature=+sse4.1 -C target-feature=+sse4.2' cargo run --release -- -c config/default_10K.json
  Time (mean ± σ):      5.804 s ±  2.924 s    [User: 5.754 s, System: 0.032 s]
  Range (min … max):    1.378 s …  8.730 s    10 runs
 
Benchmark #1: RUSTFLAGS='-C target-cpu=native -C target-feature=+sse -C target-feature=+sse2 -C target-feature=+sse3 -C target-feature=+sse4.1 -C target-feature=+sse4.2' cargo run --release -- -c config/default_50K.json
  Time (mean ± σ):     33.323 s ± 21.163 s    [User: 33.166 s, System: 0.062 s]
  Range (min … max):    8.857 s … 54.258 s    10 runs
 
Benchmark #1: RUSTFLAGS='-C target-cpu=native -C target-feature=+sse -C target-feature=+sse2 -C target-feature=+sse3 -C target-feature=+sse4.1 -C target-feature=+sse4.2' cargo run --release -- -c config/default_100K.json
  Time (mean ± σ):     127.983 s ± 61.551 s    [User: 127.522 s, System: 0.124 s]
  Range (min … max):   20.382 s … 175.501 s    10 runs
```

## With kernels

* Without native CPU optimization

```
Benchmark #1: cargo run --release -- -c config/default_10K.json
  Time (mean ± σ):      5.576 s ±  3.001 s    [User: 5.521 s, System: 0.033 s]
  Range (min … max):    1.445 s …  8.342 s    10 runs

Benchmark #1: cargo run --release -- -c config/default_50K.json
  Time (mean ± σ):     23.586 s ± 22.642 s    [User: 23.471 s, System: 0.052 s]
  Range (min … max):    5.198 s … 59.961 s    10 runs

Benchmark #1: cargo run --release -- -c config/default_100K.json
  Time (mean ± σ):     144.721 s ± 69.953 s    [User: 144.085 s, System: 0.164 s]
  Range (min … max):   22.857 s … 205.899 s    10 runs
```

* With native CPU optimization

```
Benchmark #1: RUSTFLAGS='-C target-cpu=native -C target-feature=+sse -C target-feature=+sse2 -C target-feature=+sse3 -C target-feature=+sse4.1 -C target-feature=+sse4.2' cargo run --release -- -c config/default_10K.json
  Time (mean ± σ):      4.559 s ±  3.055 s    [User: 4.518 s, System: 0.028 s]
  Range (min … max):    0.939 s …  7.833 s    10 runs
 
Benchmark #1: RUSTFLAGS='-C target-cpu=native -C target-feature=+sse -C target-feature=+sse2 -C target-feature=+sse3 -C target-feature=+sse4.1 -C target-feature=+sse4.2' cargo run --release -- -c config/default_50K.json
  Time (mean ± σ):     41.775 s ± 22.946 s    [User: 41.607 s, System: 0.058 s]
  Range (min … max):    8.837 s … 62.005 s    10 runs
 
Benchmark #1: RUSTFLAGS='-C target-cpu=native -C target-feature=+sse -C target-feature=+sse2 -C target-feature=+sse3 -C target-feature=+sse4.1 -C target-feature=+sse4.2' cargo run --release -- -c config/default_100K.json
  Time (mean ± σ):     94.810 s ± 80.097 s    [User: 94.474 s, System: 0.094 s]
  Range (min … max):   10.505 s … 178.747 s    10 runs
```

## Summary

Refactored and CPU-native optimized kernel version takes 94 seconds on aveage versus the non-kernel version taking 127 seconds.
