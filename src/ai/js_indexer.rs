use ast::{Declaration, Program, Statement};
use bincode;
use ignore::Walk;
use oxc_allocator::Allocator;
use oxc_ast::*;
use oxc_parser::Parser;
use oxc_span::SourceType;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use std::io::Read as _;
use std::path::{Path, PathBuf};

#[derive(thiserror::Error, Debug)]
pub enum JsIndexerError {
    #[error("I/O Error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Bincode Error: {0}")]
    Bincode(#[from] bincode::Error),

    #[error("Walk Error: {0}")]
    WalkError(#[from] ignore::Error),
}

#[derive(Debug, Serialize, Deserialize)]
struct FunctionInfo {
    name: String,
    start_span: usize,
    end_span: usize,
}

#[derive(Debug, Serialize, Deserialize)]
struct ClassInfo {
    name: String,
    start_span: usize,
    end_span: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileDocument {
    path: PathBuf,
    functions: Vec<FunctionInfo>,
    classes: Vec<ClassInfo>,
}

pub struct JsIndexer {
    documents: Vec<FileDocument>,
    term_frequencies: HashMap<String, HashMap<PathBuf, u32>>,
    index_path: PathBuf,
    file_hashes: HashMap<PathBuf, String>,
}

impl JsIndexer {
    pub fn new(index_path: PathBuf) -> Self {
        JsIndexer {
            documents: Vec::new(),
            term_frequencies: HashMap::new(),
            index_path,
            file_hashes: HashMap::new(),
        }
    }

    fn calculate_file_hash(&self, path: &Path) -> Result<String, JsIndexerError> {
        let mut file = std::fs::File::open(path)?;
        let mut hasher = Sha256::new();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        hasher.update(&buffer);
        Ok(format!("{:x}", hasher.finalize()))
    }

    pub fn scan_directory(&mut self, dir_path: &Path) -> Result<(), JsIndexerError> {
        for result in Walk::new(dir_path) {
            match result {
                Ok(entry) => {
                    let path = entry.path();
                    if self.should_process_file(path) {
                        let current_hash = self.calculate_file_hash(path)?;
                        if let Some(stored_hash) = self.file_hashes.get(path) {
                            if stored_hash == &current_hash {
                                continue;
                            }
                        }
                        self.process_file(path, &current_hash)?;
                    }
                }
                Err(e) => eprintln!("Error walking directory: {}", e),
            }
        }
        Ok(())
    }

    fn should_process_file(&self, path: &Path) -> bool {
        if !path.is_file() {
            return false;
        }

        match path.extension().and_then(|ext| ext.to_str()) {
            Some("js") | Some("ts") => (),
            _ => return false,
        }

        let ignored_dirs = ["node_modules", "dist", "build", ".git"];
        !path.components().any(|comp| {
            comp.as_os_str()
                .to_str()
                .map(|s| ignored_dirs.contains(&s))
                .unwrap_or(false)
        })
    }

    fn process_file(&mut self, path: &Path, current_hash: &str) -> Result<(), JsIndexerError> {
        let content = fs::read_to_string(path)?;

        let allocator = Allocator::default();
        let source_type = SourceType::default().with_module(true);
        let parser = Parser::new(&allocator, &content, source_type);

        let program = parser.parse().program;

        let (functions, classes) = self.extract_info(&program);

        let document = FileDocument {
            path: path.to_path_buf(),
            functions,
            classes,
        };

        self.update_term_frequencies(&document);
        self.documents.push(document);
        self.file_hashes
            .insert(path.to_path_buf(), current_hash.to_string());

        Ok(())
    }

    fn extract_info(&self, program: &Program) -> (Vec<FunctionInfo>, Vec<ClassInfo>) {
        let mut functions = Vec::new();
        let mut classes = Vec::new();

        for statement in &program.body {
            let Statement::Declaration(declaration) = statement else {
                continue;
            };

            match declaration {
                Declaration::FunctionDeclaration(func_decl) => {
                    if let Some(id) = &func_decl.id {
                        functions.push(FunctionInfo {
                            name: id.name.to_string(),
                            start_span: func_decl.span.start as usize,
                            end_span: func_decl.span.end as usize,
                        });
                    }
                }
                Declaration::ClassDeclaration(class_decl) => {
                    if let Some(id) = &class_decl.id {
                        classes.push(ClassInfo {
                            name: id.name.to_string(),
                            start_span: class_decl.span.start as usize,
                            end_span: class_decl.span.end as usize,
                        });
                    }
                }
                Declaration::TSImportEqualsDeclaration(import_delc) => {
                    dbg!(&import_delc.id.name);
                }
                _ => {}
            }
        }

        (functions, classes)
    }

    fn update_term_frequencies(&mut self, document: &FileDocument) {
        // Update with function names
        for function in &document.functions {
            let term_freq = self
                .term_frequencies
                .entry(function.name.clone())
                .or_insert_with(HashMap::new);
            *term_freq.entry(document.path.clone()).or_insert(0) += 1;
        }
        // Update with class names
        for class in &document.classes {
            let term_freq = self
                .term_frequencies
                .entry(class.name.clone())
                .or_insert_with(HashMap::new);
            *term_freq.entry(document.path.clone()).or_insert(0) += 1;
        }
    }

    pub fn save_index(&self) -> Result<(), JsIndexerError> {
        let serialized = bincode::serialize(&self.documents)?;
        fs::write(&self.index_path, serialized)?;
        Ok(())
    }

    pub fn load_index(&mut self) -> Result<(), JsIndexerError> {
        let data = fs::read(&self.index_path)?;
        self.documents = bincode::deserialize(&data)?;
        Ok(())
    }

    pub fn search(&self, query_terms: &[String]) -> Vec<PathBuf> {
        let mut results: Vec<PathBuf> = self
            .documents
            .iter()
            .filter(|doc| {
                query_terms.iter().any(|term| {
                    doc.functions.iter().any(|f| f.name == *term)
                        || doc.classes.iter().any(|c| c.name == *term)
                })
            })
            .map(|doc| doc.path.clone())
            .collect();

        results.sort();
        results
    }
}
