// weight_convert.rs - Конвертер весов на Rust (CLI)
use clap::{Arg, App};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::str::FromStr;

lazy_static::lazy_static! {
    static ref UNITS: HashMap<&'static str, f64> = {
        let mut m = HashMap::new();
        m.insert("mg", 0.001);
        m.insert("g", 1.0);
        m.insert("kg", 1000.0);
        m.insert("cwt", 100000.0);
        m.insert("t", 1000000.0);
        m.insert("oz", 28.349523125);
        m.insert("lb", 453.59237);
        m.insert("st", 6350.29318);
        m.insert("us_t", 907184.74);
        m.insert("ct", 0.2);
        m.insert("gr", 0.06479891);
        m.insert("amu", 1.66054e-24);
        m
    };
    static ref UNIT_NAMES: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("mg", "Миллиграмм");
        m.insert("g", "Грамм");
        m.insert("kg", "Килограмм");
        m.insert("cwt", "Центнер (метрический)");
        m.insert("t", "Тонна (метрическая)");
        m.insert("oz", "Унция (авердюпуа)");
        m.insert("lb", "Фунт (авердюпуа)");
        m.insert("st", "Стоун (британский)");
        m.insert("us_t", "Американская тонна");
        m.insert("ct", "Карат (метрический)");
        m.insert("gr", "Гран");
        m.insert("amu", "Атомная единица массы");
        m
    };
}

fn convert(value: f64, from: &str, to: &str) -> f64 {
    if from == to { return value; }
    let grams = value * UNITS[from];
    grams / UNITS[to]
}

fn main() {
    let matches = App::new("Weight Converter")
        .arg(Arg::with_name("value").short("v").long("value").takes_value(true))
        .arg(Arg::with_name("from").short("f").long("from").default_value("kg"))
        .arg(Arg::with_name("to").short("t").long("to").default_value("lb"))
        .arg(Arg::with_name("precision").short("p").long("precision").default_value("2"))
        .arg(Arg::with_name("batch").short("b").long("batch").takes_value(true))
        .arg(Arg::with_name("output").short("o").long("output").takes_value(true))
        .arg(Arg::with_name("list").long("list"))
        .arg(Arg::with_name("range").long("range").takes_value(true))
        .get_matches();

    if matches.is_present("list") {
        println!("Доступные единицы:");
        for (code, name) in UNIT_NAMES.iter() {
            println!("  {}: {}", code, name);
        }
        return;
    }

    let precision: usize = matches.value_of("precision").unwrap().parse().unwrap_or(2);
    let from = matches.value_of("from").unwrap();
    let to = matches.value_of("to").unwrap();

    if let Some(range) = matches.value_of("range") {
        let parts: Vec<&str> = range.split(',').collect();
        if parts.len() != 3 { eprintln!("Формат: start,end,step"); return; }
        let start: f64 = parts[0].parse().unwrap();
        let end: f64 = parts[1].parse().unwrap();
        let step: f64 = parts[2].parse().unwrap();
        if step <= 0.0 { eprintln!("Шаг должен быть положительным"); return; }
        let mut rows = Vec::new();
        let mut v = start;
        while v <= end + 1e-9 {
            let res = convert(v, from, to);
            rows.push(format!("{:.prec$} {} = {:.prec$} {}", v, UNIT_NAMES[from], res, UNIT_NAMES[to], prec=precision));
            v += step;
        }
        if let Some(out) = matches.value_of("output") {
            let content = rows.join("\n");
            std::fs::write(out, content).unwrap();
            println!("Таблица сохранена в {}", out);
        } else {
            println!("Таблица {} -> {}:", UNIT_NAMES[from], UNIT_NAMES[to]);
            for row in rows { println!("{}", row); }
        }
        return;
    }

    if let Some(batch_file) = matches.value_of("batch") {
        let file = File::open(batch_file).expect("Не удалось открыть файл");
        let reader = io::BufReader::new(file);
        let mut values = Vec::new();
        for line in reader.lines() {
            let line = line.unwrap();
            if line.trim().is_empty() { continue; }
            values.push(line.trim().parse::<f64>().unwrap());
        }
        let results: Vec<f64> = values.iter().map(|&v| convert(v, from, to)).collect();
        if let Some(out) = matches.value_of("output") {
            let lines: Vec<String> = values.iter().zip(results.iter())
                .map(|(v, r)| format!("{:.prec$} -> {:.prec$}", v, r, prec=precision))
                .collect();
            std::fs::write(out, lines.join("\n")).unwrap();
            println!("Результаты сохранены в {}", out);
        } else {
            for (v, r) in values.iter().zip(results.iter()) {
                println!("{:.prec$} -> {:.prec$}", v, r, prec=precision);
            }
        }
        return;
    }

    if let Some(val_str) = matches.value_of("value") {
        let value: f64 = val_str.parse().unwrap();
        let res = convert(value, from, to);
        println!("{:.prec$} {} = {:.prec$} {}", value, UNIT_NAMES[from], res, UNIT_NAMES[to], prec=precision);
    } else {
        eprintln!("Укажите --value, --batch, --range или --list");
    }
}
