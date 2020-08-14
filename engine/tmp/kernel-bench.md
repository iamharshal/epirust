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

```
Performance counter stats for 'cargo run --release -- -c config/default_10K.json':

          3,881.33 msec task-clock:u              #    0.998 CPUs utilized
                 0      context-switches:u        #    0.000 K/sec
                 0      cpu-migrations:u          #    0.000 K/sec
             6,231      page-faults:u             #    0.002 M/sec
    16,859,482,867      cycles:u                  #    4.344 GHz                      (83.31%)
     6,674,982,164      stalled-cycles-frontend:u #   39.59% frontend cycles idle     (83.25%)
     2,698,422,066      stalled-cycles-backend:u  #   16.01% backend cycles idle      (66.77%)
    28,114,738,980      instructions:u            #    1.67  insn per cycle
                                                  #    0.24  stalled cycles per insn  (83.39%)
     2,926,331,094      branches:u                #  753.950 M/sec                    (83.37%)
       102,565,898      branch-misses:u           #    3.50% of all branches          (83.30%)

       3.890038595 seconds time elapsed

       3.820368000 seconds user
       0.030645000 seconds sys
```

```
Performance counter stats for 'cargo run --release -- -c config/default_100K.json':

         80,865.10 msec task-clock:u              #    1.000 CPUs utilized
                 0      context-switches:u        #    0.000 K/sec
                 0      cpu-migrations:u          #    0.000 K/sec
            24,894      page-faults:u             #    0.308 K/sec
   354,147,592,989      cycles:u                  #    4.379 GHz                      (83.33%)
   200,376,383,721      stalled-cycles-frontend:u #   56.58% frontend cycles idle     (83.33%)
   132,031,206,480      stalled-cycles-backend:u  #   37.28% backend cycles idle      (66.67%)
   397,587,256,370      instructions:u            #    1.12  insn per cycle
                                                  #    0.50  stalled cycles per insn  (83.33%)
    41,475,345,939      branches:u                #  512.895 M/sec                    (83.34%)
     1,806,734,718      branch-misses:u           #    4.36% of all branches          (83.33%)

      80.895438582 seconds time elapsed

      80.044078000 seconds user
       0.099871000 seconds sys
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

```
Performance counter stats for 'cargo run --release -- -c config/default_10K.json':

          3,551.88 msec task-clock:u              #    1.001 CPUs utilized
                 0      context-switches:u        #    0.000 K/sec
                 0      cpu-migrations:u          #    0.000 K/sec
             6,215      page-faults:u             #    0.002 M/sec
    15,397,001,515      cycles:u                  #    4.335 GHz                      (83.36%)
     5,954,037,953      stalled-cycles-frontend:u #   38.67% frontend cycles idle     (83.36%)
     2,471,914,094      stalled-cycles-backend:u  #   16.05% backend cycles idle      (66.55%)
    25,454,298,509      instructions:u            #    1.65  insn per cycle
                                                  #    0.23  stalled cycles per insn  (83.25%)
     2,762,015,101      branches:u                #  777.622 M/sec                    (83.37%)
        95,789,104      branch-misses:u           #    3.47% of all branches          (83.36%)

       3.547354860 seconds time elapsed

       3.495615000 seconds user
       0.028628000 seconds sys
```

```
Performance counter stats for 'cargo run --release -- -c config/default_100K.json':

         87,275.60 msec task-clock:u              #    1.000 CPUs utilized
                 0      context-switches:u        #    0.000 K/sec
                 0      cpu-migrations:u          #    0.000 K/sec
            24,886      page-faults:u             #    0.285 K/sec
   382,026,365,380      cycles:u                  #    4.377 GHz                      (83.33%)
   219,576,402,598      stalled-cycles-frontend:u #   57.48% frontend cycles idle     (83.33%)
   146,070,015,612      stalled-cycles-backend:u  #   38.24% backend cycles idle      (66.67%)
   421,606,961,505      instructions:u            #    1.10  insn per cycle
                                                  #    0.52  stalled cycles per insn  (83.33%)
    43,949,864,023      branches:u                #  503.576 M/sec                    (83.33%)
     1,908,341,698      branch-misses:u           #    4.34% of all branches          (83.33%)

      87.285226421 seconds time elapsed

      86.367952000 seconds user
       0.104928000 seconds sys
```

## Summary

Refactored and CPU-native optimized kernel version takes 94 seconds on aveage versus the non-kernel version taking 127 seconds.
