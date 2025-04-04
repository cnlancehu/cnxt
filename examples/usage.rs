use cnxt::Colorize as _;

fn main() {
    #[cfg(windows)]
    let _ = cnxt::control::set_virtual_terminal(true);
    let print_src = |s: &str| {
        println!(
            "\n{}{}{}",
            r#"> println!("{}", "#,
            s.bright_yellow(),
            r#");"#
        );
    };

    print_src(r#""Blue".blue()"#);
    println!("  {}", "Blue".blue());

    print_src(r#""Red".red()"#);
    println!("  {}", "Red".red());

    print_src(r#""Red on Blue".red().on_blue()"#);
    println!("  {}", "Red on Blue".red().on_blue());

    print_src(r#""Red on Blue (reversed)".on_blue().red()"#);
    println!("  {}", "Red on Blue (reversed)".on_blue().red());

    print_src(r#""Truecolor".truecolor(0, 255, 136)"#);
    println!("  {}", "Truecolor".truecolor(0, 255, 136));

    print_src(r#""Background Truecolor".on_truecolor(135, 28, 167)"#);
    println!("  {}", "Background Truecolor".on_truecolor(135, 28, 167));

    print_src(r#""Truecolor from tuple".custom_color((0, 255, 136))"#);
    println!("  {}", "Truecolor from tuple".custom_color((0, 255, 136)));

    print_src(r#""Background Truecolor tuple".on_custom_color((0, 255, 136))"#);
    println!(
        "  {}",
        "Background Truecolor tuple".on_custom_color((0, 255, 136))
    );

    print_src(r#""Bright Red on Bright Blue".on_bright_blue().bright_red()"#);
    println!(
        "  {}",
        "Bright Red on Bright Blue".on_bright_blue().bright_red()
    );

    print_src(r#""Bold text".bold()"#);
    println!("  {}", "Bold text".bold());

    print_src(
        r#"format!("{} {} {}", "Multiple".cyan(), "formats".italic().yellow(), "combined".cyan())"#,
    );
    println!(
        "  {}",
        format!(
            "{} {} {}",
            "Multiple".cyan(),
            "formats".italic().yellow(),
            "combined".cyan()
        )
    );

    print_src(r#""Color overriding (Red)".yellow().blue().red()"#);
    println!("  {}", "Color overriding (Red)".yellow().blue().red());

    print_src(r#""Default style (cleared)".red().bold().clear()"#);
    println!("  {}", "Default style (cleared)".red().bold().clear());

    print_src(r#""Purple = Magenta".purple().magenta()"#);
    println!("  {}", "Purple = Magenta".purple().magenta());

    print_src(r#""Normal = Clear".normal().clear()"#);
    println!("  {}", "Normal = Clear".normal().clear());

    print_src(r#"String::from("Green Bold").green().bold()"#);
    println!("  {}", String::from("Green Bold").green().bold());

    print_src(r#"format!("{:30}", "Blue padded".blue())"#);
    println!("  {}", format!("{:30}", "Blue padded".blue()));

    print_src(r#"format!("{:.3}", "Green truncated".green())"#);
    println!("  {}", format!("{:.3}", "Green truncated".green()));
}
