use std::path::PathBuf;

use etrace::some_or;
use rustc_ast::*;
use rustc_ast_pretty::pprust;
use rustc_middle::ty::TyCtxt;
use rustc_parse::parser::{AttemptLocalParseRecovery, ForceCollect, Parser};
use rustc_session::parse::ParseSess;
use rustc_span::{FileName, RealFileName};

#[derive(Debug)]
pub struct TransformationResult(pub Vec<(PathBuf, String)>);

impl TransformationResult {
    pub fn apply(&self) {
        for (path, content) in &self.0 {
            std::fs::write(path, content).unwrap();
        }
    }
}

pub fn transform_ast<F: std::ops::FnMut(&mut Crate) -> bool>(
    mut f: F,
    tcx: TyCtxt<'_>,
) -> TransformationResult {
    tcx.resolver_for_lowering();

    let source_map = tcx.sess.source_map();
    let parse_sess = new_parse_sess();

    let mut v = vec![];
    for file in source_map.files().iter() {
        let FileName::Real(RealFileName::LocalPath(p)) = &file.name else { continue };
        let src = some_or!(file.src.as_ref(), continue);
        let mut parser = rustc_parse::new_parser_from_source_str(
            &parse_sess,
            file.name.clone(),
            src.to_string(),
        )
        .unwrap();
        let mut krate = parser.parse_crate_mod().unwrap();
        if f(&mut krate) {
            let s = pprust::crate_to_string_for_macros(&krate);
            v.push((p.clone(), s));
        }
    }
    TransformationResult(v)
}

#[inline]
pub fn new_parse_sess() -> ParseSess {
    ParseSess::with_fatal_emitter(vec![], "".to_string())
}

#[inline]
pub fn new_parser_from_str(parse_sess: &ParseSess, s: String) -> Parser<'_> {
    let file_name = FileName::Custom("main.rs".to_string());
    rustc_parse::new_parser_from_source_str(parse_sess, file_name, s).unwrap()
}

#[inline]
pub fn parse_item(item: String) -> Item {
    let parse_sess = new_parse_sess();
    let mut parser = new_parser_from_str(&parse_sess, item);
    *parser.parse_item(ForceCollect::No).unwrap().unwrap()
}

#[macro_export]
macro_rules! item {
    ($($arg:tt)*) => {{
        $crate::ast_util::parse_item(format!($($arg)*))
    }};
}

pub fn parse_ty_param(param: String) -> GenericParam {
    let item = item!("fn f<{}>() {{}}", param);
    let ItemKind::Fn(box mut f) = item.kind else { panic!() };
    f.generics.params.pop().unwrap()
}

#[macro_export]
macro_rules! ty_param {
    ($($arg:tt)*) => {{
        $crate::ast_util::parse_ty_param(format!($($arg)*))
    }};
}

pub fn parse_param(param: String) -> Param {
    let item = item!("fn f({}) {{}}", param);
    let ItemKind::Fn(box mut f) = item.kind else { panic!() };
    f.sig.decl.inputs.pop().unwrap()
}

#[macro_export]
macro_rules! param {
    ($($arg:tt)*) => {{
        $crate::ast_util::parse_param(format!($($arg)*))
    }};
}

#[inline]
pub fn parse_stmt(stmt: String) -> Stmt {
    let parse_sess = new_parse_sess();
    let mut parser = new_parser_from_str(&parse_sess, stmt);
    parser
        .parse_full_stmt(AttemptLocalParseRecovery::No)
        .unwrap()
        .unwrap()
}

#[macro_export]
macro_rules! stmt {
    ($($arg:tt)*) => {{
        $crate::ast_util::parse_stmt(format!($($arg)*))
    }};
}

#[inline]
pub fn parse_expr(expr: String) -> Expr {
    let parse_sess = new_parse_sess();
    let mut parser = new_parser_from_str(&parse_sess, expr);
    *parser.parse_expr().unwrap()
}

#[macro_export]
macro_rules! expr {
    ($($arg:tt)*) => {{
        $crate::ast_util::parse_expr(format!($($arg)*))
    }};
}

pub fn parse_path(path: String) -> Path {
    let ExprKind::Path(_, path) = parse_expr(path).kind else { panic!() };
    path
}

#[macro_export]
macro_rules! path {
    ($($arg:tt)*) => {{
        $crate::ast_util::parse_path(format!($($arg)*))
    }};
}

#[inline]
pub fn parse_ty(ty: String) -> Ty {
    let parse_sess = new_parse_sess();
    let mut parser = new_parser_from_str(&parse_sess, ty);
    *parser.parse_ty().unwrap()
}

#[macro_export]
macro_rules! ty {
    ($($arg:tt)*) => {{
        $crate::ast_util::parse_ty(format!($($arg)*))
    }};
}
