# standard_paper_sizes

The `standard_paper_sizes` crate provides an enum of different paper types across the world and their dimensions in millimeters.

This is particularly useful when generating documents (e.g. with [printpdf](https://github.com/fschutt/printpdf)) of exact sizes for printing.

This work is inspired by [papersizes.io](https://papersizes.io/).

# Features

`standard_paper_sizes` features 136 unique sizes paired with 10 common names for sizes (e.g. `UsLetter` for `AnsiA`).

<details>
<summary>6 US paper sizes plus three convenience aliases for ANSI sizes</summary>

- Letter (same as ANSI A)
- Legal
- Ledger (Same as ANSI B)
- Tabloid (Same as ANSI B)
- Executive
- Junior Legal
- Half Letter
- Government Letter
- Government Legal

</details>

<details>
<summary>32 US envelope sizes</summary>

- Usually expressed with mixed numbers, these are enumified as decimals
    - For example, 1 3/4 is expressed as `UsEnvelope1_75`
- 1
- 1 3/4
- 3
- 6
- 6 1/4
- 6 3/4
- 7
- 7 3/4 Monarch
- 8
- 8 5/8
- 9
- 9 3/4
- 10
- 10 1/2
- 11
- 12
- 12 1/2
- 13 1/2
- 14
- 14 1/2
- 15
- 15 1/2
- 16
- Envelope A1
- Envelope A2 Lady Grey
- Envelope A4
- Envelope A6 Thompsons
- Envelope A7 Besselheim
- Envelope A8 Carrs
- Envelope A9 Diplomat
- Envelope A10 Willow
- A Long

</details>

<details>
<summary>26 ISO 216 envelope sizes</summary>

- DL
- Envelope B4
- Envelope B5
- Envelope B6
- Envelope C3
- Envelope C4
- Envelope C6
- Envelope C64M
- Envelope C7
- Envelope CE4
- Envelope CE64
- Envelope E4
- Envelope EC45
- Envelope EC5
- Envelope E5
- Envelope E56
- Envelope E6
- Envelope E65
- Envelope X5
- Envelope EX5

</details>

<details>
<summary>8 international business card sizes</summary>

- US
- ISO 216
- European
- Scandinavian
- Chinese
- Japanese
- Iranian
- Hungarian

</details>

<details>
<summary>4 ID and credit card sizes (ISO-7810)</summary>

- ISO 7810 ID-1, with three common aliases:
    - `CR80`
    - `TD1`
    - `CreditCard`
- ISO 7810 ID-2, with a common alias
    - `Visa`
- ISO 7810 ID-3, with a common alias
    - `Passport`
- ISO 7810 ID-0000, with a common alias
    - `MiniSIM`
    - `Passport`

</details>

<details>
<summary>47 ISO 216 sizes</summary>

- A series
    - A0 through A13
    - 2A0
    - 4A0
    - A0+
    - A1+
    - A3+
- B series
    - B0 through B13
    - B0+
    - B1+
    - B2+
- C series
    - C0 through C10

</details>

<details>
<summary>5 NA ANSI sizes</summary>

- A-E

</details>

<details>
<summary>5 NA ARCH sizes</summary>

- A-E
- E1-E3

</details>

# Usage

Add to your project using `cargo add`:

```bash
$ cargo add standard_paper_sizes
```

Alternatively, you can add crate dependencies in your `Cargo.toml` directly:

```toml
[dependencies]
standard_paper_sizes = "1.0.0"
```

You can then use the `Type` enum and its method `size`:

```rust
use standard_paper_sizes::Type;

fn main() {
    println("{}", Type::UsLetter.size())
}

// Output: "216.0x279.0"
```

# License

GPLv3
