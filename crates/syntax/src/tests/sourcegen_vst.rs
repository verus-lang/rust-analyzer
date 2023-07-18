//! This module generates VST datatype used by verus-analyzer.
//!
//! The VST datatype is generated from the ungrammar file.

use crate::tests::ast_src::{AstSrc, KindsSrc, KINDS_SRC};
use itertools::Itertools;
use quote::{format_ident, quote};

use crate::tests::sourcegen_ast::*;

// From sourcegen_ast::extract_struct_traits
const SPECIAL_ITEMS: &[(&str, &[&str])] = &[
    ("HasAttrs", &["attrs"]),
    ("HasName", &["name"]),
    ("HasVisibility", &["visibility"]),
    ("HasGenericParams", &["generic_param_list", "where_clause"]),
    ("HasTypeBounds", &["type_bound_list", "colon_token"]),
    ("HasModuleItem", &["items"]),
    ("HasLoopBody", &["label", "loop_body"]),
    ("HasArgList", &["arg_list"]),
];

const HAND_WRITTEN: &[&str] = &["BinExpr", "IfExpr", "Literal"];

const HAND_WRITTEN_PRINT_ONLY: &[&str] = &["ParamList", "ArgList"];

const LIST_AUTO_GEN_SEP_COMMA: &[&str] = &["VariantList", "RecordFieldList", "TupleFieldList"];
const LIST_AUTO_GEN_SEP_NEWLINE: &[&str] = &["StmtList"];

#[test]
fn sourcegen_vst() {
    let grammar =
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/rust.ungram")).parse().unwrap();
    let ast = lower(&grammar, true);

    let ast_nodes = generate_vst(KINDS_SRC, &ast);
    let ast_nodes_file =
        sourcegen::project_root().join("crates/syntax/src/ast/generated/vst_nodes.rs");
    sourcegen::ensure_file_contents(ast_nodes_file.as_path(), &ast_nodes);
}

