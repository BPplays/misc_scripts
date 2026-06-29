use std::collections::HashMap;

use tabled::{
    settings::{
        object::Rows,
        object::Columns,
        style::Style,
        Color,
        Modify,
    },
    Table, Tabled,
};

#[derive(Debug)]
struct Tape {
    name: &'static str,
    capacity_tb: f64,
    tape_price_usd: f64,
    drive_key: &'static str,
}

impl Tape {
    fn media_price_per_tb(&self) -> f64 {
        self.tape_price_usd / self.capacity_tb
    }

    fn tapes_needed(&self, required_tb: f64) -> u32 {
        (required_tb / self.capacity_tb).ceil() as u32
    }

    fn total_cost(&self, required_tb: f64, drive_prices: &HashMap<&str, f64>) -> f64 {
        let tapes = self.tapes_needed(required_tb) as f64;
        drive_prices[self.drive_key] + tapes * self.tape_price_usd
    }

    fn effective_price_per_tb(
        &self,
        required_tb: f64,
        drive_prices: &HashMap<&str, f64>,
    ) -> f64 {
        self.total_cost(required_tb, drive_prices) / required_tb
    }
}

#[derive(Tabled)]
struct Row {
    #[tabled(rename = "Tape")]
    tape: String,

    #[tabled(rename = "Drive")]
    drive: String,

    #[tabled(rename = "TB/Tape")]
    tb_per_tape: String,

    #[tabled(rename = "Media $")]
    media_price: String,

    #[tabled(rename = "$/TB Media")]
    media_price_per_tb: String,

    #[tabled(rename = "Tapes")]
    tapes: u32,

    #[tabled(rename = "Total $")]
    total_cost: String,

    #[tabled(rename = "$/TB")]
    effective_price_per_tb: String,
}

fn main() {
    let drive_prices = HashMap::from([
        ("lto-10", 13_000.0),
        ("lto-9", 4_500.0),
        ("lto-8", 2_500.0),
        ("lto-7", 650.0),
        ("lto-6", 425.0),
    ]);

    let tapes = vec![
        Tape {
            name: "LTO-10 40TB",
            capacity_tb: 40.0,
            tape_price_usd: 500.0,
            drive_key: "lto-10",
        },
        Tape {
            name: "LTO-10 30TB",
            capacity_tb: 30.0,
            tape_price_usd: 287.0,
            drive_key: "lto-10",
        },
        Tape {
            name: "LTO-9 18TB",
            capacity_tb: 18.0,
            tape_price_usd: 99.0,
            drive_key: "lto-9",
        },
        Tape {
            name: "LTO-8 12TB",
            capacity_tb: 12.0,
            tape_price_usd: 65.0,
            drive_key: "lto-8",
        },
        Tape {
            name: "LTO-7 6TB",
            capacity_tb: 6.0,
            tape_price_usd: 58.0,
            drive_key: "lto-7",
        },
        Tape {
            name: "LTO-6 2.5TB",
            capacity_tb: 2.5,
            tape_price_usd: 30.25,
            drive_key: "lto-6",
        },
    ];

    let required_tb = 500.0;

    let mut ranked: Vec<&Tape> = tapes.iter().collect();

    ranked.sort_by(|a, b| {
        a.effective_price_per_tb(required_tb, &drive_prices)
            .partial_cmp(&b.effective_price_per_tb(required_tb, &drive_prices))
            .unwrap()
    });

    let rows: Vec<Row> = ranked
        .iter()
        .map(|tape| {
            let total = tape.total_cost(required_tb, &drive_prices);

            Row {
                tape: tape.name.to_string(),
                drive: tape.drive_key.to_string(),
                tb_per_tape: format!("{:.1}", tape.capacity_tb),
                media_price: format!("${:.2}", tape.tape_price_usd),
                media_price_per_tb: format!("${:.2}", tape.media_price_per_tb()),
                tapes: tape.tapes_needed(required_tb),
                total_cost: format!("${:.2}", total),
                effective_price_per_tb: format!("${:.2}", total / required_tb),
            }
        })
        .collect();


    println!("Storage required: {:.1} TB\n", required_tb);

    let mut table = Table::new(rows);

    table
        .with(Style::rounded())
        .with(
            Modify::new(Columns::one(0))
            .with(Color::FG_BRIGHT_GREEN),
        )

        .with(
            Modify::new(Columns::one(6))
            .with(Color::FG_BRIGHT_YELLOW),
        )

        .with(
            Modify::new(Columns::one(7))
            .with(Color::FG_BRIGHT_MAGENTA),
        )
        .with(
            Modify::new(Rows::first())
                .with(Color::FG_BLUE),
        );

    println!("{table}");
}
