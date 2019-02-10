#![feature(rustc_private)]

extern crate rustc;
extern crate syntax;

mod data_structure;
use data_structure::*;

mod expand;

pub type ParseSess = syntax::parse::ParseSess;
pub type LangRust = syntax::ast::Crate;
pub type LangHIR = rustc::hir::Crate;
pub type DiagnosticBuilder<'a> = syntax::diagnostics::plugin::DiagnosticBuilder<'a>;
