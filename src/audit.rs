//! Syntax rules. They read one file with no type information: see docs/REGLAS.md for their limits.

use proc_macro2::{Span, TokenStream, TokenTree};
use std::collections::BTreeMap;
use syn::spanned::Spanned;
use syn::visit::{self, Visit};
use syn::{
    Attribute, BinOp, Expr, ExprAssign, ExprBinary, ExprClosure, ExprPath, ExprReference, FnArg,
    ForeignItemStatic, Ident, ImplItemFn, Item, ItemExternCrate, ItemFn, ItemStatic, ItemUse,
    Macro, Meta, PatIdent, Signature, TraitItemFn, UseTree, Visibility,
};

/// Lints that Clippy and the compiler enforce on behalf of the harness.
pub const ENFORCED_LINTS: &[&str] = &[
    "clippy::disallowed_types",
    "clippy::disallowed_methods",
    "clippy::disallowed_macros",
    "clippy::print_stdout",
    "clippy::print_stderr",
    "clippy::exit",
    "clippy::mem_forget",
    "clippy::needless_pass_by_ref_mut",
    "clippy::too_many_arguments",
    "clippy::shadow_same",
    "clippy::shadow_reuse",
    "clippy::shadow_unrelated",
    "clippy::undocumented_unsafe_blocks",
    "unsafe_code",
];
/// The enforced lints that the entry point may relax, because it is where the outside is obtained.
const ENTRY_POINT_LINTS: &[&str] = &[
    "clippy::disallowed_methods",
    "clippy::disallowed_macros",
    "clippy::print_stdout",
    "clippy::print_stderr",
];
/// Groups that contain enforced lints, so relaxing one relaxes those too.
const LINT_GROUPS: &[&str] = &[
    "warnings",
    "clippy::all",
    "clippy::style",
    "clippy::complexity",
    "clippy::nursery",
    "clippy::restriction",
];
const RELAXING_ATTRIBUTES: &[&str] = &["allow", "expect", "warn", "cfg_attr"];
/// Paths that reach outside the program: a module with all it holds, or a single item.
const WORLD_PATHS: &[&str] = &[
    "std::env",
    "std::fs",
    "std::net",
    "std::os",
    "std::process",
    "std::io::stdin",
    "std::io::stdout",
    "std::io::stderr",
    "std::time::Instant::now",
    "std::time::SystemTime::now",
];
const WORLD_MACROS: &[&str] = &["print", "println", "eprint", "eprintln", "dbg"];
const MAX_ALIAS_HOPS: usize = 8;

pub struct Finding {
    pub rule: &'static str,
    pub message: String,
    pub line: usize,
    pub column: usize,
}

pub fn audit_source(file_name: &str, source: &str) -> Vec<Finding> {
    let file = match syn::parse_file(source) {
        Ok(parsed) => parsed,
        Err(error) => {
            let start = error.span().start();
            return vec![Finding {
                rule: "PARSE",
                message: error.to_string(),
                line: start.line,
                column: start.column + 1,
            }];
        }
    };
    let mut collector = AliasCollector::default();
    collector.visit_file(&file);
    let mut auditor = Auditor {
        aliases: collector.aliases,
        findings: Vec::new(),
        in_entry_point: false,
    };
    for attribute in &file.attrs {
        auditor.visit_attribute(attribute);
    }
    for item in &file.items {
        auditor.in_entry_point = file_name != "lib.rs" && is_entry_point(item);
        auditor.visit_item(item);
    }
    auditor.findings
}

fn is_entry_point(item: &Item) -> bool {
    match item {
        Item::Fn(function) => {
            function.sig.ident == "main"
                && function.sig.inputs.is_empty()
                && matches!(function.vis, Visibility::Inherited)
        }
        _ => false,
    }
}

struct UseLeaf {
    path: Vec<String>,
    alias: Option<String>,
    is_glob: bool,
}

fn use_leaves(tree: &UseTree, prefix: &[String], leaves: &mut Vec<UseLeaf>) {
    match tree {
        UseTree::Path(part) => {
            let longer = extended(prefix, &part.ident);
            use_leaves(&part.tree, &longer, leaves);
        }
        UseTree::Name(name) => {
            let path = named_path(prefix, &name.ident);
            let alias = path.last().cloned();
            leaves.push(UseLeaf {
                path,
                alias,
                is_glob: false,
            });
        }
        UseTree::Rename(rename) => leaves.push(UseLeaf {
            path: named_path(prefix, &rename.ident),
            alias: Some(rename.rename.to_string()),
            is_glob: false,
        }),
        UseTree::Glob(_) => leaves.push(UseLeaf {
            path: prefix.to_vec(),
            alias: None,
            is_glob: true,
        }),
        UseTree::Group(group) => {
            for member in &group.items {
                use_leaves(member, prefix, leaves);
            }
        }
    }
}

