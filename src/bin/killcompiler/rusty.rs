use std::sync::LazyLock;
use crate::toki::{AbstractToken, BlueprintToken, MetaToken, Operators, Token};

#[derive(Debug, Copy, Clone)]
pub enum LineType {
    Assignment,
    Reassignment,
    Keyword
}

static VALID_BLUEPRINTS: LazyLock<[(Vec<BlueprintToken>, LineType); 5]> = LazyLock::new(|| {
    [
        (vec![BlueprintToken::Abstract{ token: AbstractToken::Type }, BlueprintToken::Abstract{ token: AbstractToken::Identification }, BlueprintToken::EqualsToken, BlueprintToken::ValueToken], LineType::Assignment),
        (vec![BlueprintToken::Abstract{ token: AbstractToken::Type }, BlueprintToken::Abstract{ token: AbstractToken::Identification }, BlueprintToken::EqualsToken, BlueprintToken::ValueToken], LineType::Assignment),
        (vec![BlueprintToken::Abstract{ token: AbstractToken::Identification }, BlueprintToken::EqualsToken, BlueprintToken::ExpressionToken], LineType::Reassignment),
        (vec![BlueprintToken::Abstract{ token: AbstractToken::Identification }, BlueprintToken::EqualsToken, BlueprintToken::ValueToken], LineType::Reassignment),
        (vec![BlueprintToken::KeywordToken, BlueprintToken::Meta { token: MetaToken::WhateverIsNext }], LineType::Keyword),
    ]
});

fn check_abstract(b1: &AbstractToken, b2: &BlueprintToken) -> bool {
    return match (b1, b2) {
        (AbstractToken::Identification, BlueprintToken::IdentificationToken) => true,
        (AbstractToken::Type, BlueprintToken::DoublePurrType) => true,
        (AbstractToken::Type, BlueprintToken::PurrType) => true,
        (AbstractToken::Type, BlueprintToken::WoolType) => true,
        (AbstractToken::Special, BlueprintToken::Period) => true,
        (AbstractToken::Special, BlueprintToken::EndOfLine) => true,
        (AbstractToken::Value, BlueprintToken::DoublePurrValue) => true,
        (AbstractToken::Value, BlueprintToken::PurrValue) => true,
        (AbstractToken::Value, BlueprintToken::WoolValue) => true,
        (AbstractToken::ExpressionToken, BlueprintToken::ExpressionToken) => true,
        _ => false
    }
}

pub fn check_structure(tokens: &Vec<Token>) -> Option<LineType> {
    for blueprint in VALID_BLUEPRINTS.iter() {
        let mut is_valid = true;

        for (index, token) in blueprint.0.iter().enumerate() {
            if tokens.get(index).is_none() && matches!(token, &BlueprintToken::Meta { token: MetaToken::WhateverIsNext }) {
                is_valid = false;
                break
            }

            if token != &BlueprintToken::EndOfLine {
                is_valid = false;
                break
            }

            if &tokens[index].to_blueprint() != token {
                is_valid = false;
                break
            }
        }

        if is_valid {
            return Some(blueprint.1)
        }
    }
    None
}