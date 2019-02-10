use super::{data_structure::*, *};
use syntax::ast;

// krate is mutable, because we will fill in its NodeId
pub fn expand_crate<'a>(krate: &mut LangRust, sess: &'a ParseSess) -> LangHIR {
    let mut ribcage = ModRibcage {};
    let LangRust {
        module,
        attrs,
        span,
    } = krate;

    // Step 1: Collect all bindings
    for item in module.items.iter() {
        expand_item(item, sess, &mut ribcage);
    }

    // Step 2: Expand all macro uses

    // Step 3: Do all the remaining work: generate HIR
    panic!();
}

pub fn expand_item<'a>(item: &mut ast::Item, sess: &'a ParseSess, ribcage: &mut ModRibcage) {
    use syntax::ast::ItemKind::*;
    item.id = match item.node {
        ExternCrate(o_name) => ribcage.add_extern_crate(item.ident, item.vis, o_name),
        Use(p_use_tree) => ribcage.add_use_tree(p_use_tree),
        Static(_p_ty, mutability, _p_expr) => ribcage.add_static(item.ident, mutability),
        Const(_p_ty, _p_expr) => ribcage.add_const(item.ident),
        Fn(_p_fn_declP, _fn_header, _generics, _p_block) => ribcage.add_function(item.ident),
        Mod(_mud) => ribcage.add_mod(item.ident),
        ForeignMod(foreign_mod) => {
            ribcage.add_foreign_mod(&foreign_mod);
        }
        GlobalAsm(p_global_asm) => ast::DUMMY_NODE_ID,
        Ty(p_ty, generics) => {
            ribcage.add_type(item.ident);
        }
        Existential(generics_bounds, generics) => {
            // FIXME: currently we ignore existensial
            ast::DUMMY_NODE_ID
        }
        Enum(_enum_def, _generics) => {}
        Struct(_variant_data, _generics) => ribcage.add_struct(item.ident),
        Union(_variant_data, _generics) => ribcage.add_union(item.ident),
        Trait(_is_auto, _unsafety, _generics, _generics_bounds, _vec_trait_item) => {
            ribcage.add_trait(item.ident)
        }
        TraitAlias(_generics, _generic_bounds) => ribcage.add_trait_alias(item.ident),
        Impl(
            _unsafety,
            _impl_polarity,
            _defaultness,
            _generics,
            _o_trait_ref,
            _p_ty,
            _vec_impl_item,
        ) => ribcage.add_impl(item.ident),
        Mac(_mac) => {
            // defer macro expansion to next phase to permit macro-use before macro-def
            // eg:
            // foo!{}
            // macro!(foo ...)
            ast::DUMMY_NODE_ID
        }
        MacroDef(_macro_def) => ribcage.add_macro_def(item.ident),
    }
}
