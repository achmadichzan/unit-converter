mod cli;
mod units;
mod history;

use clap::Parser;
use cli::{Cli, Commands};
use units::Unit;
use history::{load_history, save_to_history};
use std::str::FromStr;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Convert(args) => {
            let from_unit = Unit::from_str(&args.from)
                .map_err(|unit_name| format!("[ERROR] Satuan asal '{}' tidak dikenali.", unit_name))?;

            let to_unit = Unit::from_str(&args.to)
                .map_err(|unit_name| format!("[ERROR] Satuan tujuan '{}' tidak dikenali.", unit_name))?;

            let value = args.value;

            let from_category = from_unit.category();
            let to_category = to_unit.category();

            if from_category != to_category {
                return Err(
                    format!(
                        "[ERROR] Tidak dapat mengonversi satuan yang berbeda kategori: [{:?}] {} → [{:?}] {}",
                        from_category, from_unit, to_category, to_unit
                    ).to_lowercase().into()
                );
            }

            let from_symbol = match from_unit {
                Unit::Celsius => "°C",
                Unit::Fahrenheit => "°F",
                Unit::Kelvin => "K",
                Unit::Cm => "cm",
                Unit::Inch => "inch",
                Unit::Km => "km",
                Unit::Miles => "miles",
            };

            let (result_value, to_symbol) = match (from_unit, to_unit) {
                (Unit::Celsius, Unit::Fahrenheit) => (value * 9.0 / 5.0 + 32.0, "°F"),
                (Unit::Celsius, Unit::Kelvin) => (value + 273.15, "K"),
                (Unit::Celsius, Unit::Celsius) => (value, "°C"),

                (Unit::Fahrenheit, Unit::Celsius) => ((value - 32.0) * 5.0 / 9.0, "°C"),
                (Unit::Fahrenheit, Unit::Kelvin) => ((value - 32.0) * 5.0 / 9.0 + 273.15, "K"),
                (Unit::Fahrenheit, Unit::Fahrenheit) => (value, "°F"),

                (Unit::Kelvin, Unit::Celsius) => (value - 273.15, "°C"),
                (Unit::Kelvin, Unit::Fahrenheit) => ((value - 273.15) * 9.0 / 5.0 + 32.0, "°F"),
                (Unit::Kelvin, Unit::Kelvin) => (value, "K"),

                (Unit::Cm, Unit::Inch) => (value / 2.54, "inch"),
                (Unit::Cm, Unit::Km) => (value / 100_000.0, "km"),
                (Unit::Cm, Unit::Miles) => (value / 160_934.0, "miles"),
                (Unit::Cm, Unit::Cm) => (value, "cm"),

                (Unit::Inch, Unit::Cm) => (value * 2.54, "cm"),
                (Unit::Inch, Unit::Km) => (value * 2.54 / 100_000.0, "km"),
                (Unit::Inch, Unit::Miles) => (value * 2.54 / 160_934.0, "miles"),
                (Unit::Inch, Unit::Inch) => (value, "inch"),

                (Unit::Km, Unit::Cm) => (value * 100_000.0, "cm"),
                (Unit::Km, Unit::Inch) => (value * 100_000.0 / 2.54, "inch"),
                (Unit::Km, Unit::Miles) => (value / 1.60934, "miles"),
                (Unit::Km, Unit::Km) => (value, "km"),

                (Unit::Miles, Unit::Cm) => (value * 160_934.0, "cm"),
                (Unit::Miles, Unit::Inch) => (value * 160_934.0 / 2.54, "inch"),
                (Unit::Miles, Unit::Km) => (value * 1.60934, "km"),
                (Unit::Miles, Unit::Miles) => (value, "miles"),

                _ => unreachable!("Kombinasi unit tidak valid, seharusnya sudah ditangani oleh pengecekan kategori."),
            };

            let value_str = if args.value.fract() == 0.0 {
                format!("{:.0}", args.value)
            } else {
                format!("{}", args.value)
            };

            let result_string = format!(
                "{} {} = {:.4} {}",
                value_str, from_symbol, result_value, to_symbol
            );

            println!("{}", result_string);

            save_to_history(result_string)?;
        }

        Commands::List => {
            println!("Satuan yang didukung:");
            println!("1. [suhu] celsius");
            println!("2. [suhu] fahrenheit");
            println!("3. [suhu] kelvin");
            println!("4. [panjang] cm");
            println!("5. [panjang] inch");
            println!("6. [panjang] km");
            println!("7. [panjang] miles");
        }

        Commands::History => {
            let history = load_history()?;

            if history.records.is_empty() {
                println!("Belum ada riwayat konversi.");
            } else {
                println!("Riwayat Konversi:");
                for (i, record) in history.records.iter().enumerate() {
                    println!("{}. {}", i + 1, record);
                }
            }
        }
    }

    Ok(())
}