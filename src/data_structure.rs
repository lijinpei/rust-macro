use syntax::ast;
use syntax::source_map::symbol::Ident;

pub enum Binding {
    Type(),
    Var(),
    Function(),
    Macro(),
}

pub struct ModRibcage {}

impl ModRibcage {
    pub fn add_extern_crate(
        ident: Ident,
        vis: ast::Visibility,
        o_name: &Option<String>,
    ) -> ast::NodeId {
    }
    pub fn add_use_tree(use_tree: &ast::UseTree) -> ast::NodeId {}
}

pub enum RibCage {
    RecRibCage(),
    LetRibCage(),
}

pub struct SyntaxObject {}
