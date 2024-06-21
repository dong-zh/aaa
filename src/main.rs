use anyhow::{Context, Result};
use rodio::{Decoder, Source};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "assets"]
struct Assets;

fn main() -> Result<()> {
    let file_names = std::env::args().skip(1);
    if file_names.len() == 0 {
        eprint!("This program checks for C++ Core Guidelines ES.30, ES.31, and ES.32 violations and enforces them according to the its suggestions. Make sure to turn your volume up!\n");
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
    let mut violation_found = false;

    for (line_number, line) in std::io::BufRead::lines(reader).enumerate() {
        let line = line.context("Failed to read line")?;
        if line.starts_with("#define ") {
            println!("{}: line {}: {}", file_name, line_number + 1, line);
            violation_found = true;
        }
    }

    if violation_found {
        println!(
            "{} violated the C++ Core Guidelines ES.30, ES.31, or ES.32!",
            file_name
        );
        play_scream().context("Failed to play scream")
    } else {
        println!(
            "{} didn't violate C++ Core Guidelines ES.30, ES.31, nor ES.32.",
            file_name
        );
        Ok(())
    }
}
