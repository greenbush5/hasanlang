use crate::{
	HasanCodegen, GeneralModifiers, Type,
	cond_vec_transform, ClassFunctionAttributes, vec_transform_str,
	DefinitionType
};

#[derive(Debug, Clone, PartialEq)]
pub enum InterfaceMember {
	Variable(InterfaceVariable),
	Function(InterfaceFunction),
	AssocType(InterfaceAssocType)
}

impl HasanCodegen for InterfaceMember {
	fn codegen(&self) -> String {
		match self {
			Self::Variable(variable) => variable.codegen(),
			Self::Function(function) => function.codegen(),
			Self::AssocType(assoc_type) => assoc_type.codegen()
		}
	}
}

//-----------------------------------------------------------------//

#[derive(Debug, Clone, PartialEq)]
pub struct InterfaceVariable {
	pub modifiers: GeneralModifiers,

	pub name: String,
	pub kind: Type
}

impl HasanCodegen for InterfaceVariable {
	fn codegen(&self) -> String {
		let modifiers = self.modifiers.to_string();

		format!(
			"{modifiers}var {}: {};",
			self.name,
			self.kind.codegen()
		)
	}
}

//-----------------------------------------------------------------//

#[derive(Debug, Clone, PartialEq)]
pub struct InterfaceFunction {
	pub attributes: Option<ClassFunctionAttributes>,
	pub prototype: InterfaceFunctionPrototype
}

impl HasanCodegen for InterfaceFunction {
	fn codegen(&self) -> String {
		let attributes = match self.attributes.clone() {
			Some(attributes) => {
				let attributes = vec_transform_str(
					&attributes,
					|value| value.to_string(),
					", "
				);

				if !attributes.is_empty() {
					format!("#[{attributes}]\n")
				} else {
					String::new()
				}
			},

			None => String::new()
		};

		let prototype = self.prototype.codegen();

		format!("{attributes}{prototype};")
	}
}

//-----------------------------------------------------------------//

#[derive(Debug, Clone, PartialEq)]
pub struct InterfaceFunctionPrototype {
	pub modifiers: GeneralModifiers,

	pub name: String,
	pub generics: Vec<DefinitionType>,
	pub argument_types: Vec<Type>,
	pub return_type: Type
}

impl HasanCodegen for InterfaceFunctionPrototype {
	fn codegen(&self) -> String {
		let modifiers = self.modifiers.to_string();
		let name = self.name.clone();
		let generics = cond_vec_transform!(&self.generics, |value| value.codegen(), ", ", "<{}>");
		let argument_types = vec_transform_str(&self.argument_types, |value| value.codegen(), ", ");
		let return_type = self.return_type.codegen();

		format!("{modifiers}func {name}{generics}({argument_types}) -> {return_type}")
	}
}

//-----------------------------------------------------------------//

#[derive(Debug, Clone, PartialEq)]
pub struct InterfaceAssocType {
	pub modifiers: GeneralModifiers,
	pub name: String
}

impl HasanCodegen for InterfaceAssocType {
	fn codegen(&self) -> String {
		let modifiers = self.modifiers.to_string();
		let name = self.name.clone();

		format!("{modifiers}type {name};")
	}
}