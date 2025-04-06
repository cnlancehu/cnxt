<div align="center">
    <img src="./assets/banner-cli.gif" alt="banner">
    <p>Run <code>cargo run --example banner -r</code> to print the Banner!</p>
</div>

# Colored Next (CNXT)
[![text](https://api.lance.fun/badge/cratesio/cnxt)](https://crates.io/crates/cnxt)

A fork of [colored](https://github.com/colored-rs/colored) which introduces better functionalities.

Why CNXT?

1. **Enhanced Performance**: Uses `Cow` to minimize allocations [(Inspired by this PR)](https://github.com/colored-rs/colored/pull/135)
2. **Streamlined Codebase**: Removed outdated and redundant code
3. **Superior Terminal Support**: Improved detection and handling of terminal capabilities

## Usage
Coloring your terminal made simple. You already know how to do it.

![usage](./assets/usage.png)

Small tips

1. For **Windows targets**, add this to enable colors in **Windows CMD**:
    ```rust
    #[cfg(windows)]
    cnxt::control::set_virtual_terminal(true);
    ```

    Comparison of colors with virtual terminal disabled vs enabled.

    ![comparison](./assets/set_virtual_terminal_comparison.png)

2. CNXT automatically detects **terminal color support** across **3 levels**:

    - `Ansi16`
    - `Ansi256`
    - `TrueColor`
     
    When using colors beyond your terminal's capabilities, CNXT automatically downgrades them to the maximum supported level.
    ```rust
    use cnxt::control::{set_should_colorize, ShouldColorize};

    // By default, the support level is detected from environment
    ShouldColorize::from_env()

    // You can explicitly set the support level:
    set_should_colorize(ShouldColorize::YesWithTrueColor);  // Enable colorization with true color support
    set_should_colorize(ShouldColorize::YesWithAnsi256);    // Enable colorization with 256 color support

    // Simple on/off control:
    set_should_colorize(ShouldColorize::No);    // Disable colorization
    set_should_colorize(ShouldColorize::Yes);   // Enable colorization

    // Reset to environment-based detection:
    set_should_colorize(ShouldColorize::from_env());
    ```

    And for manual color fallback control:
    ```rust
    use cnxt::Color;
    
    let color = Color::TrueColor {
        r: 166,
        g: 227,
        b: 161,
    };
    let ansi16_color = color.fallback_to_ansi16();
    # or
    let ansi256_color = color.fallback_to_ansi256();
    ```

## License
As a fork of [colored](https://github.com/colored-rs/colored), CNXT maintains the same [MPL-2.0](https://github.com/colored-rs/colored?tab=readme-ov-file#license) license as the original repository.
