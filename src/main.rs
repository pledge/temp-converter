use clap::Parser;
use colored::*;
use std::io::{self, Read};
use temp_converter::{Scale, Temperature};
use atty::Stream;

const LOGO: &str = r#"
 _______                        ________
|       \                      /        |
| $$$$\ ______   __    __  | $$$$|
| $$__/  \      \ |  \  /  | | $__
| $$    \  $$$$\| $ /  | | $  
| $$$   /      $| $<   | | $$$
| $__/  |  $$$$| $ \  | | $__/
| $    |  $    $| $  \ | | $
 \$$$$ \$$$$ \$   \| \$$$$
"#;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, before_help = LOGO)]
struct Args {
    /// The temperature value to convert
    #[arg(required = false)]
    value: Option<f64>,

    /// The scale of the input temperature
    #[arg(required = false)]
    scale: Option<Scale>,

    /// The target scale to convert to
    #[arg(short, long)]
    to: Option<Scale>,
}

fn main() {
    let args = Args::parse();

    let input_temp = if let (Some(value), Some(scale)) = (args.value, args.scale) {
        Temperature::new(value, scale)
    } else if !atty::is(Stream::Stdin) {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer).expect("Failed to read from stdin.");
        let value: f64 = buffer.trim().parse().expect("Stdin must be a valid number.");
        Temperature::new(value, Scale::Celsius)
    } else {
        eprintln!("{}", "Error: No input provided. Please provide a value and scale as arguments, or pipe a value to the program.".red());
        std::process::exit(1);
    };

    println!("{}", format!("Input: {}", input_temp).bold());

    if let Some(target_scale) = args.to {
        let converted_temp = match target_scale {
            Scale::Celsius => input_temp.to_celsius(),
            Scale::Fahrenheit => input_temp.to_fahrenheit(),
            Scale::Kelvin => input_temp.to_kelvin(),
            Scale::Rankine => input_temp.to_rankine(),
        };
        println!("{}", format!("Converted: {}", converted_temp).green());
    } else {
        println!("{}", "Converted:".green());
        let all_scales = vec![Scale::Celsius, Scale::Fahrenheit, Scale::Kelvin, Scale::Rankine];
        for scale in all_scales {
            if scale != input_temp.scale {
                let converted_temp = match scale {
                    Scale::Celsius => input_temp.to_celsius(),
                    Scale::Fahrenheit => input_temp.to_fahrenheit(),
                    Scale::Kelvin => input_temp.to_kelvin(),
                    Scale::Rankine => input_temp.to_rankine(),
                };
                println!("  - {}", format!("{}: {}", scale, converted_temp).cyan());
            }
        }
    }
}
