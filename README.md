# atspi-counters

Count real-world prevalence of events per interface.

## Purpose

For the most efficient event parsing in [atspi](https://github.com/odilia-app/atspi), it matters (a little) in which order we match on the interface.
This utility tries to get an idea of the event prevalence per interface.

## usage

```Term
cargo run --release 
```

Press ctrl-C.

## Example output

```Term
Press Ctrl+C to stop the program and get stats

^C

Stats:
Total events: 6756
object: 4489 (66.44464%)
mouse: 2172 (32.1492%)
cache: 70 (1.036116%)
window: 15 (0.22202486%)
focus: 9 (0.13321492%)
document: 1 (0.014801658%)
terminal: 0 (0%)
keyboard: 0 (0%)
listener: 0 (0%)
available: 0 (0%)
```

## License  

MIT
