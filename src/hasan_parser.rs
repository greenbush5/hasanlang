use std::iter::Peekable;

use pest::error::{Error, ErrorVariant};
use pest::iterators::{Pair, Pairs};
use pest::Span;

use strum_macros::Display;

use crate::pest_parser::Rule;

macro_rules! error {
	($msg:expr, $span:expr) => {
		panic!("{}", create_error($msg, $span))
	};

	($msg:expr, $span:expr, $($var_args:expr),*) => {
		panic!("{}", create_error(&format!($msg, $($var_args),*), $span))
	};
}

fn create_error<'p>(message: &str, span: Span<'p>) -> Error<Rule> {
	Error::new_from_span(
		ErrorVariant::CustomError { message: message.to_owned() },
		span
	)
}

pub struct HasanParser<'p> {
	pairs: Pairs<'p, Rule>
}

#[derive(Debug, Clone)]
pub struct Program {
	pub statements: Vec<Statement>,
	pub module_info: Option<ModuleInfo>
}

#[derive(Debug, Clone)]
pub struct ModuleInfo {
	pub name: String,
	pub path: Vec<String>
}

#[derive(Debug, Clone, Display)]
pub enum Statement {
	FunctionDefinition {
		modifiers: GeneralModifiers,

		name: String,
		generics: Vec<DefinitionType>,
		arguments: Vec<FunctionArgument>,
		return_type: Option<Type>,
		statements: Vec<Statement>
	},

	FunctionDeclaration {
		modifiers: GeneralModifiers,

		name: String,
		generics: Vec<DefinitionType>,
		arguments: Vec<FunctionArgument>,
		return_type: Option<Type>
	},

	TypeAlias {
		modifiers: GeneralModifiers,

		name: String,
		generics: Vec<DefinitionType>,
		definition: Type
	},

	ClassDefinition {
		modifiers: GeneralModifiers,

		name: String,
		generics: Vec<DefinitionType>,
		members: Vec<ClassDefinitionMember>
	},

	ClassDeclaration {
		modifiers: GeneralModifiers,

		name: String,
		generics: Vec<DefinitionType>,
		members: Vec<ClassDeclarationMember>
	},

	VariableDefinition {
		modifiers: GeneralModifiers,

		name: String,
		kind: Option<Type>,
		value: Expression
	},

	VariableAssign {
		name: Expression,
		value: Expression
	},

	FunctionCall {
		callee: Expression,
		generics: Vec<Type>,
		arguments: Vec<Expression>
	},

	Return(Expression),

	EnumDefinition {
		modifiers: GeneralModifiers,

		name: String,
		variants: Vec<EnumVariant>
	},

	If {
		condition: Expression,
		statements: Vec<Statement>,
		elseif_branches: Vec<ConditionBranch>,
		else_branch: Option<ConditionBranch>
	},

	While {
		condition: Expression,
		statements: Vec<Statement>
	},

	For {
		left: Expression,
		right: Expression,
		statements: Vec<Statement>
	},
	
	Break,

	Interface {
		modifiers: GeneralModifiers,

		name: String,
		generics: Vec<DefinitionType>,
		members: Vec<InterfaceMember>
	},

	InterfaceImpl {
		interface_name: String,
		generics: Vec<DefinitionType>,
		class_name: String,
		members: Vec<ClassDefinitionMember>
	},

	UseModule {
		path: Vec<String>,
		name: String
	},

	UseModuleAll {
		path: Vec<String>,
		name: String
	},

	UseModuleItems {
		path: Vec<String>,
		name: String,
		items: Vec<ModuleItem>
	},

	/// Special statement that is only intended for testing use
	Unimplemented
}

#[derive(Debug, Clone)]
pub enum ModuleItem {
	Regular(String),

	Renamed {
		from: String,
		to: String
	}
}

#[derive(Debug, Clone)]
pub enum InterfaceMember {
	Variable {
		modifiers: GeneralModifiers,

		name: String,
		kind: Type
	},

	Function {
		attributes: Option<ClassFunctionAttributes>,
		modifiers: GeneralModifiers,

		name: String,
		generics: Vec<DefinitionType>,
		argument_types: Vec<Type>,
		return_type: Type
	}
}

#[derive(Debug, Clone)]
pub struct ConditionBranch {
	pub condition: Expression,
	pub statements: Vec<Statement>
}

#[derive(Debug, Clone)]
pub struct EnumVariant {
	pub name: String
}

#[derive(Debug, Clone)]
pub struct FunctionArgument {
	pub name: String,
	pub kind: Type
}

impl FunctionArgument {
	pub fn new(name: String, kind: Type) -> Self {
		FunctionArgument { name, kind }
	}
}

pub type ClassFunctionAttributes = Vec<ClassFunctionAttribute>;

#[derive(Debug, Clone, PartialEq)]
pub enum ClassFunctionAttribute {
	Constructor,
	Get,
	Set
}

impl TryFrom<&str> for ClassFunctionAttribute {
	type Error = String;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		match value {
			"constructor" => Ok(Self::Constructor),
			"get" => Ok(Self::Get),
			"set" => Ok(Self::Set),

			_ => Err(format!("Unknown class function attribute '{}'", value))
		}
	}
}

pub type GeneralModifiers = Vec<GeneralModifier>;

#[derive(Debug, Clone, PartialEq)]
pub enum GeneralModifier {
	Public,
	Constant,
	Static
}

impl TryFrom<&str> for GeneralModifier {
	type Error = String;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		match value {
			"pub" => Ok(Self::Public),
			"const" => Ok(Self::Constant),
			"static" => Ok(Self::Static),

			_ => Err(format!("Unknown modifier '{}'", value))
		}
	}
}

#[derive(Debug, Clone)]
pub enum ClassDefinitionMember {
	Variable {
		modifiers: GeneralModifiers,

		name: String,
		kind: Type,
		default_value: Expression
	},

	Function {
		attributes: Option<ClassFunctionAttributes>,
		modifiers: GeneralModifiers,

		name: String,
		generics: Vec<DefinitionType>,
		arguments: Vec<FunctionArgument>,
		return_type: Option<Type>,
		statements: Vec<Statement>
	}
}

impl ClassDefinitionMember {
	pub fn function_from_statement(statement: Statement, attributes: Option<ClassFunctionAttributes>) -> Self {
		if let Statement::FunctionDefinition {
			modifiers,
			name,
			generics,
			arguments,
			return_type,
			statements
		} = statement {
			return ClassDefinitionMember::Function {
				modifiers,
				name,
				attributes,
				generics,
				arguments,
				return_type,
				statements
			};
		} else {
			panic!("Failed to convert invalid statement into a ClassDefinitionMember::Function");
		}
	}
}

#[derive(Debug, Clone)]
pub enum ClassDeclarationMember {
	Variable {
		modifiers: GeneralModifiers,

		name: String,
		kind: Type
	},

	Function {
		attributes: Option<ClassFunctionAttributes>,
		modifiers: GeneralModifiers,

		name: String,
		generics: Vec<DefinitionType>,
		arguments: Vec<FunctionArgument>,
		return_type: Option<Type>
	}
}

impl ClassDeclarationMember {
	pub fn function_from_statement(statement: Statement, attributes: Option<ClassFunctionAttributes>) -> Self {
		if let Statement::FunctionDeclaration {
			modifiers,
			name,
			generics,
			arguments,
			return_type
		} = statement {
			return ClassDeclarationMember::Function {
				attributes,
				modifiers,
				
				name,
				generics,
				arguments,
				return_type
			};
		} else {
			panic!("Failed to convert invalid statement into a ClassDefinitionMember::Function");
		}
	}
}

type IntType = i64;
type FloatType = f64;

#[derive(Debug, Clone)]
pub enum Type {
	Regular {
		base: Box<Expression>,
		generics: Vec<DefinitionType>,

		// Type attributes
		raw: bool,
		array: bool
	},

	Function {
		argument_types: Vec<Type>,
		return_type: Box<Type>
	}
}

#[derive(Debug, Clone, PartialEq)]
pub struct DefinitionType {
	pub name: String,
	pub requires_implementations: Vec<String>
}

#[derive(Debug, Clone, Display)]
pub enum Expression {
	Int(IntType),
	Float(FloatType),
	String(String),
	Boolean(bool),

	Unary {
		operator: UnaryOperator,
		operand: Box<Expression>
	},

	Binary {
		lhs: Box<Expression>,
		operator: BinaryOperator,
		rhs: Box<Expression>
	},

	FunctionCall {
		callee: Box<Expression>,
		generics: Vec<Type>,
		arguments: Vec<Expression>
	},

	ArrayAccess {
		expression: Box<Expression>,
		accessor: Box<Expression>
	},

