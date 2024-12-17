# aoc-2024-rs

For 2024 I'm trying [Advent of Code](https://adventofcode.com/) on a Raspberry Pi 5 with 8 GB RAM.

![Raspberry Pi 5 with 8 GB RAM](docs/machine.jpg)

It's kitted out with:

* Official silicone bumper
* Official active cooler
* Boots from external SSD over USB3
* Runs Raspberry Pi OS Lite 64-bit (Debian 12 (bookworm))

All activities are over SSH, and development happens in [Neovim](https://neovim.io/) (BTW).

## timings

Average of 20 trials in release mode.

```
| year | day | part a                | part b                |
| ---  | --- | ---                   | ---                   |
| 2024 | 01  | 0.000736689567565918  | 0.0009833335876464843 |
| 2024 | 02  | 0.0013817667961120606 | 0.0014211654663085938 |
| 2024 | 03  | 0.0016565322875976562 | 0.002190244197845459  |
| 2024 | 04  | 0.009282290935516357  | 0.008666455745697021  |
| 2024 | 05  | 0.006460726261138916  | 0.02109997272491455   |
| 2024 | 06  | 0.0035663247108459473 | 0.09860951900482177   |
| 2024 | 07  | 0.002495479583740234  | 0.03600732088088989   |
| 2024 | 08  | 0.0007539629936218262 | 0.001094663143157959  |
| 2024 | 09  | 0.00602569580078125   | 0.008433377742767334  |
| 2024 | 10  | 0.002243328094482422  | 0.0022826552391052245 |
| 2024 | 11  | 0.0036974430084228517 | 0.029175734519958495  |
| 2024 | 12  | 0.02530210018157959   | 0.02868642807006836   |
| 2024 | 13  | 0.0021719932556152344 | 0.002286362648010254  |
| 2024 | 14  | 0.0021587371826171874 | 0.12479873895645141   |
| 2024 | 15  | 0.0077830076217651365 | 0.009727537631988525  |
| 2024 | 16  | 0.013852167129516601  | 0.11171153783798218   |
```

## deeper profiling

https://github.com/flamegraph-rs/flamegraph

```bash
cargo install flamegraph
time cargo flamegraph --bin year2024day01a --dev

python3 -m http.server 9000
```

View it: `http://<machine-name>:9000/flamegraph.svg`

## takeaways

* Hashing is really slow in Rust! This came up on 2024-06-b!
    * https://nnethercote.github.io/perf-book/hashing.html
* The [itertools](https://docs.rs/itertools/latest/itertools/) crate can be
  really slow! For 2024-07-b I had even materialized all the permutations for
  caching. Turns out that's just a suboptimal approach to the problem
  altogether (explicitly iterating permutations).


