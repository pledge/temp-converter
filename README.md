# Temperature Converter CLI

A simple and efficient command-line tool for converting temperatures between Celsius, Fahrenheit, Kelvin, and Rankine scales.

## Project Genesis

This project was bootstrapped entirely by the Gemini CLI. It was created iteratively through a conversation, starting with a high-level goal and progressively adding features and refinements.

The final prompt that encapsulates the entire feature set is included below:

```
I want to start a new project using Rust. It will be a command line tool that will convert between various temperature scales.

I want to include Celsius, Fahrenheit, Kelvin, and Rankine. When you input one, it should give the output in all the others, unless you specify a specific conversion target.

It should be a comprehensive CLI tool, with a help command, both short and long flags, and color output. The help screen should have a cool, text-based ASCII art logo for the words 'Temp Converter'.

We also need to have good validation of the inputs and give meaningful suggestions on how to correct invalid inputs.

The project should have a README on how to build and run it. There should be a test suite that includes unit tests and integration tests with high coverage that covers all the temperature scales, commands, and piping functionality that we support.

The tool should also support receiving a piped value. If a value is piped in, we will assume the scale is Celsius.

We want to keep dependencies to a minimum, but we should select a popular foundation for building a CLI tool.
```

## Features

- Convert from any of the four scales to the others.
- Specify a single target scale for conversion.
- Colored output for better readability.
- Helpful error messages for invalid input.

## Building

To build the project, you need to have Rust and Cargo installed. You can find instructions on how to install them on the [official Rust website](https://www.rust-lang.org/tools/install).

Once you have Rust installed, you can build the project by running the following command in the project's root directory:

```bash
cargo build --release
```

The compiled binary will be located at `target/release/temp-converter`.

## Testing

To run the test suite, use the following command:

```bash
cargo test
```

## Usage

To use the tool, run the binary with the value and scale you want to convert from. By default, it will output the conversion in all other available scales.

### Basic Conversion

```bash
./target/release/temp-converter <VALUE> <SCALE>
```

**Example:**

```bash
./target/release/temp-converter 100 celsius
```

This will output:

```
Input: 100.00 °C
Converted:
  - Fahrenheit: 212.00 °F
  - Kelvin: 373.15 °K
  - Rankine: 671.67 °R
```

### Specific Conversion

You can also specify a single target scale using the `--to` or `-t` flag.

```bash
./target/release/temp-converter <VALUE> <SCALE> --to <TARGET_SCALE>
```

**Example:**

```bash
./target/release/temp-converter 32 f -t celsius
```

This will output:

```
Input: 32.00 °F
Converted: 0.00 °C
```

'''### Piped Input

You can also pipe a value directly into the tool. When you do this, the input is assumed to be in **Celsius**.

**Example:**

```bash
echo "25" | ./target/release/temp-converter
```

This will output:

```
Input: 25.00 °C
Converted:
  - Fahrenheit: 77.00 °F
  - Kelvin: 298.15 °K
  - Rankine: 536.67 °R
```

### Supported Scales'''

The following scales are supported, with both full names and short forms:

- `celsius` / `c`
- `fahrenheit` / `f`
- `kelvin` / `k`
- `rankine` / `r`
