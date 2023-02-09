fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let input = include_str!("input.txt");
    println!("{}",input);
    Ok(())
}