# MESSY NOTES

## Parsing

- String to number: `<string slice>.parse::<u64>().unwrap()`
  - `parse` uses trait `FromStr` function `from_str()` that can be overridden
- Define a struct to use `struct Pos {x: i64, y: i64}`

## Processing

```rust
input
  .split("\n\n")  // Split on two newlines
  .map(|batch| {  // For each section
    batch
      .lines()
      .map(|line| line.parse::<u64>().unwrap())  // For each line in section
      .sum::<u64>()
  })
  .max()  // For each section get max
  .unwrap()

  // End could be this
  .sorted() // from itertools
  .rev()
  .take(3)
  .sum()
```

- `itertools`
- `regex`
  - Can use named capture groups to later refernce
- `nom` - https://tfpk.github.io/nominomicon/chapter_1.html

```rust
  let re = Regex::new(r"<SOME REGEX> x=(?P<x1>[-]?\d+), .....").unwrap();
  let caputures = re.captures(line).unwrap();
  let my_point = Pos {
    x: captures["x1"].parse().unwdrap(),
    y: captures["y1"].parse().unwdrap(),
  }
```

```rust
  fn from_str(s: &str) -> Result<Self, Self::Err> {...}
```

Debugging iterators:

```rust
    .inspect(|x| println!("Value: {:?}", x))
    .map(|x| dbg!(x))
```

## Review

- Option
- Result
- HashMap
- Vec
- `itertools`
- `regex`
- `nom` - https://tfpk.github.io/nominomicon/chapter_1.html
- Review of 2023 - https://www.youtube.com/playlist?list=PLWtPciJ1UMuD3_8Pb-EqrFhkYpastR2cn
