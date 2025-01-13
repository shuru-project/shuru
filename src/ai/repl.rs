use reedline::{DefaultPrompt, Reedline, Signal};

pub fn create_repl() -> Result<(), shuru::ai::error::AiError> {
    let current_dir = std::env::current_dir()?;
    let dot_shuru_dir = current_dir.join(".shuru");

    if !dot_shuru_dir.exists() {
        std::fs::create_dir_all(&dot_shuru_dir).map_err(|e| {
            shuru::ai::error::AiError::IoError(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to create .shuru directory: {}", e),
            ))
        })?;
    }

    let index_file = dot_shuru_dir.join("./index_file");

    let mut indexer = shuru::ai::js_indexer::JsIndexer::new(index_file);
    indexer.scan_directory(&current_dir).unwrap();
    indexer
        .save_index()
        .map_err(|e| shuru::ai::error::AiError::JsIndexerError(e))?;

    let mut line_editor = Reedline::create();
    let prompt = DefaultPrompt::default();

    loop {
        let sig = line_editor.read_line(&prompt);
        match sig {
            Ok(Signal::Success(buffer)) => {
                let prompt = buffer.trim();

                if prompt.eq_ignore_ascii_case("/exit") || prompt.eq_ignore_ascii_case("/quit") {
                    println!("bye!");
                    return Ok(());
                }

                let results = indexer
                    .search(
                        &prompt
                            .split_whitespace()
                            .map(|p| p.to_string())
                            .collect::<Vec<_>>(),
                    )
                    .iter()
                    .map(|file_path| file_path.to_string_lossy())
                    .collect::<Vec<_>>()
                    .join("\n");

                println!("ai: {}", results);
            }
            Ok(Signal::CtrlD) | Ok(Signal::CtrlC) => {
                println!("\nbye!");
                return Ok(());
            }
            Err(e) => {
                return Err(shuru::ai::error::AiError::IoError(e));
            }
        }
    }
}
