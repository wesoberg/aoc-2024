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

```
| year | day | part | avg 20 trials (release) |
| ---  | --- | ---  | ---                     |
| 2024 | 01  | a    | 0.000736689567565918    |
| 2024 | 01  | b    | 0.0009833335876464843   |
| 2024 | 02  | a    | 0.0013817667961120606   |
| 2024 | 02  | b    | 0.0014211654663085938   |
| 2024 | 03  | a    | 0.0016565322875976562   |
| 2024 | 03  | b    | 0.002190244197845459    |
| 2024 | 04  | a    | 0.009282290935516357    |
| 2024 | 04  | b    | 0.008666455745697021    |
| 2024 | 05  | a    | 0.006460726261138916    |
| 2024 | 05  | b    | 0.02109997272491455     |
| 2024 | 06  | a    | 0.0035663247108459473   |
| 2024 | 06  | b    | 0.09860951900482177     |
| 2024 | 07  | a    | 0.002495479583740234    |
| 2024 | 07  | b    | 0.03600732088088989     |
| 2024 | 08  | a    | 0.0007539629936218262   |
| 2024 | 08  | b    | 0.001094663143157959    |
| 2024 | 09  | a    | 0.00602569580078125     |
| 2024 | 09  | b    | 0.008433377742767334    |
| 2024 | 10  | a    | 0.002243328094482422    |
| 2024 | 10  | b    | 0.0022826552391052245   |
| 2024 | 11  | a    | 0.0036974430084228517   |
| 2024 | 11  | b    | 0.029175734519958495    |
| 2024 | 12  | a    | 0.02530210018157959     |
| 2024 | 12  | b    | 0.02868642807006836     |
| 2024 | 13  | a    | 0.0021719932556152344   |
| 2024 | 13  | b    | 0.002286362648010254    |
| 2024 | 14  | a    | 0.0021587371826171874   |
| 2024 | 14  | b    | 0.07314798831939698     |
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


