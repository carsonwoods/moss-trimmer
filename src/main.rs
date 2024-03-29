use std::fs;
use std::io;
use std::process::Command;

use clap::Parser;

/// Downloads MOSS results
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// URL of the MOSS results page
    url: String,

    /// Removes results not containing the specified string
    #[arg(short, long, value_name = "STRING")]
    trim_string: Option<String>,

    /// Disables download, so only trimming occurs
    #[arg(short, long, default_value_t = false)]
    skip_download: bool,
}

fn trim_results(content: &str, trim_string: &str) -> String {
    let mut result = String::new();
    let mut skip_row = false;
    let mut in_table = true; // Flag to track whether we are inside the table or not

    // Iterate through each line in the HTML content
    for line in content.lines() {
        if line.contains("<TR>") {
            // Check if this row contains the specified string
            skip_row = !line.contains(trim_string);
        }

        // Append the line to the result if it's not to be skipped and we're still inside the table
        if in_table {
            if !skip_row {
                result.push_str(line);
                result.push('\n');
            }
        } else {
            result.push_str(line);
            result.push('\n');
        }

        if line.contains("</TABLE>") {
            // Reached the end of the table, stop removing lines
            in_table = false;
            result.push_str(line);
            result.push('\n');
        }

        if line.contains("</TR>") {
            // Reset the skip flag at the end of the row
            skip_row = false;
        }
    }

    result
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    if !&args.skip_download {
        println!("Starting download (please be patient, this can take a while)...");

        let output = Command::new("wget")
            .arg("--recursive")
            .arg("--no-clobber")
            .arg("--page-requisites")
            .arg("--html-extension")
            .arg("--convert-links")
            .arg("--restrict-file-names=windows")
            .arg("--domains")
            .arg("moss.stanford.edu")
            .arg("--no-parent")
            .arg("-e")
            .arg("robots=off")
            .arg(&args.url)
            .output()
            .expect("Could not retrieve results");

        if output.status.success() {
            println!("Download successful");
        } else {
            eprintln!(
                "Download failed: {}",
                String::from_utf8_lossy(&output.stderr)
            );
            return Ok(());
        }
    }

    match args.trim_string {
        Some(trim_string) => {
            println!("Trimming...");

            // modifies the URL to better match a path on disk
            let trimmed_url = args.url.trim();
            let modified_url = if trimmed_url.starts_with("https://") {
                &trimmed_url[8..]
            } else if trimmed_url.starts_with("http://") {
                &trimmed_url[7..]
            } else {
                trimmed_url
            };

            let mut final_url = String::from(modified_url);

            // removes trailing "/" so that the submission
            // id can be extracted to create filename
            if final_url.ends_with('/') {
                final_url.pop();
            }
            final_url.push_str(".html");

            // Read input file
            let file_content = fs::read_to_string(&final_url)?;

            // Process content in memory
            let modified_content = trim_results(&file_content, &trim_string);

            // Write modified content back to the same file
            fs::write(&final_url, modified_content)?;

            println!("Trimming complete, see trimmed file: {}", final_url);
        }
        None => {
            return Ok(());
        }
    };

    Ok(())
}
