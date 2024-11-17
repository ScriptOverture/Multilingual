use std::fs::read_to_string;
use swc_common::BytePos;
use swc_ecma_parser::{Parser, StringInput, Syntax, TsSyntax};
use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitWith};

pub struct LanguageNode {
    pub nodes: Vec<String>,
}

impl LanguageNode {
    fn new() -> Self {
       Self::default()
    }
}

impl Default for LanguageNode {
    fn default() -> Self {
        LanguageNode {
            nodes: Vec::new(),
        }
    }
}


impl Visit for LanguageNode {
    
}


pub struct LanguageParse {
    path: String,
    language: LanguageNode
}

impl LanguageParse {
    pub fn new(
        path: String,
        language: LanguageNode
    ) -> Self {
        Self {
            path,
            language
        }
    }

    pub fn run(&mut self) -> anyhow::Result<Module> {
        let module = self.get_module()?;
        module.visit_with(&mut self.language);
        Ok(module)
    }

    fn get_module(&self) -> anyhow::Result<Module> {
        let path = self.path.clone();
        let content = read_to_string(&path)?;

        let ts_syntax = TsSyntax {
            tsx: true,
            ..Default::default()
        };

        let mut parser = Parser::new(
            Syntax::Typescript(ts_syntax), 
            StringInput::new(
                    &content, 
                    BytePos(0),
                    BytePos(content.len() as u32)
                ),
             None
        );

        parser.parse_module().map_err(|err| anyhow::anyhow!("{:?}", err))
    }
}