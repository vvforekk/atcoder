use chrono::Local;

fn main() -> anyhow::Result<()> {
    let contests = xtasks::get_upcoming_contests()?;

    println!("\nnow {}", Local::now().format("%Y-%m-%d %H:%M:%S"));
    xtasks::print_contests(&contests)
}