pub(crate) fn generate_vst(_kinds: KindsSrc<'_>, grammar: &AstSrc) -> String {
    // TODO: add "Comment" item

    // generate struct definitions
    let node_defs: Vec<_> = grammar
        .nodes
        .iter()
        .map(|node| {
            let name = format_ident!("{}", node.name);
            let fields = node.fields.iter().map(|field| {
                let name = field.method_name();
                let ty = field.ty();

                if field.is_many() {
                    quote! {
                        pub #name : Vec<#ty>,
                    }
                } else if let Some(token_kind) = field.token_kind() {
                    // hacky for now
                    // maybe special-case identifier to "#name : Option<String>"
                    // 'ident, 'int_number', and 'lifetime_ident'.
                    if token_kind.to_string() == "T ! [ident]"
                        || token_kind.to_string() == "T ! [int_number]"
                        || token_kind.to_string() == "T ! [lifetime_ident]"
                    {
                        quote! {
                            pub #name : Option<String>,
                        }
                    } else {
                        quote! {
                            pub #name : bool,
                        }
                    }
                } else {
                    // As source code can be incomplete, we use Option even if the field is not optional in ungrammar.
                    // TODO:
                    // As source code can be incomplete, we use might use `Option` even if the field is not optional in ungrammar.
                    // instead, however, since proof action might choose to be available when syntax is complete
                    // therefore, we do not use `Option` for VST.
                    // we only use `Option` when the syntax item is optional in ungrammar.
                    if field.is_one() {
                        quote! {
                            pub #name : Box<#ty>,
                        }
                    } else {
                        quote! {
                            pub #name : Option<Box<#ty>>,
                        }
                    }
                }
            });

            if HAND_WRITTEN.contains(&node.name.as_str()) {
                quote! {}
            } else {
                quote! {
                    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
                    pub struct #name {
                        #(#fields)*
                        pub cst: Option<super::nodes::#name>,
                    }
                }
            }
        })
        .collect_vec();

    // CST -> VST
    // impl From (eventually `TryFrom` to remove all the options around every fields) for each node
    let from_node_to_vnode_struct: Vec<_> = grammar
        .nodes
        .iter()
        .map(|node| {
            let name = format_ident!("{}", node.name);
            let fields = node.fields.iter().map(|field| {
                let name = field.method_name();
                let ty = field.ty();

                if field.is_many() {
                    quote! {
                        #name : item.#name().into_iter().map(#ty::try_from).collect::<Result<Vec<#ty>, String>>()?,
                    }
                } else if let Some(token_kind) = field.token_kind() {
                    // hacky for now
                    // maybe special-case identifier to "#name : Option<String>"
                    // 'ident, 'int_number', and 'lifetime_ident'.
                    if token_kind.to_string() == "T ! [ident]"
                        || token_kind.to_string() == "T ! [int_number]"
                        || token_kind.to_string() == "T ! [lifetime_ident]"
                    {
                        // #name : Option<String>,
                        quote! {
                            #name : item.#name().map(|it| it.text().to_string()),
                        }
                    } else {
                        // #name : bool,
                        quote! {
                            #name : item.#name().is_some(),
                        }
                    }
                } else {
                    if field.is_one() {
                        // pub #name : Box<#ty>,
                        quote! {
                            #name: Box::new(item.#name().ok_or(format!("{}", stringify!(#name))).map(|it| #ty::try_from(it))??),
                        }
                    } else {                    
                        // pub #name : Option<Box<#ty>>,
                        quote! {
                            #name: match item.#name() {
                                Some(it) => Some(Box::new(#ty::try_from(it)?)),
                                None => None,
                            },
                        }
                    }
                }
            });
            if HAND_WRITTEN.contains(&node.name.as_str()) {
                quote! {
                }
            } else {
                quote! {
                    impl TryFrom<super::nodes::#name> for #name {
                        type Error = String;
                        fn try_from(item: super::nodes::#name) -> Result<Self, Self::Error>  {
                            Ok(Self {
                                #(#fields)*
                                cst: Some(item.clone()),
                            })
                        }
                    }
                }
            }
        })
        .collect_vec();

    // display struct
    let display_impls_struct: Vec<_> = grammar
        .nodes
        .iter()
        .map(|node| {
            let name = format_ident!("{}", node.name);
            let fields = node.fields.iter().map(|field| {
                let name = field.method_name();
                // let ty = field.ty();

                if field.is_many() {
                    let sep;
                    if LIST_AUTO_GEN_SEP_COMMA.contains(&node.name.as_str()) {
                        sep = ", ";
                    } else if LIST_AUTO_GEN_SEP_NEWLINE.contains(&node.name.as_str()) {
                        sep = "\n    ";
                    } else {
                        sep = " ";
                    }
                    quote! {
                        s.push_str(&self.#name.iter().map(|it| it.to_string()).collect::<Vec<String>>().join(#sep));
                    }
                } else if let Some(token_kind) = field.token_kind() {
                    // hacky for now
                    // maybe special-case identifier to "#name : Option<String>"
                    // 'ident, 'int_number', and 'lifetime_ident'.
                    if token_kind.to_string() == "T ! [ident]"
                        || token_kind.to_string() == "T ! [int_number]"
                        || token_kind.to_string() == "T ! [lifetime_ident]"
                    {
                        // #name : Option<String>,
                        quote! {
                            if let Some(it) = &self.#name {
                                s.push_str(&it); s.push_str(" ");
                            }
                        }
                    } else {
                        // #name : bool,
                        quote! {
                            if self.#name {
                                let mut tmp = stringify!(#name).to_string();
                                tmp.truncate(tmp.len() - 6);
                                s.push_str(token_ascii(&tmp));
                                s.push_str(" ");
                            }
                        }
                    }
                } else {
                    if field.is_one() {
                        // pub #name : Box<#ty>,
                        quote! {
                            s.push_str(&self.#name.to_string());
                            s.push_str(" ");
                        }
                    } else {                    
                        // pub #name : Option<Box<#ty>>,
                        quote! {
                            if let Some(it) = &self.#name {
                                s.push_str(&it.to_string()); s.push_str(" ");
                            }
                        }
                    }
                }
            });
            if HAND_WRITTEN.contains(&node.name.as_str()) || HAND_WRITTEN_PRINT_ONLY.contains(&node.name.as_str()) {
                quote! {
                }
            } else {
                quote! {
                    impl std::fmt::Display for #name {
                        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                            let mut s = String::new();
                            #(#fields)*
                            write!(f, "{s}")
                        }
                    }
                }
            }
        })
        .collect_vec();

    // generate enum definitions
    let enum_defs: Vec<_> = grammar
        .enums
        .iter()
        .map(|en| {
            let variants: Vec<_> = en.variants.iter().map(|var| format_ident!("{}", var)).collect();
            let name = format_ident!("{}", en.name);
            quote! {
                #[derive(Debug, Clone, PartialEq, Eq, Hash)]
                pub enum #name {
                    #(#variants(Box<#variants>),)*
                }
            }
        })
        .collect_vec();

    // CST to VST
    let from_node_to_vnode_enum:  Vec<_> = grammar
    .enums
    .iter()
    .map(|en| {
        let variants: Vec<_> = en.variants.iter().map(|var| format_ident!("{}", var)).collect();
        let name = format_ident!("{}", en.name);
        quote! {
            impl TryFrom<super::nodes::#name> for #name {
                type Error = String;
                fn try_from(item: super::nodes::#name) -> Result<Self, Self::Error> {
                    match item {
                        #(
                            super::nodes::#name::#variants(it) => Ok(Self::#variants(Box::new(it.try_into()?))),
                        )*
                    }
                }
            }
        }  
    })
    .collect_vec();

    // display
    let display_impls_enum: Vec<_> = grammar
        .enums
        .iter()
        .map(|en| {
            let variants: Vec<_> = en.variants.iter().map(|var| format_ident!("{}", var)).collect();
            let name = format_ident!("{}", en.name);
            quote! {
                impl std::fmt::Display for #name {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        match self {
                            #(
                                #name::#variants(it) => write!(f, "{}", it.to_string()),
                            )*
                        }
                    }
                }
            }
        })
        .collect_vec();

    // .cst impl for enum
    let get_cst_impls_enum: Vec<_> = grammar
        .enums
        .iter()
        .map(|en| {
            let name = format_ident!("{}", en.name);
            let variants: Vec<_> = en.variants.iter().map(|var| format_ident!("{}", var)).collect();
            let vars = variants.iter().map(|v| {
                if grammar.enums.iter().any(|en| en.name == v.to_string()) {
                    quote! {
                        #name::#v(it) => Some(super::nodes::#name::#v(it.cst()?.clone())),
                    }
                } else {
                    quote! {
                        #name::#v(it) => Some(super::nodes::#name::#v(it.cst.as_ref()?.clone())),
                    }
                }
            });

            quote! {
                impl #name {
                    pub fn cst(&self) -> Option<super::nodes::#name> {
                        match self {
                            #(#vars)*
                        }
                    }
                }
            }
        })
        .collect_vec();

    let ast = quote! {
        #![allow(non_snake_case)]
        use crate::{
            ast::{traits::*, vst::*},
        };

        #(#node_defs)*
        #(#enum_defs)*
        #(#from_node_to_vnode_struct)*
        #(#from_node_to_vnode_enum)*
        #(#display_impls_struct)*
        #(#display_impls_enum)*
        #(#get_cst_impls_enum)*
    };

    // TODO: expr_ext
    // this file contains manual `impl`s that are not auto-generated.
    // VST should have all corresponding `impl`s

    // VST -> CST
    // TODO: generate display impls (this is to print VST and parse into CST)
    //

    sourcegen::add_preamble("sourcegen_vst", sourcegen::reformat(ast.to_string()))
}

