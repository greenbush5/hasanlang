mod classes;
mod conditionals;
mod enums;
mod functions;

pub use classes::*;
pub use conditionals::*;
pub use enums::*;
pub use functions::*;

pub use super::{Statement, TypeRef, Type, HIRCodegen};
use hasan_parser::vec_transform_str;

#[derive(Debug, Clone)]
pub struct Program {
	pub statements: Vec<Statement>,
	pub module_info: Option<ModuleInfo>,
	pub imports: Vec<ModuleInfo>
}

impl HIRCodegen for Program {
	fn codegen(&self) -> String {
		let statements = vec_transform_str(&self.statements, |statement| statement.codegen(), "\n");

		if let Some(info) = self.module_info.clone() {
			format!("{}\n{}", info.codegen(), statements)
		} else {
			statements
		}
	}
}

impl ToString for Program {
	fn to_string(&self) -> String {
		self.codegen()
	}
}

#[derive(Debug, Clone)]
pub struct ModuleInfo {
	pub name: String,
	pub path: Vec<String>
}

impl HIRCodegen for ModuleInfo {
	fn codegen(&self) -> String {
		if self.path.is_empty() {
			format!("module {}", self.name)
		} else {
			format!("module {}.{}", self.path.join("."), self.name)
		}
	}
}

impl ToString for ModuleInfo {
	fn to_string(&self) -> String {
		self.codegen()
	}
}

#[derive(Debug, Clone)]
pub struct Variable {
	pub name: String,
	pub kind: Type,
	pub value: hasan_parser::Expression
}