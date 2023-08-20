use super::Symbol;

use hasan_hir as hir;

use std::collections::HashMap;
use anyhow::{Error, bail};

pub type SymbolTable = HashMap<String, Symbol>;
/// Contains a "generics map" for a given symbol
pub type GenericTable = HashMap<Symbol, Vec<Symbol>>;

// TODO: One possible way of implementing the generics table:
//
// Given this scope structure:
//   A
//  / \
// B  C
//
// From scope `A`, analyze both `B` and `C` scopes, and clone their generic tables.
// Next, search for symbols that exist both in `A` and the underlying scope.
// If such shared symbols exist, propagate the updated generic table to scope `A`.
// Given that there may be conflicts between `B` and `C`, we only want to
// add non-existing generic table entries, rather than overwriting existing ones.

#[derive(Debug, Clone)]
pub struct Scope {
	pub symbol_table: SymbolTable,
	pub generic_table: GenericTable,
	pub flags: ScopeFlags
}

impl Scope {
	pub fn new() -> Self {
		Self {
			symbol_table: HashMap::new(),
			generic_table: HashMap::new(),
			flags: ScopeFlags::default()
		}
	}

	/// Creates a child scope while keeping all of the variables in scope
	pub fn create_child_scope(&self) -> Self {
		Self {
			symbol_table: self.symbol_table.clone(),
			generic_table: HashMap::new(), // Create a new generic table for easier merging
			flags: self.flags
		} 
	}

	/// Inserts a symbol into the symbol table. Panics if a symbol with the provided name already exists
	pub fn insert_symbol(&mut self, name: String, symbol: Symbol) -> Result<(), Error> {
		if self.symbol_table.insert(name.clone(), symbol).is_some() {
			bail!("Cannot overwrite symbol with name `{}`", name);
		}

		Ok(())
	}

	/// Gets a symbol from the symbol table. Panics if a symbol does not exist
	pub fn get_symbol(&self, name: &str) -> Result<Symbol, Error> {
		if let Some(symbol) = self.symbol_table.get(name) {
			return Ok(symbol.to_owned());
		}

		bail!("Symbol with name `{}` does not exist", name);
	}

	pub fn update_symbol(&mut self, name: String, symbol: Symbol) -> Result<(), Error> {
		if self.symbol_table.insert(name.clone(), symbol).is_none() {
			bail!("Symbol with name `{}` does not exist", name);
		}

		Ok(())
	}
}

impl hir::HirDiagnostics for Scope {
	fn info_string(&self) -> String {
		use indent::{indent_all_by, indent_all_with};
		use hasan_parser::NUM_SPACES;

		let symbol_table = self
			.symbol_table
			.values()
			.map(|symbol| symbol.info_string())
			.collect::<Vec<_>>()
			.join("\n\n");

		let generic_table = self
			.generic_table
			.iter()
			.map(|(symbol, table)| {
				let mut table_str = String::new();

				for table_entry in table {
					table_str.push_str(&table_entry.info_string());
				}

				format!(
					"{}:\n{}",
					symbol.name(),
					table_str
				)
			})
			.collect::<Vec<_>>()
			.join("\n");

		let flags = self.flags.info_string();

		let scope_info = indent_all_by(NUM_SPACES, format!(
			"Scope Info:\n{}",

			indent_all_by(hasan_parser::NUM_SPACES, format!(
				"Symbol Table:\n{}\n\nGeneric Table:\n{}\n\nFlags:\n{}",

				indent_all_by(NUM_SPACES, symbol_table),
				indent_all_by(NUM_SPACES, generic_table),
				indent_all_by(NUM_SPACES, indent_all_with("- ", flags))
			))
		));

		format!("/*\n{}\n*/", scope_info)
	}
}

impl Default for Scope {
	fn default() -> Self {
		Self::new()
	}
}

//-----------------------------------------------------------------//

#[derive(Debug, Clone, Copy)]
pub struct ScopeFlags {
	pub global: bool,

	pub in_function: bool,
	pub in_loop: bool,
	pub in_class: bool
}

impl ScopeFlags {
	pub fn as_string_vec(&self) -> Vec<String> {
		let mut flags = vec![];

		if self.global {
			flags.push("global".to_owned());
		}

		if self.in_function {
			flags.push("in_function".to_owned());
		}

		if self.in_loop {
			flags.push("in_loop".to_owned());
		}

		flags
	}

	pub fn info_string(&self) -> String {
		self.as_string_vec().join(",\n")
	}
}

impl Default for ScopeFlags {
	fn default() -> Self {
		Self {
			global: true,

			in_function: false,
			in_loop: false,
			in_class: false
		}
	}
}

impl ToString for ScopeFlags {
	fn to_string(&self) -> String {
		format!("ScopeFlags({})", self.as_string_vec().join(", "))
	}
}