	DotAccess {
		expression: Box<Expression>,
		accessor: Box<Expression>
	},

	ArrowAccess {
		expression: Box<Expression>,
		accessor: Box<Expression>
	},

	Array(Vec<Expression>),
	Identifier(String),

	Type(Type),

	TypeCast {
		value: Box<Expression>,
		kind: Box<Type>
	},

	AnonymousFunction {
		generics: Vec<DefinitionType>,
		arguments: Vec<FunctionArgument>,
		return_type: Box<Option<Type>>,
		statements: Vec<Statement>
	},

	Empty,
	Unimplemented
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
	Plus,
	Minus,
	Divide,
	Times,
	Modulo,
	Equals,
	NotEquals,
	And,
	Or,
	GreaterThan,
	LessThan,
	GreaterThanEqual,
	LessThanEqual
}

impl BinaryOperator {
	pub fn as_str(&self) -> &'static str {
		match self {
			Self::Plus => "+",
			Self::Minus => "-",
			Self::Divide => "/",
			Self::Times => "*",
			Self::Modulo => "%",
			Self::Equals => "==",
			Self::NotEquals => "!=",
			Self::And => "and",
			Self::Or => "or",
			Self::GreaterThan => ">",
			Self::LessThan => "<",
			Self::GreaterThanEqual => ">=",
			Self::LessThanEqual => "<="
		}
	}
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperator {
	Minus,
	Not
}

impl UnaryOperator {
	pub fn as_str(&self) -> &'static str {
		match self {
			Self::Minus => "-",
			Self::Not => "not"
		}
	}
}

impl<'p> HasanParser<'p> {
	pub fn new(pairs: Pairs<'p, Rule>) -> Self {
		HasanParser { pairs }
	}

	pub fn parse(&self) -> Program {
		let mut statements: Option<Vec<Statement>> = None;
		let mut module_info: Option<ModuleInfo> = None;

		for pair in self.pairs.clone() {
			match pair.as_rule() {
				Rule::COMMENT |
				Rule::WHITESPACE |
				Rule::line_comment |
				Rule::block_comment => (),

				Rule::program => {
					// Check for module marker
					let mut pairs = pair.into_inner();
					let first_pair = pairs.peek();

					if first_pair.is_some() {
						let first_pair = first_pair.unwrap();

						if first_pair.as_rule() == Rule::module_declaration_marker {
							module_info = Some(self.parse_module_marker(first_pair));
							pairs.next();
						}

						// Actually parse the statements
						statements = Some(self.parse_statements(pairs));
					}
				},

				rule => error!("expected '{:?}', got '{:?}'", pair.as_span(), Rule::program, rule)
			}
		}

		let statements = statements.unwrap_or_else(|| unreachable!("Failed to parse file: program rule is missing"));
		Program { statements, module_info }
	}

	fn parse_module_path(&self, pair: Pair<Rule>) -> Vec<String> {
		if pair.as_rule() != Rule::module_path {
			error!("expected '{:?}', got '{:?}'", pair.as_span(), Rule::module_path, pair.as_rule());
		}

		let mut pairs = pair.into_inner();
		let mut path: Vec<String> = Vec::new();

		while let Some(pair) = pairs.next() {
			path.push(self.pair_str(pair));
		}

		path
	}

	fn parse_module_marker(&self, pair: Pair<Rule>) -> ModuleInfo {
		if pair.as_rule() != Rule::module_declaration_marker {
			error!("expected '{:?}', got '{:?}'", pair.as_span(), Rule::module_declaration_marker, pair.as_rule());
		}

		let mut pairs = pair.into_inner();

		let mut next_pair = pairs
			.next()
			.expect("Failed to parse module marker: path/module name pair is missing");

		let mut path: Vec<String> = Vec::new();

		if next_pair.as_rule() == Rule::module_path {
			path = self.parse_module_path(next_pair);

			next_pair = pairs
				.next()
				.expect("Failed to parse module marker: module name pair is missing");
		}

		let name = self.pair_str(next_pair);

		ModuleInfo { name, path }
	}

	fn parse_statements(&self, pairs: Pairs<Rule>) -> Vec<Statement> {
		let mut statements: Vec<Statement> = Vec::new();

		for pair in pairs {
			if pair.as_rule() == Rule::EOI {
				continue;
			}

			let statement = match pair.as_rule() {
				Rule::module_use_stmt => self.parse_module_use(pair),
				Rule::module_use_all_stmt => self.parse_module_use_all(pair),
				Rule::module_use_items_stmt => self.parse_module_use_items(pair),

				Rule::function_definition_stmt => self.parse_function_definition(pair),
				Rule::function_declaration_stmt => self.parse_function_declaration(pair),
				Rule::type_alias_stmt => self.parse_type_alias(pair),
				Rule::class_definition => self.parse_class_definition(pair),
				Rule::class_declaration => self.parse_class_declaration(pair),
				Rule::variable_definition_stmt => self.parse_variable_definition(pair),
				Rule::variable_assign_stmt => self.parse_variable_assign(pair),
				Rule::function_call_stmt => self.parse_function_call(pair),
				Rule::return_stmt => self.parse_return(pair),
				Rule::enum_definition_stmt => self.parse_enum_definition(pair),

				Rule::interface_stmt => self.parse_interface(pair),
				Rule::interface_impl_stmt => self.parse_interface_impl(pair),

				Rule::if_stmt => self.parse_if(pair),
				Rule::while_stmt => self.parse_while(pair),
				Rule::break_stmt => Statement::Break,
				Rule::for_in_stmt => self.parse_for_in(pair),

				rule => error!("unexpected statement '{:?}'", pair.as_span(), rule)
			};

			statements.push(statement);
		}

		statements
	}

	pub fn pair_str(&self, pair: Pair<Rule>) -> String {
		pair.as_str().to_owned()
	}

	fn parse_operator(&self, pair: &Pair<Rule>) -> BinaryOperator {
		match pair.as_str() {
			"+" => BinaryOperator::Plus,
			"-" => BinaryOperator::Minus,
			"/" => BinaryOperator::Divide,
			"*" => BinaryOperator::Times,
			"%" => BinaryOperator::Modulo,
			"==" => BinaryOperator::Equals,
			"!=" => BinaryOperator::NotEquals,
			"and" => BinaryOperator::And,
			"or" => BinaryOperator::Or,
			">" => BinaryOperator::GreaterThan,
			"<" => BinaryOperator::LessThan,
			">=" => BinaryOperator::GreaterThanEqual,
			"<=" => BinaryOperator::LessThanEqual,

			operator => error!("expected '+', '-', '/', '*', '%', '==', '!=', 'and', 'or', '>', '<', '>=' or '<=', got '{}'", pair.as_span(), operator)
		}
	}

	fn parse_unary_operator(&self, pair: &Pair<Rule>) -> UnaryOperator {
		match pair.as_str() {
			"-" => UnaryOperator::Minus,
			"not" => UnaryOperator::Not,

			operator => error!("expected '-' or 'not', got '{}'", pair.as_span(), operator)
		}
	}

	fn is_term(&self, pair: Pair<Rule>) -> bool {
		matches!(
			pair.as_rule(),
			Rule::anonymous_function
		)
	}

	fn parse_expression(&self, expression_pair: Pair<Rule>) -> Expression {
		let mut pairs = expression_pair
			.clone()
			.into_inner()
			.peekable();

		// Check if an iterator is empty
		if pairs.len() < 1 || self.is_term(expression_pair.clone()) {
			return self.parse_term(expression_pair);
		}

		if expression_pair.as_rule() == Rule::recursive_expression {
			return self.parse_recursive_expression(expression_pair.into_inner());
		}

		self.parse_expression_with_precedence(&mut pairs, 0)
	}

	fn parse_expression_with_precedence(&self, pairs: &mut Peekable<Pairs<Rule>>, precedence: u8) -> Expression {
		if pairs.len() < 1 {
			unreachable!("Failed to parse expression: pairs are empty");
		}

		let left_pair = pairs
			.next()
			.unwrap_or_else(|| unreachable!("Failed to parse expression: pairs are empty"));
		
		let mut left = self.parse_term(left_pair);
	
		while let Some(pair) = pairs.peek() {
			if pair.as_rule() == Rule::binary_operator {
				let operator_precedence = self.get_operator_precedence(pair);
				
				if operator_precedence < precedence {
					break;
				}
	
				let operator = self.parse_operator(pair);
				
				// Consume the operator
				pairs.next();

				let right = self.parse_expression_with_precedence(pairs, operator_precedence + 1);
	
				left = Expression::Binary {
					lhs: Box::new(left),
					operator,
					rhs: Box::new(right)
				};
			} else {
				break;
			}
		}
	
		left
	}

