# Scalar Types
- 4 primary scalar types
- represents a single value.

## Integer types

| Length | Signed | Unsigned |
| ------ | ------ | -------- |
| 8 bit  | i8     | u8       |
| 16 bit | i16    | u16      |
| 32 bit | i32    | u32      |
| 64 bit | i64    | u64      |
| 128 bit| i128   | u128     |
| arch   | isize  | usize    |

### Integer Literals

- N.B. Each notation allows a type suffix (such as  'u8' in `71u8`).

| Number Literals | Example      |
| ----            | ----         |
| Decimal         | 98_322u32    |
| Hex             | 0x10i8       |
| Octal           | 0o77u16      |
| Binary          | 0b11_110_000 |
| Byte (u8 only)  | b'A'         |

- In Rust, integer overflow occurs a wrapping.

## Floating Point
- `f32` or `f64`

## Numeric Operations
- All basic operations between valus of the same type.

## Character
- `char`
- specified by single quotes `''`, whereas `str` is specified by double quotes `""`

## Boolean
- `bool`
- `true` or `false`


# Compound Types
