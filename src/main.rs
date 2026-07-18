use anyhow::{Context, Result};
use clap::Parser;
use colored::Colorize;

#[derive(Parser)]
struct Options {
    #[clap(default_value = "Meow!")]
    message: String, // [1]
    #[clap(short='d', long="dead")]
    ///Make the cat appear dead
    dead: bool,
    #[clap(short='f', long="file")]
    ///Load the cat picture from the file
    catfile: Option<std::path::PathBuf>,
}

fn main() -> Result<()> {
    let options = Options::parse();
    let message = options.message;
    let eye = if options.dead { "x" } else { "o" };

    match &options.catfile {
        Some(path) => {
            let cat_template = std::fs::read_to_string(path).with_context(
                || format!("Could not read the file {:?}", path)
            )?;
            let eye = format!("{}", eye.red().bold());
            let cat_picture = cat_template.replace("{eye}", &eye);
            println!("{}", message.bright_black().underline().on_white());
            println!("{}", &cat_picture);
        },
        None => {
            if message.to_lowercase() == "woof" {
                eprintln!("The cat shouldn't bark like a dog.");
            }

            println!("{}", message.black().underline().on_white());
            println!("  \\");
            println!("   \\");
            println!("      /\\_/\\");
            println!("     ( {eye} {eye} )", eye=eye.red().bold());
            println!("     =( I )=");
        }
    }
    Ok(())
}