	fn get_operator_precedence(&self, pair: &Pair<Rule>) -> u8 {
		match pair.as_str() {
			"==" | "!=" | "and" | "or" | ">" | "<" | ">=" | "<=" => 1,
			"+" | "-" => 2,
			"*" | "/" | "%" => 3,
			
			operator => error!("expected '+', '-', '/', '*', '%', '==', '!=', 'and', or 'or', got '{}'", pair.as_span(), operator)
		}
	}

	fn parse_identifier(&self, pair: Pair<Rule>) -> Expression {
		if pair.as_rule() != Rule::identifier {
			error!("expected '{:?}', got '{:?}'", pair.as_span(), Rule::identifier, pair.as_rule());
		}

		Expression::Identifier(self.pair_str(pair))
	}

	fn parse_number_literal(&self, pair: Pair<Rule>) -> Expression {
		let string = pair.as_str().to_owned();

		let literal = match string.parse::<IntType>() {
			Ok(i) => Expression::Int(i),
			Err(_) => match string.parse::<FloatType>() {
				Ok(f) => Expression::Float(f),
				Err(_) => error!("failed to parse number literal '{}'", pair.as_span(), string),
			},
		};

		literal
	}

	fn parse_string_literal(&self, pair: Pair<Rule>) -> Expression {
		let literal = pair.as_str().to_owned();
		let clean_literal = literal.trim_start_matches(&['\'', '\"'][..]).trim_end_matches(&['\'', '\"'][..]);

		Expression::String(clean_literal.to_owned())
	}

	fn parse_boolean_literal(&self, pair: Pair<Rule>) -> Expression {
		let literal = pair.as_str();

		match literal {
			"true" => Expression::Boolean(true),
			"false" => Expression::Boolean(false),

			_ => error!("expected 'true' or 'false', got '{}'", pair.as_span(), literal)
		}
	}

	fn parse_term(&self, pair: Pair<Rule>) -> Expression {
		match pair.as_rule() {
			Rule::anonymous_function => self.parse_anonymous_function(pair),
			Rule::unary_expression => self.parse_unary_expression(pair),
			Rule::binary_expression | Rule::expression => self.parse_expression(pair),

			Rule::array_expression => self.parse_array_expression(pair),
			Rule::recursive_expression => self.parse_recursive_expression(pair.into_inner()),

			Rule::number_literal |
			Rule::string_literal |
			Rule::boolean_literal => self.parse_literal(pair),

			Rule::identifier => self.parse_identifier(pair),
			Rule::r#type => Expression::Type(self.parse_type(pair)),

			rule => error!("invalid expression rule '{:?}'", pair.as_span(), rule)
		}
	}

	fn parse_module_use(&self, pair: Pair<Rule>) -> Statement {
		if pair.as_rule() != Rule::module_use_stmt {
			error!("expected '{:?}', got '{:?}'", pair.as_span(), Rule::module_use_stmt, pair.as_rule());
		}

		let mut pairs = pair.into_inner();

		let mut next_pair = pairs
			.next()
			.expect("Failed to parse use module statement: path/module name pair is missing");

		let mut path: Vec<String> = Vec::new();

		if next_pair.as_rule() == Rule::module_path {
			path = self.parse_module_path(next_pair);

			next_pair = pairs
				.next()
				.expect("Failed to parse use module statement: module name pair is missing");
		}

		let name = self.pair_str(next_pair);
		
		Statement::UseModule { path, name }
	}

	fn parse_module_use_all(&self, pair: Pair<Rule>) -> Statement {
		if pair.as_rule() != Rule::module_use_all_stmt {
			error!("expected '{:?}', got '{:?}'", pair.as_span(), Rule::module_use_all_stmt, pair.as_rule());
		}

		let mut pairs = pair.into_inner();

		let mut next_pair = pairs
			.next()
			.expect("Failed to parse use module statement: path/module name pair is missing");

		let mut path: Vec<String> = Vec::new();

		if next_pair.as_rule() == Rule::module_path {
			path = self.parse_module_path(next_pair);

			next_pair = pairs
				.next()
				.expect("Failed to parse use module statement: module name pair is missing");
		}

		let name = self.pair_str(next_pair);
		
		Statement::UseModuleAll { path, name }
	}

	fn parse_module_use_items(&self, pair: Pair<Rule>) -> Statement {
		if pair.as_rule() != Rule::module_use_items_stmt {
			error!("expected '{:?}', got '{:?}'", pair.as_span(), Rule::module_use_items_stmt, pair.as_rule());
		}

		let mut pairs = pair.into_inner();

		let mut next_pair = pairs
			.next()
			.expect("Failed to parse use module statement: path/module name pair is missing");

		let mut path: Vec<String> = Vec::new();

		if next_pair.as_rule() == Rule::module_path {
			path = self.parse_module_path(next_pair);

			next_pair = pairs
				.next()
				.expect("Failed to parse use module statement: module name pair is missing");
		}
		
		let name = self.pair_str(next_pair);

		let mut items_pair = pairs
			.next()
			.expect("Failed to parse use module statement: items pair is missing")
			.into_inner();

		let mut items: Vec<ModuleItem> = Vec::new();
		
		while let Some(item_pair) = items_pair.next() {
			items.push(self.parse_module_item(item_pair));
		}
		
		Statement::UseModuleItems { path, name, items }
	}

	fn parse_module_item(&self, pair: Pair<Rule>) -> ModuleItem {
		if !matches!(pair.as_rule(), Rule::module_item_rename | Rule::module_item_regular ) {
			error!(
				"expected '{:?}' or '{:?}', got '{:?}'",
				pair.as_span(),
				
				Rule::module_item_rename,
				Rule::module_item_regular,
				pair.as_rule()
			);
		}

		let as_rule = pair.as_rule();

		let mut pairs = pair.into_inner();

		let name = self.pair_str(
			pairs
				.next()
				.expect("Failed to parse module import item: name pair is missing")
		);

		match as_rule {
			Rule::module_item_rename => {
				let new_name = self.pair_str(
					pairs
						.next()
						.expect("Failed to parse module import item: new name pair is missing")
				);

				ModuleItem::Renamed {
					from: name,
					to: new_name
				}
			},

			Rule::module_item_regular => ModuleItem::Regular(name),

			rule => unreachable!("Failed to parse module import item: got unexpected rule '{:?}'", rule)
		}
	}

	fn parse_interface_variable(&self, pair: Pair<Rule>) -> InterfaceMember {
		if pair.as_rule() != Rule::interface_variable {
			error!("expected '{:?}', got '{:?}'", pair.as_span(), Rule::interface_variable, pair.as_rule());
		}

		let mut pairs = pair.into_inner();

		let modifiers_pair = pairs
			.next()
			.expect("Failed to parse interface variable: modifiers pair is missing");

		let modifiers = self.parse_general_modifiers(modifiers_pair);

		let name = self.pair_str(
			pairs
				.next()
				.expect("Failed to parse interface variable: name pair is missing")
		);

		let type_pair = pairs
			.next()
			.expect("Failed to parse interface variable: type pair is missing");

		InterfaceMember::Variable {
			modifiers,
			name,
			kind: self.parse_type(type_pair)
		}
	}

	fn parse_interface_function_arguments(&self, pair: Pair<Rule>) -> Vec<Type> {
		if pair.as_rule() != Rule::interface_function_arguments {
			error!("expected '{:?}', got '{:?}'", pair.as_span(), Rule::interface_function_arguments, pair.as_rule());
		}

		let mut pairs = pair.into_inner();
		let mut argument_types: Vec<Type> = Vec::new();

		while let Some(pair) = pairs.next() {
			if pair.as_rule() != Rule::r#type {
				error!("expected '{:?}', got '{:?}'", pair.as_span(), Rule::r#type, pair.as_rule());
			}
			
			argument_types.push(self.parse_type(pair));
		}

		argument_types
	}