fn extended(prefix: &[String], ident: &Ident) -> Vec<String> {
    let mut path = prefix.to_vec();
    path.push(ident.to_string());
    path
}

fn named_path(prefix: &[String], ident: &Ident) -> Vec<String> {
    if ident == "self" {
        prefix.to_vec()
    } else {
        extended(prefix, ident)
    }
}

#[derive(Default)]
struct AliasCollector {
    aliases: BTreeMap<String, Vec<String>>,
}

impl<'ast> Visit<'ast> for AliasCollector {
    fn visit_item_use(&mut self, node: &'ast ItemUse) {
        let mut leaves = Vec::new();
        use_leaves(&node.tree, &[], &mut leaves);
        for leaf in leaves {
            if let Some(alias) = leaf.alias {
                self.aliases.insert(alias, leaf.path);
            }
        }
    }

    fn visit_item_extern_crate(&mut self, node: &'ast ItemExternCrate) {
        if let Some((_, rename)) = &node.rename {
            self.aliases
                .insert(rename.to_string(), vec![node.ident.to_string()]);
        }
    }

    fn visit_macro(&mut self, _node: &'ast Macro) {}
}

fn resolve(aliases: &BTreeMap<String, Vec<String>>, segments: &[String]) -> String {
    let mut resolved = segments.to_vec();
    for _ in 0..MAX_ALIAS_HOPS {
        let target = resolved.first().and_then(|head| aliases.get(head));
        match target {
            Some(path) if path.first() != resolved.first() => {
                let mut longer = path.clone();
                longer.extend_from_slice(&resolved[1..]);
                resolved = longer;
            }
            _ => break,
        }
    }
    resolved.join("::")
}

fn is_world(path: &str) -> bool {
    WORLD_PATHS
        .iter()
        .any(|world| path == *world || path.starts_with(&format!("{world}::")))
}

fn is_world_ancestor(path: &str) -> bool {
    WORLD_PATHS
        .iter()
        .any(|world| world.starts_with(&format!("{path}::")))
}

fn path_segments(path: &syn::Path) -> Vec<String> {
    path.segments
        .iter()
        .map(|segment| segment.ident.to_string())
        .collect()
}

struct Auditor {
    aliases: BTreeMap<String, Vec<String>>,
    findings: Vec<Finding>,
    in_entry_point: bool,
}

impl Auditor {
    fn report(&mut self, rule: &'static str, span: Span, message: String) {
        let start = span.start();
        self.findings.push(Finding {
            rule,
            message,
            line: start.line,
            column: start.column + 1,
        });
    }

    fn report_static(&mut self, span: Span) {
        self.report(
            "STATE",
            span,
            "static item: only constants may be global, use `const`".to_string(),
        );
    }

    fn report_world(&mut self, span: Span, path: &str) {
        self.report(
            "WORLD",
            span,
            format!("`{path}` reaches the outside: obtain it in `main` and pass it as a parameter"),
        );
    }

    fn check_world_path(&mut self, span: Span, segments: &[String]) {
        if self.in_entry_point {
            return;
        }
        let resolved = resolve(&self.aliases, segments);
        if is_world(&resolved) {
            self.report_world(span, &resolved);
        }
    }

    fn check_use_leaf(&mut self, span: Span, leaf: &UseLeaf, is_public: bool) {
        let resolved = resolve(&self.aliases, &leaf.path);
        let rejected = if leaf.is_glob {
            is_world(&resolved) || is_world_ancestor(&resolved)
        } else {
            let is_type = leaf
                .path
                .last()
                .is_some_and(|name| name.starts_with(char::is_uppercase));
            is_world(&resolved) && (is_public || !is_type)
        };
        if rejected {
            self.report_world(span, &resolved);
        }
    }