/*
impl Stmt {
    pub fn cst(&self) -> Option<super::nodes::Stmt> {
        match self {
            Stmt::ExprStmt(it) => Some(super::nodes::Stmt::ExprStmt(it.cst.as_ref()?.clone())),
            Stmt::Item(it) => Some(super::nodes::Stmt::Item(it.cst()?.clone())),
            Stmt::LetStmt(it) => Some(super::nodes::Stmt::LetStmt(it.cst.as_ref()?.clone())),
        }
    }
}
*/

/*
below stuff are removed in "sourcege_ast" with "remove_field"
through "extract_struct_traits"

("HasAttrs", &["attrs"]),
("HasName", &["name"]),
("HasVisibility", &["visibility"]),
("HasGenericParams", &["generic_param_list", "where_clause"]),
("HasTypeBounds", &["type_bound_list", "colon_token"]),
("HasModuleItem", &["items"]),
("HasLoopBody", &["label", "loop_body"]),
("HasArgList", &["arg_list"]),
 */

/*
impl From<super::nodes::AssertExpr> for AssertExpr {
    fn from(item: super::nodes::AssertExpr) -> Self {
        Self {
            assert_token: item.assert_token().is_some(),
            l_paren_token: item.l_paren_token().is_some(),
            expr: item.expr().map(Expr::from).map(Box::new),
            r_paren_token: item.r_paren_token().is_some(),
            by_token: item.by_token().is_some(),
            requires_clause: item.requires_clause().map(RequiresClause::from).map(Box::new),
            block_expr: item.block_expr().map(BlockExpr::from).map(Box::new),
        }
    }
}

impl From<super::nodes::Name> for Name {
    fn from(item: super::nodes::Name) -> Self {
        Self {
            ident_token: item.ident_token().map(|it| it.text().to_string()),
            self_token: item.self_token().is_some(),
        }
    }
}

impl TryFrom<super::nodes::AssertExpr> for AssertExpr {
    type Error = ();

    fn try_from(item: super::nodes::AssertExpr) -> Result<Self, Self::Error> {
        let res = Self {
            assert_token: item.assert_token().is_some(),
            l_paren_token: item.l_paren_token().is_some(),
            expr: Some(Box::new(item.expr().try_into()?)),
            r_paren_token: item.r_paren_token().is_some(),
            by_token: item.by_token().is_some(),
            requires_clause: item.requires_clause().map(RequiresClause::try_from).map(Box::new),
            block_expr: item.block_expr.map(Box::new),
        };
        Ok(res)
    }
}
 */