	fn parse_interface_function(&self, pair: Pair<Rule>) -> InterfaceMember {
		if pair.as_rule() != Rule::interface_function {
			error!("expected '{:?}', got '{:?}'", pair.as_span(), Rule::interface_function, pair.as_rule());
		}

		let mut pairs = pair.into_inner();

		// Next pair can either be attributes or modifiers
		let mut next_pair = pairs
			.next()
			.expect("Failed to parse interface function: attributes/modifiers pair is missing");

		let mut attributes: Option<ClassFunctionAttributes> = None;
		
		if next_pair.as_rule() == Rule::attributes {
			attributes = Some(self.parse_class_function_attributes(next_pair));

			// Next pair is guaranteed to be `general_modifiers`, even if it's empty
			next_pair = pairs
				.next()
				.unwrap_or_else(|| unreachable!("Failed to parse interface function: modifiers pair is missing"));
		}

		// Parse modifiers
		let modifiers = self.parse_general_modifiers(next_pair);

		// Get function name
		let name = self.pair_str(
			pairs
				.next()
				.expect("Failed to parse interface variable: name pair is missing")
		);

		// Next pair can be generics, arguments or return_type
		let mut next_pair = pairs
			.next()
			.expect("Failed to parse interface function: generics/arguments/return type pair is missing");

		let mut generics: Vec<DefinitionType> = Vec::new();

		// If it's generics, parse and go to the next pair
		if next_pair.as_rule() == Rule::definition_generics {
			generics = self.parse_generics_as_definition_types(next_pair);

			next_pair = pairs
				.next()
				.unwrap_or_else(|| unreachable!("Failed to parse interface function: arguments/return type pair is missing"));
		}

		let mut arguments: Vec<Type> = Vec::new();

		// If it's arguments, parse them as types and skip to the return type
		if next_pair.as_rule() == Rule::interface_function_arguments {
			arguments = self.parse_interface_function_arguments(next_pair);

			next_pair = pairs
				.next()
				.unwrap_or_else(|| unreachable!("Failed to parse interface function: return type pair is missing"));
		}

		// Make sure that the last rule is a type (return type)
		if next_pair.as_rule() != Rule::r#type {
			error!("expected '{:?}', got '{:?}'", next_pair.as_span(), Rule::r#type, next_pair.as_rule());
		}

		// Parse the return type
		let return_type = self.parse_type(next_pair);

		InterfaceMember::Function {
			modifiers,
			attributes,

			name,
			generics,
			argument_types: arguments,
			return_type
		}
	}

	fn parse_interface_members(&self, pair: Pair<Rule>) -> Vec<InterfaceMember> {
		if pair.as_rule() != Rule::interface_members {
			error!("expected '{:?}', got '{:?}'", pair.as_span(), Rule::interface_members, pair.as_rule());
		}

		// Not needed yet
		// let span = pair.as_span();
		let mut pairs = pair.into_inner();

		let mut members = Vec::new();

		while let Some(pair) = pairs.next() {
			let member = match pair.as_rule() {
				Rule::interface_variable => self.parse_interface_variable(pair),
				Rule::interface_function => self.parse_interface_function(pair),

				rule => error!(
					"expected '{:?}' or '{:?}', got '{:?}'",
					pair.as_span(),
					
					Rule::interface_variable,
					Rule::interface_function,
					rule
				)
			};

			members.push(member);
		}

		members
	}

	fn parse_interface(&self, pair: Pair<Rule>) -> Statement {
		if pair.as_rule() != Rule::interface_stmt {
			error!("expected '{:?}', got '{:?}'", pair.as_span(), Rule::interface_stmt, pair.as_rule());
		}

		let mut pairs = pair.into_inner();

		let modifiers_pair = pairs
			.next()
			.expect("Failed to parse interface statement: modifiers pair is missing");

		let modifiers = self.parse_general_modifiers(modifiers_pair);

		let name = self.pair_str(
			pairs
				.next()
				.expect("Failed to parse interface statement: name pair is missing")
		);

		let mut next_pair = pairs
			.next()
			.expect("Failed to parse interface statement: generics/members pair is missing");

		let mut generics: Vec<DefinitionType> = Vec::new();

		if next_pair.as_rule() == Rule::definition_generics {
			generics = self.parse_generics_as_definition_types(next_pair);

			// Next pair is guaranteed to be `interface_members`, even if there are no members
			next_pair = pairs
				.next()
				.unwrap_or_else(|| unreachable!("Failed to parse interface statement: members pair is missing"));
		}

		let members = self.parse_interface_members(next_pair);

		Statement::Interface { modifiers, name, generics, members }
	}

	fn parse_interface_impl(&self, pair: Pair<Rule>) -> Statement {
		if pair.as_rule() != Rule::interface_impl_stmt {
			error!("expected '{:?}', got '{:?}'", pair.as_span(), Rule::interface_impl_stmt, pair.as_rule());
		}

		let mut pairs = pair.into_inner();

		let interface_name = self.pair_str(
			pairs
				.next()
				.expect("Failed to parse interface implementation statement: interface name pair is missing")
		);

		let mut next_pair = pairs
			.next()
			.expect("Failed to parse interface implementation statement: generics/class name pair is missing");

		let mut generics: Vec<DefinitionType> = Vec::new();

		if next_pair.as_rule() == Rule::definition_generics {
			generics = self.parse_generics_as_definition_types(next_pair);

			next_pair = pairs
				.next()
				.expect("Failed to parse interface implementation statement: class name pair is missing");
		}

		let class_name = self.pair_str(next_pair);
		let mut members: Vec<ClassDefinitionMember> = Vec::new();

		while let Some(pair) = pairs.next() {
			if pair.as_rule() != Rule::class_definition_member {
				error!("expected '{:?}', got '{:?}'", pair.as_span(), Rule::class_definition_member, pair.as_rule());
			}

			members.push(self.parse_class_definition_member(pair));
		}

		Statement::InterfaceImpl {
			interface_name,
			generics,
			class_name,
			members
		}
	}

	fn parse_anonymous_function(&self, pair: Pair<Rule>) -> Expression {
		if pair.as_rule() != Rule::anonymous_function {
			error!("expected '{:?}', got '{:?}'", pair.as_span(), Rule::anonymous_function, pair.as_rule());
		}

		let mut pairs = pair.into_inner();

		let mut generics: Vec<DefinitionType> = Vec::new();
		let mut arguments: Vec<FunctionArgument> = Vec::new();
		let mut statements: Vec<Statement> = Vec::new();
		let mut return_type: Option<Type> = None;

		while let Some(pair) = pairs.next() {
			match pair.as_rule() {
				Rule::do_block => statements = self.parse_statements(pair.into_inner()),
				Rule::r#type => return_type = Some(self.parse_type(pair)),
				Rule::function_arguments => arguments = self.parse_function_arguments(pair),
				Rule::definition_generics => generics = self.parse_generics_as_definition_types(pair),

				rule => error!(
					"expected '{:?}', '{:?}', '{:?}' or '{:?}', got '{:?}'",
					pair.as_span(),

					Rule::do_block,
					Rule::r#type,
					Rule::function_arguments,
					Rule::definition_generics,
					rule
				)
			};
		}

		Expression::AnonymousFunction {
			generics,
			arguments,
			return_type: Box::new(return_type),
			statements
		}
	}

	fn parse_elseif_branch(&self, pair: Pair<Rule>) -> ConditionBranch {
		if pair.as_rule() != Rule::if_elseif {
			error!("expected '{:?}', got '{:?}'", pair.as_span(), Rule::if_elseif, pair.as_rule());
		}

		let mut pairs = pair.into_inner();

		if pairs.len() < 1 {
			panic!("Failed to parse elseif branch: pairs are empty");
		}

		let expression_pair = pairs
			.next()
			.expect("Failed to parse elseif branch: expected expression, got nothing");

		let statements_pair = pairs
			.next()
			.expect("Failed to parse elseif branch: expected statements, got nothing");

		ConditionBranch {
			condition: self.parse_expression(expression_pair),
			statements: self.parse_statements(statements_pair.into_inner())
		}
	}

	fn parse_else_branch(&self, pair: Pair<Rule>) -> ConditionBranch {
		if pair.as_rule() != Rule::if_else {
			error!("expected '{:?}', got '{:?}'", pair.as_span(), Rule::if_else, pair.as_rule());
		}

		let mut pairs = pair.into_inner();

		if pairs.len() < 1 {
			panic!("Failed to parse else branch: pairs are empty");
		}

		let statements_pair = pairs
			.next()
			.expect("Failed to parse else branch: expected statements, got nothing");

		ConditionBranch {
			condition: Expression::Empty,
			statements: self.parse_statements(statements_pair.into_inner())
		}
	}

