use crate::{HasanCodegen, vec_transform_str};
use super::RegularType;

#[derive(Debug, Clone, PartialEq)]
pub struct DefinitionType {
	pub name: String,

	// Is it a good idea to reuse `RegularType` for this case?
	pub requires_implementations: Vec<RegularType>
}

impl HasanCodegen for DefinitionType {
	fn codegen(&self) -> String {
		let requires_impls = vec_transform_str(
			&self.requires_implementations,
			|interface| interface.codegen(),
			", "
		);

		if requires_impls.is_empty() {
			self.name.clone()
		} else {
			format!("{}: impl<{}>", self.name, requires_impls)
		}
	}
}