# 🌡️ Temperature Converter

A simple command-line temperature converter written in Rust. Enter a temperature in Celsius, Fahrenheit, or Kelvin and instantly get the converted values in the other two units.

## Features

- Convert from **Celsius** → Fahrenheit & Kelvin
- Convert from **Fahrenheit** → Celsius & Kelvin
- Convert from **Kelvin** → Celsius & Fahrenheit
- Interactive CLI prompts

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.56+)

### Installation

```bash
git clone https://github.com/Pandaaa-C/temp_convert.git
cd temp_convert
```

### Run

```bash
cargo run
```

## Usage

When prompted, enter the unit of your input temperature:

```
Hello, welcome to the Temperature Converter!
Do you want to enter your Temperature in (C)elcius, (F)ahrenheit or (K)elvin?
> C
Enter the temperature in Celcius:
> 100
The temperature in Fahrenheit is: 212
The temperature in Kelvin is: 373.15
```

| Input | Output |
|-------|--------|
| `C`   | Fahrenheit & Kelvin |
| `F`   | Celsius & Kelvin |
| `K`   | Celsius & Fahrenheit |

## Conversion Formulas

| Conversion | Formula |
|---|---|
| Celsius → Fahrenheit | `(C × 9/5) + 32` |
| Fahrenheit → Celsius | `(F − 32) × 5/9` |
| Celsius → Kelvin | `C + 273.15` |
| Kelvin → Celsius | `K − 273.15` |

## License

This project is open source and available under the [MIT License](LICENSE).
