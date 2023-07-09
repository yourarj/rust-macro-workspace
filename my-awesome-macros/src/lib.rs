//! My Awesome Macro crate
//!
//! ## Home for rust macros

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse2, Attribute, Item, Local};

const LOGGABLE_TEXT: &'static str = "loggable";

mod keyword {
    syn::custom_keyword!(check_if);
    syn::custom_keyword!(ret_me);
}

/// ## simple_func_lke_macro!()
///
/// In the following example simply ignores the input\
/// and replace with following func definition irrespective of input
///
/// <pre>
///   fn give_me_number() -> u32 {
///     86543
///   }
/// </pre>
///
#[proc_macro]
pub fn simple_func_like_macro(_item: TokenStream) -> TokenStream {
    "fn give_me_the_lucky_number() -> i32 { 86543 }"
        .parse()
        .unwrap()
}

/// JustForFunNoDerive capability
/// I'll not make any changes in the code
///
#[proc_macro_derive(JustForFunNoDerive, attributes(field_one, field_two))]
pub fn just_for_fun_no_derive(_item: TokenStream) -> TokenStream {
    TokenStream::new()
}

/// simple macro attribute\
/// it prints attributes recieved and\
/// and item being annotated\
#[proc_macro_attribute]
pub fn simple_proc_macro_attribute(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!(
        "attributes of simple_proc_macro_attribute: \"{}\"",
        attr.to_string()
    );
    println!("attributed item itself: \"{}\"", item.to_string());
    item
}

/// We are making function loggable
#[proc_macro_attribute]
pub fn loggable(attr: TokenStream, item: TokenStream) -> TokenStream {
    match process_attribute_macro(attr, item) {
        Ok(ts) => ts,
        Err(err) => err.to_compile_error().into(),
    }
}

fn process_attribute_macro(
    _attr: TokenStream,
    item: TokenStream,
) -> Result<TokenStream, syn::Error> {
    let mut parsed_func = parse2::<syn::ItemFn>(item.into())?;

    println!("attrs from func: {:#?}", parsed_func.attrs);
    // println!("parsed attrs of func: {:?}", parsed_attr);

    for stmt in parsed_func.block.stmts.iter_mut() {
        if let Some(attrs) = get_attributes_of_statement(stmt) {
            for attrib in attrs {
                println!("{:#?}", attrib);
            }
            println!(
                "\n\n___________________________________________________________________________"
            )
        }
    }

    // parsed_func.block.
    Ok(quote! {}.into())
}

fn get_attributes_of_statement(stmt: &mut syn::Stmt) -> Option<&mut Vec<Attribute>> {
    match stmt {
        syn::Stmt::Local(syn::Local { attrs, .. }) => Some(attrs),
        syn::Stmt::Item(item) => get_attributes_of_item(item),
        syn::Stmt::Expr(expr, _) => get_attributes_of_expr(expr),
        syn::Stmt::Macro(syn::StmtMacro { attrs, .. }) => Some(attrs),
    }
}

/**
 * Extracts attributes of item of any type
 */
fn get_attributes_of_item(item: &mut syn::Item) -> Option<&mut Vec<Attribute>> {
    match item {
        Item::Const(syn::ItemConst { attrs, .. })
        | Item::Enum(syn::ItemEnum { attrs, .. })
        | Item::ExternCrate(syn::ItemExternCrate { attrs, .. })
        | Item::Fn(syn::ItemFn { attrs, .. })
        | Item::ForeignMod(syn::ItemForeignMod { attrs, .. })
        | Item::Impl(syn::ItemImpl { attrs, .. })
        | Item::Macro(syn::ItemMacro { attrs, .. })
        | Item::Mod(syn::ItemMod { attrs, .. })
        | Item::Static(syn::ItemStatic { attrs, .. })
        | Item::Struct(syn::ItemStruct { attrs, .. })
        | Item::Trait(syn::ItemTrait { attrs, .. })
        | Item::TraitAlias(syn::ItemTraitAlias { attrs, .. })
        | Item::Type(syn::ItemType { attrs, .. })
        | Item::Union(syn::ItemUnion { attrs, .. })
        | Item::Use(syn::ItemUse { attrs, .. }) => Some(attrs),

        // in other cases return None
        _ => None,
    }
}