	fn parse_if(&self, pair: Pair<Rule>) -> Statement {
		if pair.as_rule() != Rule::if_stmt {
			error!("expected '{:?}', got '{:?}'", pair.as_span(), Rule::if_stmt, pair.as_rule());
		}

		let mut pairs = pair.into_inner();

		if pairs.len() < 1 {
			panic!("Failed to parse if statement: pairs are empty");
		}

		let condition_pair = pairs
			.next()
			.expect("Failed to parse if statement: condition is missing");

		let statements_pair = pairs
			.next()
			.unwrap_or_else(|| unreachable!("Failed to parse if statement: statements are missing"));

		let mut elseif_branches: Vec<ConditionBranch> = Vec::new();
		let mut else_branch: Option<ConditionBranch> = None;

		while let Some(pair) = pairs.next() {
			if !matches!(pair.as_rule(), Rule::if_elseif | Rule::if_else) {
				error!("expected '{:?}' or '{:?}', got '{:?}'", pair.as_span(), Rule::if_elseif, Rule::if_else, pair.as_rule());
			}
			
			if pair.as_rule() == Rule::if_elseif {
				elseif_branches.push(self.parse_elseif_branch(pair));
			} else {
				else_branch = Some(self.parse_else_branch(pair));
			}
		}

		Statement::If {
			condition: self.parse_expression(condition_pair),
			statements: self.parse_statements(statements_pair.into_inner()),
			elseif_branches,
			else_branch
		}
	}

	fn parse_while(&self, pair: Pair<Rule>) -> Statement {
		if pair.as_rule() != Rule::while_stmt {
			error!("expected '{:?}', got '{:?}'", pair.as_span(), Rule::while_stmt, pair.as_rule());
		}

		let mut pairs = pair.into_inner();

		let expression_pair = pairs
			.next()
			.expect("Failed to parse while statement: expected expression, got nothing");

		let statements_pair = pairs
			.next()
			.expect("Failed to parse while statement: expected statements, got nothing");

		Statement::While {
			condition: self.parse_expression(expression_pair),
			statements: self.parse_statements(statements_pair.into_inner())
		}
	}

	fn parse_for_in(&self, pair: Pair<Rule>) -> Statement {
		if pair.as_rule() != Rule::for_in_stmt {
			error!("expected '{:?}', got '{:?}'", pair.as_span(), Rule::for_in_stmt, pair.as_rule());
		}

		let mut pairs = pair.into_inner();

		let left_pair = pairs
			.next()
			.expect("Failed to parse for statement: left side is missing");

		let right_pair = pairs
			.next()
			.expect("Failed to parse for statement: right side is missing");

		let statements_pair = pairs
			.next()
			.expect("Failed to parse for statement: right side is missing");

		Statement::For {
			left: self.parse_expression(left_pair),
			right: self.parse_expression(right_pair),
			statements: self.parse_statements(statements_pair.into_inner())
		}
	}

	fn parse_unary_expression(&self, pair: Pair<Rule>) -> Expression {
		let mut pairs = pair.into_inner();

		let operator = self.parse_unary_operator(&pairs.next().expect("Failed to parse unary expression: no operator is present"));
		let operand = self.parse_expression(pairs.next().expect("Failed to parse unary expression: no operand is present"));

		Expression::Unary {
			operator,
			operand: Box::new(operand)
		}
	}

	fn parse_literal(&self, pair: Pair<Rule>) -> Expression {
		match pair.as_rule() {
			Rule::number_literal => self.parse_number_literal(pair),
			Rule::string_literal => self.parse_string_literal(pair),
			Rule::boolean_literal => self.parse_boolean_literal(pair),

			rule => error!("expected number or string literal, got '{:?}'", pair.as_span(), rule)
		}
	}

	fn parse_recursive_expression(&self, pairs: Pairs<Rule>) -> Expression {
		let mut pairs = pairs.clone();

		if pairs.len() == 1 {
			return self.parse_expression(pairs.next().unwrap_or_else(|| unreachable!("Failed to parse a recursive expression: pairs are empty")));
		}

		let mut current_expression = Expression::Empty;

		while let Some(pair) = pairs.next() {
			current_expression = match pair.as_rule() {
				Rule::identifier => self.parse_identifier(pair),
				Rule::number_literal | Rule::string_literal => self.parse_literal(pair),

				Rule::expression => self.parse_expression(pair),

				Rule::recursive_call => self.parse_function_call_expression(current_expression, pair),
				Rule::recursive_array => self.parse_array_access_expression(current_expression, pair),
				Rule::recursive_dot => self.parse_dot_access_expression(current_expression, pair),
				Rule::recursive_arrow => self.parse_arrow_access_expression(current_expression, pair),
				Rule::recursive_as => self.parse_type_cast_expression(current_expression, pair),

				rule => error!("expected a recursive expression term, got '{:?}'", pair.as_span(), rule)
			}
		}

		current_expression
	}

	fn parse_function_call_expression(&self, expression: Expression, pair: Pair<Rule>) -> Expression {
		let mut call_insides = pair.into_inner();

		if call_insides.len() < 1 {
			return Expression::FunctionCall {
				callee: Box::new(expression),
				generics: Vec::new(),
				arguments: Vec::new()
			};
		}

		let mut next_pair = call_insides
			.next()
			.unwrap_or_else(|| unreachable!("Failed to parse function call expression: call pairs iterator is empty"));

		let mut generics: Vec<Type> = Vec::new();
		let mut arguments: Vec<Expression> = Vec::new();

		// Notify that incorrect generics are being passed to the function
		if next_pair.as_rule() == Rule::definition_generics {
			error!("definition generics were passed instead of call generics", next_pair.as_span());
		}

		// If the next pair is of type call_generics, parse them, and exit if there are no arguments
		if next_pair.as_rule() == Rule::call_generics {
			generics = self.parse_generics_as_types(next_pair);

			let pair = call_insides.next();
			
			if pair.is_none() {
				return Expression::FunctionCall {
					callee: Box::new(expression),
					generics,
					arguments
				};
			}

			next_pair = pair.unwrap();
		}

		// Parse arguments
		for arg_pair in next_pair.into_inner() {
			// arg_pair is always wrapped in an expression in this case
			let expression_pair = arg_pair
				.into_inner()
				.next()
				.expect("Failed to parse function call argument");

			let parsed = self.parse_expression(expression_pair);
			arguments.push(parsed);
		}

		Expression::FunctionCall {
			callee: Box::new(expression),
			generics,
			arguments
		}
	}

	fn parse_arrow_access_expression(&self, expression: Expression, pair: Pair<Rule>) -> Expression {
		Expression::ArrowAccess {
			expression: Box::new(expression),
			accessor: Box::new(self.parse_expression(pair))
		}
	}

	fn parse_dot_access_expression(&self, expression: Expression, pair: Pair<Rule>) -> Expression {
		Expression::DotAccess {
			expression: Box::new(expression),
			accessor: Box::new(self.parse_expression(pair))
		}
	}

	fn parse_array_access_expression(&self, expression: Expression, pair: Pair<Rule>) -> Expression {
		Expression::ArrayAccess {
			expression: Box::new(expression),
			accessor: Box::new(self.parse_expression(pair))
		}
	}

	fn parse_type_cast_expression(&self, expression: Expression, pair: Pair<Rule>) -> Expression {
		let kind_pair = pair
			.into_inner()
			.next()
			.unwrap_or_else(|| unreachable!("Failed to parse type cast expression: pairs are empty"));

		let kind_parsed = Box::new(self.parse_type(kind_pair));

		Expression::TypeCast {
			value: Box::new(expression),
			kind: kind_parsed
		}
	}

	fn parse_array_expression(&self, pair: Pair<Rule>) -> Expression {
		let mut pairs = pair.into_inner();
		let mut items: Vec<Expression> = Vec::new();

		while let Some(pair) = pairs.next() {
			let parsed_pair = self.parse_expression(pair);
			items.push(parsed_pair);
		}

		Expression::Array(items)
	}

	/// Not to be confused with ~~type definition~~ type alias.
	/// Parses type expressions inside definition generics:
	/// ```plaintext
	/// TypeName: impl<Interface1, Interface2, Interface3, ...>
	/// ```
	/// 
	/// # Arguments
	/// * `pair` - A pest.rs `Pair` of rule `definition_generics_type`
	fn parse_definition_type(&self, pair: Pair<Rule>) -> DefinitionType {
		if pair.as_rule() != Rule::definition_generics_type {
			error!("expected '{:?}', got '{:?}'", pair.as_span(), Rule::definition_generics_type, pair.as_rule());
		}

		let mut pairs = pair.into_inner();

		let name = self.pair_str(
			pairs
				.next()
				.expect("Failed to parse definition generics type: name pair is missing")
		);

		let interfaces_pair = pairs.next();

		if interfaces_pair.is_none() {
			// Type doesn't have to implement any interfaces. Return a regular type
			return DefinitionType {
				name,
				requires_implementations: Vec::new()
			};
		}

		let mut interface_pairs = interfaces_pair.unwrap().into_inner();
		let mut requires_implementations: Vec<String> = Vec::new();

		while let Some(pair) = interface_pairs.next() {
			if pair.as_rule() != Rule::identifier {
				error!("expected '{:?}', got '{:?}'", pair.as_span(), Rule::identifier, pair.as_rule());
			}

			requires_implementations.push(self.pair_str(pair));
		}

		DefinitionType { name, requires_implementations }
	}

