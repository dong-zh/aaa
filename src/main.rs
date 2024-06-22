use std::{io::BufRead, usize};

use anyhow::{Context, Result};
use rodio::{Decoder, Source};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "assets"]
struct Assets;

fn main() -> Result<()> {
    let file_names = std::env::args().skip(1);
    if file_names.len() == 0 {
        eprintln!("This program checks for C++ Core Guidelines ES.30, ES.31, and ES.32 violations and enforces them according to the its suggestions. Make sure to turn your volume up!\n");
        eprintln!(
            "Usage: {} <C++ source file> ...",
            std::env::args()
                .next()
                .context("Failed to get program name")?
        );
        std::process::exit(1);
    }

    for file_name in file_names {
        let file_name = std::path::PathBuf::from(file_name);
        check_file(&file_name).context("Failed to scan file")?;
    }

    Ok(())
}

/// Plays the embedded scream sound for a second.
fn play_scream() -> Result<()> {
    let file = Assets::get("scream.flac").context("Failed to get scream file")?;
    let cursor = std::io::Cursor::new(file.data);
    let decoder = Decoder::new_flac(cursor).context("Failed to create decoder")?;

    let (_output_stream, output_stream_handle) =
        rodio::OutputStream::try_default().context("Failed to create player")?;
    output_stream_handle
        .play_raw(decoder.convert_samples())
        .context("Failed to play")?;
    std::thread::sleep(std::time::Duration::from_secs(1));

    Ok(())
}

/// It just checks whether any lines start with `#define` and plays a scream lmao.
fn check_file(file: &std::path::PathBuf) -> Result<()> {
    let file_name = file.to_string_lossy();
    let reader = std::io::BufReader::new(std::fs::File::open(file)?);

    let violations: Vec<(usize, String)> = reader
        .lines()
        .enumerate()
        .filter_map(|(line_number, line)| {
            line.ok()
                .filter(|line| line.starts_with("#define "))
                .map(|line| (line_number, line))
        })
        .collect();

    if violations.len() > 0 {
        println!("Violation found in {}!", file_name);
        violations.iter().for_each(|(line_number, line)| {
            println!("\tLine {}: {}", line_number + 1, line);
        });
        play_scream().context("Failed to play scream")
    } else {
        println!("{} is OK.", file_name);
        Ok(())
    }
}
