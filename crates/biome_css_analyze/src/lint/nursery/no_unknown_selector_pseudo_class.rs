use biome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_css_syntax::{AnyCssPseudoElement, CssPseudoElementSelector};
use biome_rowan::AstNode;

use crate::utils::{is_pseudo_elements, vender_prefix};

declare_rule! {}