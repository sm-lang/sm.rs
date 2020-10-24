use super::*;
use crate::ast::{TemplateSimplified, TemplateKind};

impl AST {
    pub fn render(&self, ctx: &mut Context) -> Result<String> {
        Ok(self.kind.render(ctx)?)
    }
}

impl ASTKind {
    pub fn render(&self, ctx: &mut Context) -> Result<String> {
        let result = match self {
            ASTKind::Program(v)|ASTKind::Statement(v) => {
                render_vec_ast(v,ctx)?
            }

            ASTKind::TemplateSimplified(inner) => inner.render(ctx)?,

            ASTKind::Boolean(v) => v.to_string(),
            ASTKind::String(v) => v.to_owned(),

            _ => unimplemented!("ASTKind::{:#?}", self),
        };
        Ok(result)
    }
}

impl TemplateSimplified {
    pub fn render(&self, ctx: &mut Context) -> Result<String> {
        let tag = match &self.tag {
            Some(s) => {s.to_owned()},
            None => {"Fragment".to_string()}
        };

        let result = match self.kind {
            TemplateKind::OpenCloseTemplate => {
                format!("<{tag}> </{tag}>", tag=tag)
            }
            TemplateKind::HTMLBadTemplate => {
                format!("<{tag}>", tag=tag)
            }
            _ => unreachable!()
        };
        Ok(result)
    }
}


pub fn render_vec_ast(v: &[AST], ctx: &mut Context) -> Result<String> {
    let mut out = String::new();
    for e in v {
        out.push_str(&e.render(ctx)?)
    }
    Ok(out)
}