/*



impl TryFrom<super::nodes::Attr> for Attr {


    type Error = ();
    fn try_from(item: super::nodes::Attr) -> Result<Self, Self::Error> {
    }
}

impl TryFrom<super::nodes::Expr> for Expr {


    type Error = ();
    fn try_from(item: super::nodes::Expr) -> Result<Self, Self::Error> {
    }
}
impl TryFrom<super::nodes::Name> for Name {


    type Error = ();
    fn try_from(item: super::nodes::Name) -> Result<Self, Self::Error> {
    }
}
impl TryFrom<super::nodes::RequiresClause> for RequiresClause {


    type Error = ();
    fn try_from(item: super::nodes::RequiresClause) -> Result<Self, Self::Error> {
    }
}
// pub struct AssertExpr {
//     pub attrs: Vec<Attr>,
//     assert_token: bool,
//     l_paren_token: bool,
//     pub expr: Box<Expr>,
//     r_paren_token: bool,
//     by_token: bool,
//     pub name: Option<Box<Name>>,
//     pub requires_clause: Option<Box<RequiresClause>>,
//     pub block_expr: Option<Box<BlockExpr>>,
// }
impl TryFrom<super::nodes::AssertExpr> for AssertExpr {
    type Error = ();

    fn try_from(item: super::nodes::AssertExpr) -> Result<Self, Self::Error> {
        let attrs = item.attrs().into_iter().map(Attr::try_from).collect()?;
        let assert_token = item.assert_token().is_some();
        let l_paren_token = item.l_paren_token().is_some();
        let expr = Box::new(item.expr().ok_or(()).map(|it| Expr::try_from(it))??);
        let r_paren_token= item.r_paren_token().is_some();
        let by_token = item.by_token().is_some();
        let name = match item.name() {
            Some(it) => Some(Box::new(Name::try_from(it)?)),
            None => None,
        };
        let requires_clause = match item.requires_clause() {
            Some(it) => Some(Box::new(RequiresClause::try_from(it)?)),
            None => None,
        };
        let block_expr = match item.block_expr() {
            Some(it) => Some(Box::new(BlockExpr::try_from(it)?)),
            None => None,
        };
        Ok (Self {
            attrs,
            assert_token,
            l_paren_token,
            expr,
            r_paren_token,
            by_token,
            name,
            requires_clause,
            block_expr,
        })
    }
}
 */
