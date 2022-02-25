use advent_of_code_2021::*;
use anyhow::anyhow;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opts {
    part: String,
}

fn main() -> anyhow::Result<()> {
    let opts = Opts::from_args();
    match opts.part.as_str() {
        "1-1" => println!("{}", p1_1()),
        "1-2" => println!("{}", p1_2()),
        "2-1" => println!("{}", p2_1()),
        "2-2" => println!("{}", p2_2()),
        "3-1" => println!("{}", p3_1()),
        "3-2" => println!("{}", p3_2()),
        "4-1" => println!("{}", p4_1()),
        "4-2" => println!("{}", p4_2()),
        _ => return Err(anyhow!("Unknown puzzle")),
    }
    Ok(())
}