/**
 * Extracts attributes of item of any type
 */
fn get_attributes_of_expr(expr: &mut syn::Expr) -> Option<&mut Vec<Attribute>> {
    match expr {
        syn::Expr::Array(syn::ExprArray { attrs, .. })
        | syn::Expr::Assign(syn::ExprAssign { attrs, .. })
        | syn::Expr::Async(syn::ExprAsync { attrs, .. })
        | syn::Expr::Await(syn::ExprAwait { attrs, .. })
        | syn::Expr::Binary(syn::ExprBinary { attrs, .. })
        | syn::Expr::Block(syn::ExprBlock { attrs, .. })
        | syn::Expr::Break(syn::ExprBreak { attrs, .. })
        | syn::Expr::Call(syn::ExprCall { attrs, .. })
        | syn::Expr::Cast(syn::ExprCast { attrs, .. })
        | syn::Expr::Closure(syn::ExprClosure { attrs, .. })
        | syn::Expr::Const(syn::ExprConst { attrs, .. })
        | syn::Expr::Continue(syn::ExprContinue { attrs, .. })
        | syn::Expr::Field(syn::ExprField { attrs, .. })
        | syn::Expr::ForLoop(syn::ExprForLoop { attrs, .. })
        | syn::Expr::Group(syn::ExprGroup { attrs, .. })
        | syn::Expr::If(syn::ExprIf { attrs, .. })
        | syn::Expr::Index(syn::ExprIndex { attrs, .. })
        | syn::Expr::Infer(syn::ExprInfer { attrs, .. })
        | syn::Expr::Let(syn::ExprLet { attrs, .. })
        | syn::Expr::Lit(syn::ExprLit { attrs, .. })
        | syn::Expr::Loop(syn::ExprLoop { attrs, .. })
        | syn::Expr::Macro(syn::ExprMacro { attrs, .. })
        | syn::Expr::Match(syn::ExprMatch { attrs, .. })
        | syn::Expr::MethodCall(syn::ExprMethodCall { attrs, .. })
        | syn::Expr::Paren(syn::ExprParen { attrs, .. })
        | syn::Expr::Path(syn::ExprPath { attrs, .. })
        | syn::Expr::Range(syn::ExprRange { attrs, .. })
        | syn::Expr::Reference(syn::ExprReference { attrs, .. })
        | syn::Expr::Repeat(syn::ExprRepeat { attrs, .. })
        | syn::Expr::Return(syn::ExprReturn { attrs, .. })
        | syn::Expr::Struct(syn::ExprStruct { attrs, .. })
        | syn::Expr::Try(syn::ExprTry { attrs, .. })
        | syn::Expr::TryBlock(syn::ExprTryBlock { attrs, .. })
        | syn::Expr::Tuple(syn::ExprTuple { attrs, .. })
        | syn::Expr::Unary(syn::ExprUnary { attrs, .. })
        | syn::Expr::Unsafe(syn::ExprUnsafe { attrs, .. })
        | syn::Expr::While(syn::ExprWhile { attrs, .. })
        | syn::Expr::Yield(syn::ExprYield { attrs, .. }) => Some(attrs),
        _ => None,
    }
}

fn get_first_loggable_attribute(attrs: &Vec<Attribute>) -> syn::Result<Option<Attribute>> {
    if let Some(found_ind) = attrs.iter().position(|elem| {
        elem.path()
            .segments
            .first()
            .map_or(false, |seg| seg.ident == LOGGABLE_TEXT)
    }) {
        let attribute = attrs.remove(found_ind).into_token_stream();
        Ok(Some(syn::parse2(attribute)?))
    } else {
        Ok(None)
    }
}
