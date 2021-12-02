# Advent of Code 2020

My solutions to Advent of Code 2020.

- Last year: <https://github.com/daniel5151/aoc19>

Some goals:

- Solve the questions (duh)
- Keep the code clean (comments when applicable, using idiomatic Rust, etc...)
- Solutions should have _reasonable_ (i.e: not strictly the _best_) space and time complexity
- Solutions should run fairly quickly (on modern PCs)

Some non-goals:

- Scoring super high on the leaderboard (timezones give people an unfair advantage, and late-night-coding isn't prime-time for me)

[Last year](https://github.com/daniel5151/aoc19) I did end up racing for a leaderboard spot, but unfortunately, I'm back on the east coast this year, and I am _definitely_ not frosty and ready to race at midnight.

## Running

(Assuming that the desired day's input has already been downloaded)

```bash
cargo run --release --features extras -- <day> <question>
```

Tests can be run using the standard `cargo test` flow.

```bash
cargo test -- dayX # only runs tests for the particular day
```

## Running (when solving the day of)

```bash
./aoc19.sh <day> <question>
# e.g: ./aoc19 3 1
```

The harness will automatically download question input if a `cookie.txt` is provided. It's contents should look something like this:

```
session=53616c...
```

Getting this cookie is fairly straightforward:
- Open Chrome
- Navigate to _any_ day's input URL (e.g: https://adventofcode.com/2020/day/1/input)
- Open the Chrome Network Inspector
- Refresh the URL
- Right click the `input` request, and "copy > copy as cURL"
    - the string should include a `-H 'cookie: <cookie.txt>'` component.

Alternatively, you can just invoke `cargo run --release -- <day> <question>` manually, though it will not automatically download input data.

## Q: Why use a macro to parse input?

Speed!

If I'd used a function, I'd have to explicitly specify the return type (e.g
`HashMap<usize, Vec<(usize, usize)>>` or what have you).

> Okay, so why not just use a closure then? That'll infer the types for you!

Yeah, sure, but then error handling would be more annoying, since the return
value would have to be wrapped with `Ok()`.

So yeah, it's a bit weird, but there is a method to the madness.