	/// Used for **definition** statements. Parses generics **as definition types** to later be substituted with proper types
	/// 
	/// # Arguments
	/// * `pair` - A pest.rs `Pair` of rule `definition_generics`
	fn parse_generics_as_definition_types(&self, pair: Pair<Rule>) -> Vec<DefinitionType> {
		if pair.as_rule() != Rule::definition_generics {
			error!("expected '{:?}', got '{:?}'", pair.as_span(), Rule::definition_generics, pair.as_rule());
		}

		let inner_pairs = pair.into_inner();
		let mut generics: Vec<DefinitionType> = Vec::new();

		for arg in inner_pairs {
			if arg.as_rule() != Rule::definition_generics_type {
				error!("expected '{:?}', got '{:?}'", arg.as_span(), Rule::definition_generics_type, arg.as_rule());
			}

			generics.push(self.parse_definition_type(arg));
		}

		generics
	}

	/// Used for **call** statements. Parses generics **as parser expression types** to later be substituted with proper types
	/// 
	/// # Arguments
	/// * `pair` - A pest.rs `Pair` of rule `call_generics`
	fn parse_generics_as_types(&self, pair: Pair<Rule>) -> Vec<Type> {
		if pair.as_rule() != Rule::call_generics {
			error!("expected '{:?}', got '{:?}'", pair.as_span(), Rule::call_generics, pair.as_rule());
		}

		let inner_pairs = pair.into_inner();
		let mut generics: Vec<Type> = Vec::new();

		for arg in inner_pairs {
			if arg.as_rule() != Rule::r#type {
				error!("expected '{:?}', got '{:?}'", arg.as_span(), Rule::r#type, arg.as_rule());
			}

			generics.push(self.parse_type(arg));
		}

		generics
	}

	fn parse_enum_variant(&self, pair: Pair<Rule>) -> EnumVariant {
		let mut pairs = pair.into_inner();

		let name = self.pair_str(
			pairs
				.next()
				.expect("Failed to parse enum variant: name is missing")
		);

		EnumVariant { name }
	}

	fn parse_enum_definition(&self, pair: Pair<Rule>) -> Statement {
		let mut pairs = pair.into_inner();

		let modifiers_pair = pairs
			.next()
			.expect("Failed to parse enum definition: modifiers are missing");

		let modifiers = self.parse_general_modifiers(modifiers_pair);

		let name = self.pair_str(
			pairs
				.next()
				.expect("Failed to parse enum definition: enum name is missing")
		);

		let mut variants: Vec<EnumVariant> = Vec::new();

		while let Some(pair) = pairs.next() {
			if pair.as_rule() != Rule::enum_variant {
				error!("expected '{:?}', got '{:?}'", pair.as_span(), Rule::enum_variant, pair.as_rule());
			}

			variants.push(self.parse_enum_variant(pair));
		}

		Statement::EnumDefinition { modifiers, name, variants }
	}

	fn parse_regular_type(&self, pair: Pair<Rule>) -> Type {
		if pair.as_rule() != Rule::regular_type {
			error!("expected '{:?}', got '{:?}'", pair.as_span(), Rule::regular_type, pair.as_rule());
		}

		let mut pairs = pair.into_inner();

		let type_pair = pairs
			.next()
			.expect("Failed to parse type: type expression pair is missing");

		let operators_pair = pairs
			.next()
			.expect("Failed to parse type: operators pair is missing");

		let mut operator_pairs = operators_pair.into_inner();

		let mut output_type = Type::Regular {
			base: Box::new(self.parse_expression(type_pair)),
			generics: Vec::new(),
			raw: false,
			array: false
		};

		while let Some(pair) = operator_pairs.next() {
			match pair.as_rule() {
				Rule::type_operator_raw => {
					if let Type::Regular { base, generics, raw: _, array } = output_type {
						output_type = Type::Regular {
							base,
							generics,
							raw: true,
							array
						};
					}
				},

				Rule::type_operator_generics => {
					if let Type::Regular { base, generics: _, raw, array: _ } = output_type {
						let definition_pair = pair
							.into_inner()
							.next()
							.unwrap_or_else(|| unreachable!("Failed to parse type: generics pair is missing"));

						output_type = Type::Regular {
							base,
							generics: self.parse_generics_as_definition_types(definition_pair),
							raw,
							array: true
						};
					}
				},

				Rule::type_operator_array => {
					if let Type::Regular { base, generics, raw, array: _ } = output_type {
						output_type = Type::Regular {
							base,
							generics,
							raw,
							array: true
						};
					}
				},

				rule => error!("unexpected rule '{:?}'", pair.as_span(), rule)
			}
		}

		output_type
	}

	fn parse_function_type(&self, pair: Pair<Rule>) -> Type {
		if pair.as_rule() != Rule::function_type {
			error!("expected '{:?}', got '{:?}'", pair.as_span(), Rule::function_type, pair.as_rule());
		}

		let mut pairs = pair.into_inner();

		let mut arguments_pairs = pairs
			.next()
			.expect("Failed to parse function type: arguments pair is missing")
			.into_inner();

		let return_type_pair = pairs
			.next()
			.expect("Failed to parse function type: arguments pair is missing");

		let mut argument_types: Vec<Type> = Vec::new();

		while let Some(pair) = arguments_pairs.next() {
			if pair.as_rule() != Rule::r#type {
				error!("expected '{:?}', got '{:?}'", pair.as_span(), Rule::r#type, pair.as_rule());
			}

			argument_types.push(self.parse_type(pair));
		}

		let return_type = Box::new(self.parse_type(return_type_pair));

		Type::Function {
			argument_types,
			return_type
		}
	}

	fn parse_type(&self, pair: Pair<Rule>) -> Type {
		if pair.as_rule() != Rule::r#type {
			error!("expected '{:?}', got '{:?}'", pair.as_span(), Rule::r#type, pair.as_rule());
		}

		let mut pairs = pair.into_inner();

		let inner_type_pair = pairs
			.next()
			.expect("Failed to parse type: pairs are empty");

		match inner_type_pair.as_rule() {
			Rule::regular_type => self.parse_regular_type(inner_type_pair),
			Rule::function_type => self.parse_function_type(inner_type_pair),

			rule => error!(
				"expected '{:?}' or '{:?}', got '{:?}'",
				inner_type_pair.as_span(),

				Rule::regular_type,
				Rule::function_type,
				rule
			)
		}
	}

	fn parse_class_function_attributes(&self, pair: Pair<Rule>) -> ClassFunctionAttributes {
		if pair.as_rule() != Rule::attributes {
			panic!("Failed to parse function attributes: got an unexpected rule. Expected '{:?}', got '{:?}'", Rule::attributes, pair.as_rule());
		}
		
		let mut pairs = pair.into_inner();
		let mut attributes: ClassFunctionAttributes = Vec::new();

		// Keeping track of which attributes have already been defined to prevent users from defining them twice
		let mut met_attributes: Vec<String> = Vec::new();

		while let Some(pair) = pairs.next() {
			let as_str = pair.as_str();
			let span = pair.as_span();

			let owned_str = as_str.clone().to_owned();

			if met_attributes.contains(&owned_str) {
				error!("found more than one '{}' attribute definition", span, as_str);
			}

			let attribute = ClassFunctionAttribute::try_from(as_str).unwrap();
			attributes.push(attribute);

			// Mark the attribute as defined
			met_attributes.push(owned_str);
		}

		attributes
	}

	fn parse_function_arguments(&self, pair: Pair<Rule>) -> Vec<FunctionArgument> {
		if pair.as_rule() != Rule::function_arguments {
			error!("expected '{:?}', got '{:?}'", pair.as_span(), Rule::function_arguments, pair.as_rule());
		}

		let mut pairs = pair.into_inner();

		let mut arguments: Vec<FunctionArgument> = Vec::new();

		while let Some(pair) = pairs.next() {
			if pair.as_rule() != Rule::function_argument {
				error!("expected '{:?}', got '{:?}'", pair.as_span(), Rule::function_argument, pair.as_rule());
			}

			let mut arg_pairs = pair.into_inner();

			let name_pair = arg_pairs
				.next()
				.expect("Failed to parse function definition arguments: argument name is missing");

			let kind_pair = arg_pairs
				.next()
				.expect("Failed to parse function definition arguments: argument type is missing");

			arguments.push(FunctionArgument {
				name: self.pair_str(name_pair),
				kind: self.parse_type(kind_pair)
			});
		}

		arguments
	}

