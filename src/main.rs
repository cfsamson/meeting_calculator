use std::io::Read;
use std::fs::File;


fn main() -> Result<(), String>{
    let mut args = std::env::args();
    if args.len() < 3 {
        return Err("Du må spesifisere 2 argumenter slik: --add input.txt".to_string());
    }

    let ops = match args.nth(1).unwrap().as_str() {
        "--add" => Ops::Add,
        "--mul" => Ops::Mul,
        _ => return Err("Første argumentet må være --add eller --mul".to_string()),
    };
    let input_file = args.nth(0).expect("missing 2. argument");

    let mut file = File::open(&input_file)
    .map_err(|_| format!("Finner ikke filen: {}", input_file))?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).map_err(|e| e.to_string())?;
    let calculator = Calculator::new(ops);

    for line in buffer.lines() {
        if line.is_empty() {
            continue;
        }
        if let Some(c) = line.chars().nth(0) {
            if c == '#' {
                println!("\n{}:", &line[1..].trim());
                continue;
            }
        }
        let numbers = line.split(',').map(|n| n.trim().parse::<f64>().unwrap_or(0.0));
        calculator.calc(numbers);
    }

    Ok(())
}

pub struct Calculator {
    kind: Ops,
}

pub enum Ops {
    Add,
    Mul,
}

impl<'a> Calculator {
    pub fn new(kind: Ops) -> Self {
        Calculator {
            kind,
        }
    }

    pub fn calc(&self, numbers: impl Iterator<Item = f64>) {
        match self.kind {
            Ops::Add => Calculator::add(numbers),
            Ops::Mul => Calculator::mul(numbers),
        }
    }
    fn add(numbers: impl Iterator<Item = f64>) {
        let mut numbers = numbers;
        let first: f64 = numbers.next().unwrap() as f64;
        let mut s = format!("{}", first);
        let mut sum = first;
        for n in numbers {
            s += format!(" + {}", n).as_str();
            sum += n as f64;
        }
        println!("{} = {}", s, sum);
    }

     fn mul(numbers: impl Iterator<Item = f64>) {
        let mut numbers = numbers;
        let first: f64 = numbers.next().unwrap() as f64;
        let mut s = format!("{}", first);
        let mut sum = first;
        for n in numbers {
            s += format!(" * {}", n).as_str();
            sum *= n as f64;
        }
        println!("{} = {}", s, sum);
    }
}
