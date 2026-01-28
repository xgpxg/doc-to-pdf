use anyhow::anyhow;
use clap::Parser;
use doc_to_pdf::{convert, set_libreoffice_root_dir};
use rust_embed::Embed;
use std::path::PathBuf;
use std::{env, fs};

#[derive(Embed)]
#[folder = "resources/"]
struct Asset;

const LIBREOFFICE_RESOURCE: &str = "libreoffice.tar.gz";

#[derive(Parser)]
#[command(name = "doc-to-pdf", about = "Convert document files to PDF format")]
struct Args {
    /// Source file to convert, Support doc, docx, xls, xlsx, ppt, pptx, csv
    #[arg(short, long, value_parser)]
    input: PathBuf,

    /// Destination PDF file
    #[arg(short, long, value_parser)]
    output: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let temp_dir = env::temp_dir();
    let libreoffice_path = temp_dir.join("libreoffice");

    if !fs::exists(&libreoffice_path)? {
        let libreoffice = Asset::get(LIBREOFFICE_RESOURCE)
            .ok_or(anyhow!("{} not found", LIBREOFFICE_RESOURCE))?;
        fs::write(&temp_dir.join(LIBREOFFICE_RESOURCE), libreoffice.data)?;
        fs::create_dir_all(&libreoffice_path)?;
        std::process::Command::new("tar")
            .arg("-xzf")
            .arg(&temp_dir.join(LIBREOFFICE_RESOURCE))
            .arg("-C")
            .arg(&libreoffice_path)
            .output()?;
    }

    set_libreoffice_root_dir(&libreoffice_path.display().to_string())?;

    let args = Args::parse();

    println!("{} ===> {}", args.input.display(), args.output.display());
    convert(
        args.input.display().to_string().as_str(),
        args.output.display().to_string().as_str(),
    )?;
    println!("Ok");

    Ok(())
}
