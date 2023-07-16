use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::PathBuf;

use pest::Parser;

use hasan::{cli, tokenizer, parser};
use tokenizer::{HasanPestParser, Rule};
use parser::ASTParser;

//* Helper functions *//
fn read_file(path: &str) -> String {
	let file = File::open(path).expect(&format!("Failed to open file \"{}\" (read)", path));
	
	let mut reader = BufReader::new(file);
	let mut contents = String::new();
	
	reader.read_to_string(&mut contents).expect(&format!("Failed to read from file \"{}\"", path));
	contents.replace("\r\n", "\n")
}

fn write_file(path: &str, contents: String) {
	let mut file = File::create(path).expect(&format!("Failed to open file \"{}\" (write)", path));
	file.write_all(contents.as_bytes()).expect(&format!("Failed to write to file \"{}\"", path));
}

fn copy_file(source: &PathBuf, destination: &PathBuf) {
    // Copy file
    if let Err(e) = fs::copy(source, destination) {
        eprintln!("Failed to copy file from {} to {}: {}", source.display(), destination.display(), e);
        return;
    }
}
//* Helper functions *//

/// Subcommand to compile a file
fn compile(command: cli::CompileCommand) {
	let file_path = command.file_path;
	let debug = command.debug;
	
	fs::create_dir_all("./compiled").expect("Failed to create \"compiled\" directory");
	
	println!("Pest parsing...");
	
	let contents = read_file(&file_path);
	let result = HasanPestParser::parse(Rule::program, &contents);
	
	if let Err(e) = result {
		println!("{}", e);
		return;
	}
	
	let pairs = result.unwrap();
	
	if debug {
		println!("Parsed pairs ({}): {}", pairs.len(), pairs);
		println!();
	}
	
	write_file("./compiled/1_raw_ast.txt", format!("{:#?}", pairs));
	
	println!("AST parsing...");
	
	let ast_parser = ASTParser::new(pairs);
	let ast = ast_parser.parse();
	
	if debug {
		println!("Parsed AST ({}): {:?}", ast.len(), ast);
		println!();
	}
	
	write_file("./compiled/2_hasan_ast.txt", format!("{:#?}", ast));
}

fn test_create(command: cli::CreateTestCommand) {
    let name = command.name;

    // Construct source and destination file paths
    let source_path = PathBuf::from("./input.hsl");
    let mut destination_path = PathBuf::from("./tests/cases");
	
    destination_path.push(format!("{}.hsl", name));

    // Copy the input file
    copy_file(&source_path, &destination_path);

    // Compile the input file and update the output file
    let update_command = cli::UpdateTestCommand { name };
    test_update(update_command);
}

fn test_update(command: cli::UpdateTestCommand) {
    let name = command.name;

    // Compile the input file
    compile(cli::CompileCommand {
        file_path: "./input.hsl".to_owned(),
        debug: false
    });

    // Construct source and destination file paths
    let source_path = PathBuf::from("./compiled/2_hasan_ast.txt");
    let mut destination_path = PathBuf::from("./tests/outputs");
    destination_path.push(format!("{}.txt", name));

    // Copy the output file
    copy_file(&source_path, &destination_path);
}

fn test_delete(command: cli::DeleteTestCommand) {
	let name = command.name;

	// Construct file path
    let mut file_path = PathBuf::from("./tests/cases");
    file_path.push(format!("{}.hsl", name));

    // Delete file
    if let Err(e) = fs::remove_file(&file_path) {
        eprintln!("Failed to delete file {}: {}", file_path.display(), e);
        return;
    }

    // Construct file path
    let mut file_path = PathBuf::from("./tests/outputs");
    file_path.push(format!("{}.txt", name));

    // Delete file
    if let Err(e) = fs::remove_file(&file_path) {
        eprintln!("Failed to delete file {}: {}", file_path.display(), e);
        return;
    }
}

fn test_subcommand(subcommand: cli::TestSubcommand) {
	match subcommand.command {
		cli::TestCommand::Create(command) => test_create(command),
		cli::TestCommand::Update(command) => test_update(command),
		cli::TestCommand::Delete(command) => test_delete(command)
	}
}

fn main() {
	let args = cli::CLI::parse_custom();
	
	match args.subcommand {
		cli::CLISubcommand::Compile(command) => compile(command),
		cli::CLISubcommand::Test(command) => test_subcommand(command)
	}
}