use linemux::MuxedLines;

async fn watch_file(path: &str) -> std::io::Result<()> {
    let mut lines = MuxedLines::new()?;

    // Register some files to be tailed, whether they currently exist or not.
    lines.add_file(path).await?;

    // Wait for `Line` event, which contains the line captured for a given
    // source path.
    while let Ok(Some(line)) = lines.next_line().await {
        println!("source: {}, line: {}", line.source().display(), line.line());
    }
    Ok(())
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let target_file = std::env::args().nth(1).expect("missing target file");

    watch_file(&target_file).await
}
