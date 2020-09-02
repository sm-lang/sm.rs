mod can_parse;
mod config;
mod regroup;

pub use crate::parser::{can_parse::CanParse, config::ParserConfig};
use crate::ParserResult;
use sdl_ast::{ASTKind, Template, AST};
use sdl_pest::{Assoc, Operator, Pair, Pairs, Parser, PrecClimber, Rule, SDLParser};
use std::lazy::SyncLazy;
use url::Url;

macro_rules! debug_cases {
    ($i:ident) => {{
        println!("Rule::{:?}=>continue,", $i.as_rule());
        println!("Span: {:?}", $i.as_span());
        println!("Text: {}", $i.as_str());
        unreachable!();
    }};
}

#[rustfmt::skip]
pub static PREC_CLIMBER: SyncLazy<PrecClimber<Rule>> = SyncLazy::new(|| {
    use Rule::*;
    use Assoc::*;
    //TODO: use macro
    PrecClimber::new(vec![
        Operator::new(Set, Left),
        Operator::new(Plus, Left) | Operator::new(Minus, Left),
        Operator::new(Power, Right),
        Operator::new(Dot, Left)
    ])
});

impl ParserConfig {
    pub fn parse(&mut self, input: impl CanParse) -> ParserResult<AST> {
        if let Some(s) = input.as_url() {
            self.file_url = Some(s)
        }
        let input = input.as_text()?.replace("\r\n", "\n").replace("\\\n", "").replace("\t", &" ".repeat(self.tab_size));
        Ok(self.parse_program(SDLParser::parse(Rule::program, &input)?))
    }
    fn parse_program(&self, pairs: Pairs<Rule>) -> AST {
        let mut codes = vec![];
        for pair in pairs {
            if let Rule::statement = pair.as_rule() {
                codes.push(self.parse_statement(pair));
            };
        }
        AST::program(codes)
    }
    fn parse_statement(&self, pairs: Pair<Rule>) -> AST {
        let r = self.get_position(pairs.as_span());
        let mut codes = vec![];
        for pair in pairs.into_inner() {
            let code = match pair.as_rule() {
                Rule::expression => self.parse_expression(pair),
                _ => debug_cases!(pair),
            };
            codes.push(code);
        }
        AST::statement(codes, r)
    }
    fn parse_expression(&self, pairs: Pair<Rule>) -> AST {
        let r = self.get_position(pairs.as_span());
        let mut codes = vec![];
        let mut eos = false;
        for pair in pairs.into_inner() {
            let code = match pair.as_rule() {
                Rule::EOI => continue,
                Rule::WHITESPACE => continue,
                Rule::expr => self.parse_expr(pair),
                _ => debug_cases!(pair),
            };
            codes.push(code);
        }
        AST::expression(codes, eos, r)
    }

    #[rustfmt::skip]
    fn parse_expr(&self, pairs: Pair<Rule>) -> AST {
        let r = self.get_position(pairs.as_span());
        PREC_CLIMBER.climb(
            pairs.into_inner(),
            |pair: Pair<Rule>| match pair.as_rule() {
                //Rule::expr => self.parse_expr(pair),
                Rule::term => self.parse_term(pair),
                _ => debug_cases!(pair),
            },
            |left: AST, op: Pair<Rule>, right: AST| match op.as_rule() {
                _ => AST::infix(op.as_str(), left, right, r),
            },
        )
    }

    fn parse_term(&self, pairs: Pair<Rule>) -> AST {
        // let pos = get_position(pairs.as_span());
        let mut base = AST::default();
        // let mut prefix = vec![];
        // let mut suffix = vec![];
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::WHITESPACE => continue,
                Rule::node => base = self.parse_node(pair),
                //       Rule::Prefix => prefix.push(pair.as_str().to_string()),
                //       Rule::Suffix => suffix.push(pair.as_str().to_string()),
                _ => unreachable!(),
            };
        }
        return base;
    }

    fn parse_node(&self, pairs: Pair<Rule>) -> AST {
        let pair = pairs.into_inner().nth(0).unwrap();
        match pair.as_rule() {
            Rule::data => self.parse_data(pair),
            _ => debug_cases!(pair),
        }
    }
}

impl ParserConfig {
    fn parse_data(&self, pairs: Pair<Rule>) -> AST {
        let pair = pairs.into_inner().nth(0).unwrap();
        match pair.as_rule() {
            Rule::template => self.parse_template(pair),
            Rule::Number => self.parse_number(pair),
            _ => debug_cases!(pair),
        }
    }
    fn parse_template(&self, pairs: Pair<Rule>) -> AST {
        let r = self.get_position(pairs.as_span());
        let mut template = Template::default();
        let mut symbols = vec![];
        let mut attributes = vec![];
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::SelfClose => {
                    for inner in pair.into_inner() {
                        match inner.as_rule() {
                            Rule::WHITESPACE=>continue,
                            Rule::Symbol => symbols.push(self.parse_symbol(inner)),
                            Rule::SYMBOL => attributes.push(self.parse_symbol(inner)),
                            _ => debug_cases!(inner),
                        };
                    }
                    template = Template::self_close(symbols.first().unwrap().to_owned())
                }
                Rule::HTMLBad => {
                    for inner in pair.into_inner() {
                        match inner.as_rule() {
                            Rule::WHITESPACE=>continue,
                            Rule::HTMLBadSymbol => symbols.push(self.parse_symbol(inner)),
                            Rule::SYMBOL=>attributes.push(self.parse_string(inner)),
                            _ => debug_cases!(inner),
                        };
                    }
                    template = Template::html_bad(symbols.first().unwrap().to_owned())
                }
                _ => debug_cases!(pair),
            };
        }
        template.set_attributes(attributes);

        return AST::template(template, r);
    }
    fn parse_symbol(&self, pairs: Pair<Rule>) -> AST {
        let r = self.get_position(pairs.as_span());
        AST::string(pairs.as_str().to_string(), r)
    }

    fn parse_string(&self, pairs: Pair<Rule>) -> AST {
        let r = self.get_position(pairs.as_span());
        AST::string(pairs.as_str().to_string(), r)
    }

    fn parse_number(&self, pairs: Pair<Rule>) -> AST {
        let r = self.get_position(pairs.as_span());
        AST::string(pairs.as_str().to_string(), r)
    }
}
