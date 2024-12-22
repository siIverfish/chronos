use pest::Parser;
use pest_derive::Parser;
use pest::error::Error;
use pest::iterators::Pair;


use crate::ast::{AST, AST::*, TData::*, TAbstract::*};

#[derive(Parser)]
#[grammar = "parse.pest"]
struct ChronosParser;

pub fn parse_chronos(input: &str) -> Result<AST, Error<Rule>> {
    let pairs = ChronosParser::parse(Rule::application, input)?.next().unwrap();

    println!("{pairs:#?}");

    fn parse(pair: Pair<'_, Rule>) -> Result<AST, Error<Rule>> {
        match pair.as_rule() {
            Rule::application => {
                let mut pairs = pair.into_inner();
                let f   = Box::new(parse(pairs.next().unwrap())?);
                let arg = Box::new(parse(pairs.next().unwrap())?);
                Ok(Abstract(Application { f, arg }))
            },
            Rule::name => Ok(Abstract(Name(pair.as_span().as_str().into()))),
            Rule::string => Ok(Data(String(pair.into_inner().next().unwrap().as_span().as_str().into()))),
            Rule::number => Ok(Data(Number(pair.as_span().as_str().parse().unwrap()))),
            Rule::two => {
                let mut pairs = pair.into_inner();
                if let Some(first_element) = pairs.next() {
                    let second_element = parse(pairs.next().unwrap())?;
                    let first_element = parse(first_element)?;
                    Ok(Abstract(TwoAST(Box::new(first_element), Box::new(second_element))))
                } else {
                    Ok(Data(Nil))
                }
            }

            Rule::data 
            | Rule::expr 
            => parse(pair.into_inner().next().unwrap()), // TODO: make separate data parse function
            
            Rule::WHITESPACE 
            | Rule::inner_string 
            | Rule::name_character 
            => unreachable!(),
        }
    }

    parse(pairs)
}