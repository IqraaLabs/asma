use clap::Command;

fn main() {
    let cli = Command::new("asma")
        .bin_name("asma")
        .about("asma workflow engine command line interface")
        .subcommand_required(true)
        .subcommand(Command::new("workflow").about("run workflow").alias("wf"))
        .subcommand(Command::new("list").about("list running").alias("ls"));

    let matches = cli.get_matches();
}