	fn parse_general_modifiers(&self, pair: Pair<Rule>) -> GeneralModifiers {
		if pair.as_rule() != Rule::general_modifiers {
			error!("expected '{:?}', got '{:?}'", pair.as_span(), Rule::general_modifiers, pair.as_rule());
		}

		let mut modifiers: GeneralModifiers = Vec::new();
		let mut pairs = pair.into_inner();

		let mut met_modifiers: Vec<String> = Vec::new();

		while let Some(pair) = pairs.next() {
			let as_str = pair.as_str();
			let span = pair.as_span();

			let owned_str = as_str.clone().to_owned();

			if met_modifiers.contains(&owned_str) {
				error!("found more than one '{}' modifier definition", span, as_str);
			}

			let modifier = GeneralModifier::try_from(as_str).unwrap();
			modifiers.push(modifier);

			// Mark the modifier as defined
			met_modifiers.push(owned_str);
		}

		modifiers
	}

	fn parse_function_header(&self, pair: Pair<Rule>) -> (GeneralModifiers, String, Vec<DefinitionType>, Vec<FunctionArgument>, Option<Type>) {
		let mut header_pairs = pair.into_inner();

		let modifiers_pair = header_pairs
			.next()
			.expect("Failed to parse function header: modifiers are missing");

		let modifiers = self.parse_general_modifiers(modifiers_pair);

		let name = header_pairs
			.next()
			.expect("Failed to parse function header: function name is missing");

		let mut generics: Vec<DefinitionType> = Vec::new();
		let mut arguments: Vec<FunctionArgument> = Vec::new();
		let mut return_type: Option<Type> = None;

		while let Some(pair) = header_pairs.next() {
			match pair.as_rule() {
				Rule::definition_generics => generics = self.parse_generics_as_definition_types(pair),
				Rule::function_arguments => arguments = self.parse_function_arguments(pair),
				Rule::r#type => return_type = Some(self.parse_type(pair)),

				rule => error!("unexpected rule '{:?}'", pair.as_span(), rule)
			}
		}

		(modifiers, self.pair_str(name), generics, arguments, return_type)
	}

	fn parse_function_declaration(&self, pair: Pair<Rule>) -> Statement {
		if pair.as_rule() != Rule::function_declaration_stmt {
			panic!("Failed to parse function declaration: got an unexpected rule. Expected '{:?}', got '{:?}'", Rule::function_declaration_stmt, pair.as_rule());
		}

		let header_pair = pair
			.into_inner()
			.next()
			.expect("Failed to parse function declaration: function header is missing");

		let (modifiers, name, generics, arguments, return_type) = self.parse_function_header(header_pair);
		Statement::FunctionDeclaration { modifiers, name, generics, arguments, return_type }
	}

	fn parse_function_definition(&self, pair: Pair<Rule>) -> Statement {
		let mut pairs = pair.into_inner();

		let header_pair = pairs
			.next()
			.expect("Failed to parse function definition: function header is missing");

		// Parsing the header
		let (modifiers, name, generics, arguments, return_type) = self.parse_function_header(header_pair);

		let body_pairs = pairs
			.next()
			.expect("Failed to parse function definition: function body is missing")
			.into_inner();

		Statement::FunctionDefinition {
			modifiers,
			name,
			generics,
			arguments,
			return_type,
			statements: self.parse_statements(body_pairs)
		}
	}

	fn parse_type_alias(&self, pair: Pair<Rule>) -> Statement {
		let mut pairs = pair.into_inner();

		let modifiers_pair = pairs
			.next()
			.expect("Failed to parse type definition: expected modifiers, got nothing");

		let modifiers = self.parse_general_modifiers(modifiers_pair);

		let name_pair = pairs
			.next()
			.expect("Failed to parse type definition: expected an identifier as a type name, got nothing");

		let mut next_pair = pairs
			.next()
			.expect("Failed to parse type definition: expected generics/type, got nothing");

		let mut generics: Vec<DefinitionType> = Vec::new();

		if next_pair.as_rule() == Rule::definition_generics {
			generics = self.parse_generics_as_definition_types(next_pair);

			next_pair = pairs
				.next()
				.expect("Failed to parse type definition: expected a type, got nothing");
		}

		let type_expression = self.parse_type(next_pair);

		Statement::TypeAlias {
			modifiers,
			name: self.pair_str(name_pair),
			generics,
			definition: type_expression
		}
	}

	fn parse_class_definition_function(&self, pair: Pair<Rule>) -> ClassDefinitionMember {
		if pair.as_rule() != Rule::class_definition_function {
			panic!("Failed to parse a class definition function: expected rule '{:?}', got '{:?}'", Rule::class_definition_function, pair.as_rule());
		}

		let mut inner_pairs = pair.into_inner();

		let next_pair = inner_pairs
			.peek()
			.expect(&format!("Failed to parse a class definition function: expected '{:?}' or '{:?}', got nothing", Rule::function_definition_stmt, Rule::attributes));
		
		let mut attributes: Option<ClassFunctionAttributes> = None;

		if next_pair.as_rule() == Rule::attributes {
			attributes = Some(self.parse_class_function_attributes(next_pair));

			// Skip attributes if they exist
			inner_pairs.next();
		}

		let statement_pair = inner_pairs
			.next()
			.expect(&format!("Failed to parse a class definition function: expected rule '{:?}', got nothing", Rule::function_definition_stmt));

		let function_statement = self.parse_function_definition(statement_pair);
		ClassDefinitionMember::function_from_statement(function_statement, attributes)
	}

