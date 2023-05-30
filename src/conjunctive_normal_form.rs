use crate::negation_normal_form;

pub fn conjunctive_normal_form(formula: &str) -> String {
	return negation_normal_form(formula);
}
