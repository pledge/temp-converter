use clap::{Parser, CommandFactory};
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
#[command(author, version, about = "A CLI tool for converting between temperature scales (Celsius, Fahrenheit, Kelvin, Rankine). Run without arguments to see this help.", long_about = None, before_help = LOGO)]
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
        match io::stdin().read_to_string(&mut buffer) {
            Ok(0) => {
                // No data available, show help
                Args::command().print_help().unwrap();
                std::process::exit(0);
            }
            Ok(_) => {
                let trimmed = buffer.trim();
                if trimmed.is_empty() {
                    Args::command().print_help().unwrap();
                    std::process::exit(0);
                }
                let value: f64 = trimmed.parse().expect("Stdin must be a valid number.");
                Temperature::new(value, Scale::Celsius)
            }
            Err(_) => {
                Args::command().print_help().unwrap();
                std::process::exit(0);
            }
        }
    } else {
        Args::command().print_help().unwrap();
        std::process::exit(0);
    };

    process_temperature(input_temp, args.to);
}

fn process_temperature(input_temp: Temperature, target_scale: Option<Scale>) {
    println!("{}", format!("Input: {}", input_temp).bold());

    if let Some(target_scale) = target_scale {
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