	fn parse_class_definition_variable(&self, pair: Pair<Rule>) -> ClassDefinitionMember {
		if pair.as_rule() != Rule::class_definition_variable {
			panic!("Failed to parse class definiton variable: expected rule '{:?}', got '{:?}'", Rule::class_definition_variable, pair.as_rule());
		}

		let mut inner_pairs = pair.into_inner();

		let modifiers_pair = inner_pairs
			.next()
			.expect(&format!("Failed to parse class definition variable: expected '{:?}', got nothing", Rule::general_modifiers));

		let modifiers = self.parse_general_modifiers(modifiers_pair);

		let name = inner_pairs
			.next()
			.expect(&format!("Failed to parse class definition variable: expected '{:?}', got nothing", Rule::identifier));

		let kind = inner_pairs
			.next()
			.expect(&format!("Failed to parse class definition variable: expected '{:?}', got nothing", Rule::r#type));

		let default_value_option = inner_pairs.next();
		let mut default_value = Expression::Empty;

		if default_value_option.is_some() {
			default_value = self.parse_expression(default_value_option.unwrap_or_else(|| unreachable!("Failed to parse class definition variable: default value pair is missing")));
		}

		ClassDefinitionMember::Variable {
			modifiers,
			name: self.pair_str(name),
			kind: self.parse_type(kind),
			default_value
		}
	}

	fn parse_class_definition_member(&self, pair: Pair<Rule>) -> ClassDefinitionMember {
		if pair.as_rule() != Rule::class_definition_member {
			panic!("Failed to parse class definition member: expected rule '{:?}', got '{:?}'", Rule::class_definition_member, pair.as_rule());
		}

		let inner = pair
			.into_inner()
			.next()
			.expect("Failed to parse class definition member: pairs are empty");

		match inner.as_rule() {
			Rule::class_definition_variable => self.parse_class_definition_variable(inner),
			Rule::class_definition_function => self.parse_class_definition_function(inner),

			rule => error!(
				"expected '{:?}' or '{:?}', got '{:?}'",
				inner.as_span(),

				Rule::class_definition_variable,
				Rule::class_definition_function,
				rule
			)
		}
	}

	fn parse_class_definition(&self, pair: Pair<Rule>) -> Statement {
		let mut pairs = pair.into_inner();

		let modifiers_pair = pairs
			.next()
			.expect("Failed to parse class definition: modifiers are missing");

		let modifiers = self.parse_general_modifiers(modifiers_pair);

		let name = pairs
			.next()
			.expect("Failed to parse class definition: class name is missing");

		if name.as_rule() != Rule::identifier {
			error!("expected '{:?}', got '{:?}'", name.as_span(), Rule::identifier, name.as_rule());
		}

		let next_pair_option = pairs.peek();
		let next_pair: Pair<Rule>;

		// If the class is empty beyond this point, return early
		if next_pair_option.is_none() {
			return Statement::ClassDefinition {
				modifiers,
				name: self.pair_str(name),
				generics: Vec::new(),
				members: Vec::new()
			};
		}

		// Unwrap the next pair
		next_pair = next_pair_option.unwrap_or_else(|| unreachable!("Failed to parse class definition: unexpected end of pairs. Expected class members or generics, got nothing"));
		let mut generics: Vec<DefinitionType> = Vec::new();

		// Check if the next pair is of rule definition_generics
		if next_pair.as_rule() == Rule::definition_generics {
			generics = self.parse_generics_as_definition_types(next_pair);

			// If no class members are provided, return early
			if pairs.peek().is_none() {
				return Statement::ClassDefinition {
					modifiers,
					name: self.pair_str(name),
					generics,
					members: Vec::new()
				};
			}

			// Otherwise, skip the current pair
			pairs.next();
		}

		let mut members: Vec<ClassDefinitionMember> = Vec::new();

		while let Some(pair) = pairs.next() {
			members.push(self.parse_class_definition_member(pair));
		}

		Statement::ClassDefinition {
			modifiers,
			name: self.pair_str(name),
			generics,
			members
		}
	}

	fn parse_class_declaration_function(&self, pair: Pair<Rule>) -> ClassDeclarationMember {
		// * NOTE: attributes are to be checked later by the optimization stage/compiler

		if pair.as_rule() != Rule::class_declaration_function {
			panic!("Got an unexpected rule as a class declaration member: expected rule '{:?}', got '{:?}'", Rule::class_declaration_function, pair.as_rule());
		}

		let mut inner_pairs = pair.into_inner();

		let next_pair = inner_pairs
			.peek()
			.expect(&format!("Failed to parse a class declaration function: expected '{:?}' or '{:?}', got nothing", Rule::class_declaration_function, Rule::attributes));
		
		let mut attributes: Option<ClassFunctionAttributes> = None;

		if next_pair.as_rule() == Rule::attributes {
			attributes = Some(self.parse_class_function_attributes(next_pair));

			// Skip attributes if they exist
			inner_pairs.next();
		}

		let function_pair = inner_pairs
			.next()
			.expect(&format!("Failed to parse a class declaration function: expected rule '{:?}', got nothing", Rule::function_definition_stmt));

		let function_statement = self.parse_function_declaration(function_pair);
		ClassDeclarationMember::function_from_statement(function_statement, attributes)
	}

	fn parse_class_declaration_variable(&self, pair: Pair<Rule>) -> ClassDeclarationMember {
		if pair.as_rule() != Rule::class_declaration_variable {
			panic!("Failed to parse class declaration member: got an unexpected rule. Expected rule '{:?}', got '{:?}'", Rule::class_declaration_variable, pair.as_rule());
		}

		let mut inner_pairs = pair.into_inner();

		let modifiers_pair = inner_pairs
			.next()
			.expect(&format!("Failed to parse class declaration variable: expected rule '{:?}', got nothing", Rule::general_modifiers));

		let modifiers = self.parse_general_modifiers(modifiers_pair);

		let name = inner_pairs
			.next()
			.expect(&format!("Failed to parse class declaration variable: expected rule '{:?}', got nothing", Rule::identifier));

		let kind = inner_pairs
			.next()
			.expect(&format!("Failed to parse class declaration variable: expected rule '{:?}', got nothing", Rule::r#type));

		ClassDeclarationMember::Variable {
			modifiers,
			name: self.pair_str(name),
			kind: self.parse_type(kind)
		}
	}

	fn parse_class_declaration_member(&self, pair: Pair<Rule>) -> ClassDeclarationMember {
		if pair.as_rule() != Rule::class_declaration_member {
			panic!("Failed to parse class declaration member: got an unexpected rule. Expected rule '{:?}', got '{:?}'", Rule::class_declaration_member, pair.as_rule());
		}

		let inner = pair
			.into_inner()
			.next()
			.unwrap_or_else(|| unreachable!("Failed to parse class declaration member: pairs are empty"));

		match inner.as_rule() {
			Rule::class_declaration_variable => self.parse_class_declaration_variable(inner),
			Rule::class_declaration_function => self.parse_class_declaration_function(inner),

			rule => error!(
				"expected '{:?}' or '{:?}', got '{:?}'",
				inner.as_span(),
				Rule::class_declaration_variable,
				Rule::class_declaration_function,
				rule
			)
		}
	}

	fn parse_class_declaration(&self, pair: Pair<Rule>) -> Statement {
		let mut pairs = pair.into_inner();

		let modifiers_pair = pairs
			.next()
			.expect(&format!("Failed to parse class declaration variable: expected rule '{:?}', got nothing", Rule::general_modifiers));

		let modifiers = self.parse_general_modifiers(modifiers_pair);

		let name = pairs
			.next()
			.expect("Failed to parse class declaration: class name is missing");

		if name.as_rule() != Rule::identifier {
			error!("expected '{:?}', got '{:?}'", name.as_span(), Rule::identifier, name.as_rule());
		}

		let next_pair_option = pairs.peek();
		let next_pair: Pair<Rule>;

		// If the class is empty beyond this point, return early
		if next_pair_option.is_none() {
			return Statement::ClassDeclaration {
				modifiers,
				name: self.pair_str(name),
				generics: Vec::new(),
				members: Vec::new()
			};
		}

		// Unwrap the next pair
		next_pair = next_pair_option.unwrap_or_else(|| unreachable!("Failed to parse class declaration: expected generics or class members, got nothing"));
		let mut generics: Vec<DefinitionType> = Vec::new();

		// Check if the next pair is of rule definition_generics
		if next_pair.as_rule() == Rule::definition_generics {
			generics = self.parse_generics_as_definition_types(next_pair);

			// If no class members are provided, return early
			if pairs.peek().is_none() {
				return Statement::ClassDeclaration {
					modifiers,
					name: self.pair_str(name),
					generics,
					members: Vec::new()
				};
			}

			// Otherwise, skip the current pair
			pairs.next();
		}

		let mut members: Vec<ClassDeclarationMember> = Vec::new();

		while let Some(pair) = pairs.next() {
			members.push(self.parse_class_declaration_member(pair));
		}

		Statement::ClassDeclaration {
			modifiers,
			name: self.pair_str(name),
			generics,
			members
		}
	}

	fn parse_variable_definition(&self, pair: Pair<Rule>) -> Statement {
		let mut pairs = pair.into_inner();

		let modifiers_pair = pairs
			.next()
			.expect(&format!("Failed to parse class declaration variable: expected rule '{:?}', got nothing", Rule::general_modifiers));

		let modifiers = self.parse_general_modifiers(modifiers_pair);

		let name = pairs
			.next()
			.expect("Failed to parse variable definition: variable name is missing");

		let mut next_pair = pairs
			.next()
			.expect("Failed to parse variable definition: expected type/value, got nothing");

		let mut kind = None;
		let value: Expression;

		if next_pair.as_rule() == Rule::r#type {
			kind = Some(self.parse_type(next_pair));

			next_pair = pairs
				.next()
				.expect("Failed to parse variable definition: variable value is missing");
		}
		
		if next_pair.as_rule() == Rule::expression {
			value = self.parse_expression(next_pair);
		} else {
			error!("expected expression, got '{:?}'", next_pair.as_span(), next_pair.as_rule());
		}

		Statement::VariableDefinition {
			modifiers,
			name: self.pair_str(name),
			kind,
			value
		}
	}

	fn parse_variable_assign(&self, pair: Pair<Rule>) -> Statement {
		let mut pairs = pair.into_inner();

		let variable_pair = pairs
			.next()
			.expect("Failed to parse variable assign: expected a variable name, got nothing");

		let value_pair = pairs
			.next()
			.expect("Failed to parse variable assign: expected an expression, got nothing");

		Statement::VariableAssign {
			name: self.parse_expression(variable_pair),
			value: self.parse_expression(value_pair)
		}
	}

	fn parse_function_call(&self, pair: Pair<Rule>) -> Statement {
		let span = pair.as_span();
		let pairs = pair.into_inner();

		let last_pair = pairs
			.clone()
			.last()
			.unwrap_or_else(|| unreachable!("Failed to parse function call statement: pairs are empty"));

		// Check if the expression doesn't end with a function call `<...>(...)`
		if last_pair.as_rule() != Rule::recursive_call {
			error!("expression statement is not a function call", span);
		}

		let parsed = self.parse_recursive_expression(pairs);

		if let Expression::FunctionCall {
			callee,
			generics,
			arguments
		} = parsed {
			return Statement::FunctionCall {
				callee: *callee,
				generics,
				arguments
			};
		}

		panic!("Failed to parse function call statement: callee or arguments parameters are invalid");
	}

	fn parse_return(&self, pair: Pair<Rule>) -> Statement {
		let pairs = pair.into_inner();

		if pairs.len() > 0 {
			let expression_pair = pairs
				.peek()
				.expect("Failed to parse function call expression as a statement");

			return Statement::Return(self.parse_expression(expression_pair));
		}

		Statement::Return(Expression::Empty)
	}
}