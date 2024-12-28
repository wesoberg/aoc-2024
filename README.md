# aoc-2024-rs

For 2024 I'm trying [Advent of Code](https://adventofcode.com/) on a Raspberry Pi 5 with 8 GB RAM.

![Raspberry Pi 5 with 8 GB RAM](docs/machine.jpg)

It's kitted out with:

* Official silicone bumper
* Official active cooler
* Boots from external SSD over USB3
* Runs Raspberry Pi OS Lite 64-bit (Debian 12 (bookworm))

Development happens over SSH in [Neovim](https://neovim.io/) (BTW).

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
| 2024 | 14  | 0.0021587371826171874 | 0.04154508113861084   |
| 2024 | 15  | 0.0077830076217651365 | 0.009727537631988525  |
| 2024 | 16  | 0.013852167129516601  | 0.04991086721420288   |
| 2024 | 17  | 0.0006073594093322753 | 0.000746142864227295  |
| 2024 | 18  | 0.0022567033767700194 | 0.00218348503112793   |
| 2024 | 19  | 0.04730072021484375   | 0.09182169437408447   |
| 2024 | 20  | 0.007873153686523438  | 0.16768847703933715   |
| 2024 | 21  |                       |                       |
| 2024 | 22  | 0.0092812180519104    | 0.21672009229660033   |
| 2024 | 23  | 0.025630879402160644  | 0.02193518877029419   |
| 2024 | 24  | 0.001909637451171875  | 3.255921506881714     |
| 2024 | 25  | 0.0014955759048461913 |                       |
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

* Hashing is really slow in Rust! This first came up on
  [2024-06-b](src/bin/year2024day06b.rs) (which to be fair I should revisit,
  that one was a slog for a while for me + Rust). As a result, I've
  occasionally dropped in `FxHash*` to get a speed boost (shave off half a
  second or more, in release mode). It's still kind of lame when the flamegraph
  only shows hashing operations, though.
    * https://nnethercote.github.io/perf-book/hashing.html
* The [itertools](https://docs.rs/itertools/latest/itertools/) crate can be
  really slow! For [2024-07-b](src/bin/year2024day07b.rs) I had even
  materialized all the permutations _for caching_ (which _was faster_). Turns
  out that's just a suboptimal approach to the problem altogether (explicitly
  generating permutations to then iterate).
* As soon as I refactored ~10 days' worth of copypasta 2D grid utilities into
  the shared lib, there were no more 2D grids. Wonderful. I did this refactor
  while despairing at potential edge cases in my implementation for 2024-21-a
  (which I've ended up saving as the last day to tackle).
* "Inspect the input" came up at least twice. Always good to keep in mind, and
  always something I try to avoid _anyway_, to my own peril. I must relearn the
  lesson each time, it seems.
* Most brutal problem of the year for me was
  [2024-24-b](src/bin/year2024day24b.rs) because I refused to "study up on the
  Foo Bar Baz" and wanted to tackle it as a relative idiot to "Foo Bar Baz"
  (otherwise it felt unsatisfying for whatever reason). Though seeing some of
  the generated images on the subreddit, and being able to make some
  assumptions about the overall structure, was useful.
  * Really bad idea to [clone in a recursive
    function](docs/year2024day24b-flamegraph-highlight.png). :D
* Spent much less time fighting the Rust compiler this year, that was nice,
  though I did tactically avoid trying lifetimes again. I would like to go back
  through and see what could be cleaned up, and maybe enhanced with more
  Rust-isms.
  * Moving the 2D grid utilities into the lib may have been my first real
    start-to-finish foray into implementing my own generics. Wild that you
    can't say "any whole number" type constraint in Rust without a crate! It's
    all about the waddles and the quacks (_behavior_ expressed by traits)...