    /// Looks inside macro input, which the parser leaves as plain tokens.
    fn scan_tokens(&mut self, tokens: TokenStream) {
        let trees: Vec<TokenTree> = tokens.into_iter().collect();
        let mut index = 0;
        let mut after_quote = false;
        while index < trees.len() {
            let mut consumed = 1;
            let mut is_quote = false;
            match &trees[index] {
                TokenTree::Group(group) => self.scan_tokens(group.stream()),
                TokenTree::Punct(punct) => is_quote = punct.as_char() == '\'',
                TokenTree::Literal(_) => {}
                TokenTree::Ident(ident) => {
                    // `'static` arrives as a quote and then an identifier.
                    if ident == "static" && !after_quote {
                        self.report_static(ident.span());
                    }
                    let segments = token_path(&trees[index..]);
                    self.check_world_path(ident.span(), &segments);
                    consumed = segments.len() * 3 - 2;
                }
            }
            after_quote = is_quote;
            index += consumed;
        }
    }

    fn check_attribute_lints(&mut self, attribute: &Attribute) {
        let Meta::List(list) = &attribute.meta else {
            return;
        };
        let text = list.tokens.to_string().replace(' ', "");
        let names = text.split(|character: char| {
            !(character.is_alphanumeric() || character == '_' || character == ':')
        });
        for name in names {
            let relaxes = ENFORCED_LINTS.contains(&name) || LINT_GROUPS.contains(&name);
            let excused = self.in_entry_point && ENTRY_POINT_LINTS.contains(&name);
            if relaxes && !excused {
                self.report(
                    "BYPASS",
                    attribute.span(),
                    format!("`{name}` is a rule of the harness and cannot be relaxed here"),
                );
            }
        }
    }

    fn check_names(&mut self, signature: &Signature, block: &syn::Block, is_free: bool) {
        let mut parameters = Names::default();
        for input in &signature.inputs {
            if let FnArg::Typed(typed) = input {
                parameters.visit_pat(&typed.pat);
            }
        }
        let mut names = Names::default();
        names.visit_block(block);
        if is_free {
            for parameter in &parameters.bindings {
                if *parameter == signature.ident {
                    self.report(
                        "NAME",
                        parameter.span(),
                        format!("parameter `{parameter}` has the name of its function"),
                    );
                }
            }
        }
        for binding in parameters.bindings.iter().chain(&names.bindings) {
            if names.functions.contains(binding) {
                self.report(
                    "NAME",
                    binding.span(),
                    format!(
                        "`{binding}` names a variable and a function inside `{}`",
                        signature.ident
                    ),
                );
            }
        }
    }

    fn check_closure_writes(&mut self, closure: &ExprClosure) {
        let mut declared = Names::default();
        for input in &closure.inputs {
            declared.visit_pat(input);
        }
        declared.visit_expr(&closure.body);
        let mut writes = Writes::default();
        writes.visit_expr(&closure.body);
        for written in &writes.roots {
            if !declared.bindings.contains(written) {
                self.report(
                    "LAMBDA",
                    written.span(),
                    format!("closure writes `{written}`, a name it did not declare"),
                );
            }
        }
    }
}

/// Reads `a :: b :: c` from the start of the slice, which must begin with an identifier.
fn token_path(trees: &[TokenTree]) -> Vec<String> {
    let mut segments = Vec::new();
    let mut index = 0;
    while let Some(TokenTree::Ident(ident)) = trees.get(index) {
        segments.push(ident.to_string());
        let is_separator = matches!(trees.get(index + 1), Some(TokenTree::Punct(first)) if first.as_char() == ':')
            && matches!(trees.get(index + 2), Some(TokenTree::Punct(second)) if second.as_char() == ':')
            && matches!(trees.get(index + 3), Some(TokenTree::Ident(_)));
        if !is_separator {
            break;
        }
        index += 3;
    }
    segments
}

impl<'ast> Visit<'ast> for Auditor {
    fn visit_attribute(&mut self, node: &'ast Attribute) {
        let relaxing = node
            .path()
            .segments
            .last()
            .is_some_and(|last| RELAXING_ATTRIBUTES.contains(&last.ident.to_string().as_str()));
        if relaxing {
            self.check_attribute_lints(node);
        }
    }

    fn visit_item_static(&mut self, node: &'ast ItemStatic) {
        // The language accepts a global allocator only as a static, so `const` is no alternative.
        let is_allocator = node
            .attrs
            .iter()
            .any(|attribute| attribute.path().is_ident("global_allocator"));
        if !is_allocator {
            self.report_static(node.static_token.span());
        }
        visit::visit_item_static(self, node);
    }

    fn visit_foreign_item_static(&mut self, node: &'ast ForeignItemStatic) {
        self.report_static(node.static_token.span());
        visit::visit_foreign_item_static(self, node);
    }

