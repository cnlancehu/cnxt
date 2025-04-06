use cnxt::Colorize;

fn main() {
    #[cfg(windows)]
    cnxt::control::set_virtual_terminal(true);

    let print_src = |s: &str| {
        println!(
            "\n{}{}{}",
            r#"> println!("{}", "#,
            s.bright_yellow(),
            r#");"#
        );
    };
    println!(
        "\n> {}{}",
        r#"use cnxt::Colorize as _;"#.bright_magenta(),
        " # Remember to add this".bright_black()
    );

    print_src(r#""Cyan color".cyan()"#);
    println!("  {}", "Cyan color".cyan());

    print_src(r#""Bright yellow color".bright_yellow()"#);
    println!("  {}", "Bright yellow color".bright_yellow());

    print_src(
        r#""Later colors override earlier ones".yellow().bright_magenta()"#,
    );
    println!(
        "  {}",
        "Later colors override earlier ones"
            .yellow()
            .bright_magenta()
    );

    print_src(r#""Truecolor".truecolor(250, 179, 135)"#);
    println!("  {}", "Truecolor".truecolor(250, 179, 135));

    print_src(r#""Background Truecolor".on_truecolor(137, 180, 250)"#);
    println!("  {}", "Background Truecolor".on_truecolor(137, 180, 250));

    print_src(r#""Truecolor represent by hexcolor".hexcolor("cba6f7")"#);
    println!("  {}", "Truecolor represent by hexcolor".hexcolor("cba6f7"));

    print_src(
        r#""Bright Yellow on Bright Blue".on_bright_blue().bright_yellow()"#,
    );
    println!(
        "  {}",
        "Bright Yellow on Bright Blue"
            .on_bright_blue()
            .bright_yellow()
    );

    print_src(r#""Bold text".bold()"#);
    println!("  {}", "Bold text".bold());

    print_src(
        r#"format!(
                     "{} {} {}",
                     "Multiple".cyan(),
                     "formats".italic().yellow(),
                     "combined".bright_magenta().bold()
                 )"#,
    );
    println!(
        "  {}",
        format!(
            "{} {} {}",
            "Multiple".cyan(),
            "formats".italic().yellow(),
            "combined".bright_magenta().bold()
        )
    );

    print_src(r#""Reset style to default".red().bold().clear()"#);
    println!("  {}", "Reset style to default".red().bold().clear());

    print_src(r#"String::from("String is also supported").green().bold()"#);
    println!(
        "  {}",
        String::from("String is also supported").green().bold()
    );
}
