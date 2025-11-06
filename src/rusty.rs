use std::sync::LazyLock;
use crate::toki;
use crate::toki::{BlueprintToken, Token};

static VALID_BLUEPRINTS: LazyLock<[Vec<BlueprintToken>; 2]> = LazyLock::new(|| {
    [
        vec![BlueprintToken::TypeToken, BlueprintToken::IdentificationToken, BlueprintToken::OperatorToken, BlueprintToken::ValueToken],
        vec![BlueprintToken::TypeToken, BlueprintToken::IdentificationToken, BlueprintToken::OperatorToken, BlueprintToken::ExpressionToken],
    ]
});

pub fn check_structure(tokens: Vec<Token>) -> bool {
    VALID_BLUEPRINTS.contains(&toki::to_blueprint_tokens(&tokens))
}