    fn visit_item_use(&mut self, node: &'ast ItemUse) {
        if self.in_entry_point {
            return;
        }
        let is_public = !matches!(node.vis, Visibility::Inherited);
        let mut leaves = Vec::new();
        use_leaves(&node.tree, &[], &mut leaves);
        for leaf in &leaves {
            self.check_use_leaf(node.span(), leaf, is_public);
        }
    }

    fn visit_expr_path(&mut self, node: &'ast ExprPath) {
        self.check_world_path(node.span(), &path_segments(&node.path));
        visit::visit_expr_path(self, node);
    }

    fn visit_macro(&mut self, node: &'ast Macro) {
        let named = node
            .path
            .segments
            .last()
            .is_some_and(|last| WORLD_MACROS.contains(&last.ident.to_string().as_str()));
        if named && !self.in_entry_point {
            self.report_world(node.span(), &path_segments(&node.path).join("::"));
        }
        self.scan_tokens(node.tokens.clone());
    }

    fn visit_item_fn(&mut self, node: &'ast ItemFn) {
        self.check_names(&node.sig, &node.block, true);
        visit::visit_item_fn(self, node);
    }

    fn visit_impl_item_fn(&mut self, node: &'ast ImplItemFn) {
        self.check_names(&node.sig, &node.block, false);
        visit::visit_impl_item_fn(self, node);
    }

    fn visit_trait_item_fn(&mut self, node: &'ast TraitItemFn) {
        if let Some(block) = &node.default {
            self.check_names(&node.sig, block, false);
        }
        visit::visit_trait_item_fn(self, node);
    }

    fn visit_expr_closure(&mut self, node: &'ast ExprClosure) {
        self.check_closure_writes(node);
        visit::visit_expr_closure(self, node);
    }
}

/// The names one strong environment holds. Nested items are environments of their own.
#[derive(Default)]
struct Names {
    bindings: Vec<Ident>,
    functions: Vec<Ident>,
}

impl<'ast> Visit<'ast> for Names {
    fn visit_pat_ident(&mut self, node: &'ast PatIdent) {
        self.bindings.push(node.ident.clone());
        visit::visit_pat_ident(self, node);
    }

    fn visit_item(&mut self, node: &'ast Item) {
        if let Item::Fn(function) = node {
            self.functions.push(function.sig.ident.clone());
        }
    }
}

/// The names that one closure writes. A nested closure answers for its own writes.
#[derive(Default)]
struct Writes {
    roots: Vec<Ident>,
}

impl Writes {
    fn record(&mut self, target: &Expr) {
        if let Some(root) = written_root(target) {
            self.roots.push(root.clone());
        }
    }
}

impl<'ast> Visit<'ast> for Writes {
    fn visit_expr_assign(&mut self, node: &'ast ExprAssign) {
        self.record(&node.left);
        visit::visit_expr_assign(self, node);
    }

    fn visit_expr_binary(&mut self, node: &'ast ExprBinary) {
        if is_compound_assignment(&node.op) {
            self.record(&node.left);
        }
        visit::visit_expr_binary(self, node);
    }

    fn visit_expr_reference(&mut self, node: &'ast ExprReference) {
        if node.mutability.is_some() {
            self.record(&node.expr);
        }
        visit::visit_expr_reference(self, node);
    }

    fn visit_expr_closure(&mut self, _node: &'ast ExprClosure) {}

    fn visit_item(&mut self, _node: &'ast Item) {}
}

fn is_compound_assignment(operator: &BinOp) -> bool {
    matches!(
        operator,
        BinOp::AddAssign(_)
            | BinOp::SubAssign(_)
            | BinOp::MulAssign(_)
            | BinOp::DivAssign(_)
            | BinOp::RemAssign(_)
            | BinOp::BitXorAssign(_)
            | BinOp::BitAndAssign(_)
            | BinOp::BitOrAssign(_)
            | BinOp::ShlAssign(_)
            | BinOp::ShrAssign(_)
    )
}

/// The variable at the base of a place such as `*a.b[0].c`.
fn written_root(target: &Expr) -> Option<&Ident> {
    match target {
        Expr::Path(place) => place.path.get_ident(),
        Expr::Field(place) => written_root(&place.base),
        Expr::Index(place) => written_root(&place.expr),
        Expr::Paren(place) => written_root(&place.expr),
        Expr::Group(place) => written_root(&place.expr),
        Expr::Unary(place) => written_root(&place.expr),
        Expr::Reference(place) => written_root(&place.expr),
        Expr::MethodCall(place) => written_root(&place.receiver),
        _ => None,
    }
}
