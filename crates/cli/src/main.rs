fn main() {
    // let cli = Command::new("asma")
    //     .bin_name("asma")
    //     .about("asma workflow engine command line interface")
    //     .subcommand_required(true)
    //     .subcommand(Command::new("workflow").about("run workflow").alias("wf"))
    //     .subcommand(Command::new("list").about("list running").alias("ls"));
    //
    for item in 0..=10 {
        println!("item: {}", item);
    }

    let result = add_two_numbers(2, 2);
    println!("res {}", result);

    let x = if true { true } else { false };

    let y = match true {
        true => true,
        false => false,
    };
}


/// add two integers
///
/// # params
/// - `x` first integer
/// - `y` second integer
fn add_two_numbers(x: u32, y: u32) -> u32 {
    x + y
}