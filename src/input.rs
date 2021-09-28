use core::panic;
use std::env::Args;

pub struct Parameter {
    pub target: String,
}

impl Parameter {
    pub fn parse(args: &mut Args) -> Option<Parameter> {
        let maybe_target = &args.nth(1);
        match maybe_target {
            Some(target) => Some(Parameter { target: target.to_string() }),
            None => None
        }
    }

    pub fn unsafe_parse(args: &mut Args) -> Parameter {
        match Parameter::parse(args) {
            Some(param) => param,
            None => panic!("invalid parameter")
        }
    }
}