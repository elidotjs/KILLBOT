use std::io::{Error, ErrorKind};
use std::process::exit;
use crate::toki::{Token, ValueType};
use crate::toki::TokenType::*;
use crate::toki::ValueType::*;

pub fn get_type(token: &Token) -> ValueType {
    match token.token_type {
        Value {
            value_type
        } => {
            value_type
        },
        _ => {
            exit(421)
        }
    }
}

pub fn get_purr(token: &Token) -> Result<i32, Error> {
    match token.value.parse() {
        Err(e) => {
            Err(Error::new(ErrorKind::Other, format!("The value of {} is not a Purr, even though Toki says it is.", token.value)))
        }
        Ok(value) => {
            Ok(value)
        }
    }
}
pub fn get_doublepurr(token: &Token) -> Result<f64, Error> {
    match token.value.parse() {
        Err(e) => {
            Err(Error::new(ErrorKind::Other, format!("The value of {} is not a DoublePurr, even though Toki says it is.", token.value)))
        }
        Ok(value) => {
            Ok(value)
        }
    }
}

pub(crate) fn add(op: &Token, operand: &Token) -> Result<Token, Error> {
    let op_type = get_type(op);

    let operand_type = get_type(operand);

    let mut result_token = Token {
        value: String::new(),
        token_type: Keyword
    };

    match op_type {
        Purr => {
            let op_value: i32 = match get_purr(op) {
                Ok(value) => {
                    value
                }
                Err(e) => {
                    return Err(e)
                }
            };

            match operand_type {
                Purr => {
                    let operand_value: i32 = match get_purr(op) {
                        Ok(value) => {
                            value
                        }
                        Err(e) => {
                            return Err(e)
                        }
                    };

                    result_token.value = (operand_value + op_value).to_string();
                    result_token.token_type = Value {
                        value_type: DoublePurr
                    }
                }
                DoublePurr => {
                    let operand_value: f64 = match get_doublepurr(op) {
                        Ok(value) => {
                            value
                        }
                        Err(e) => {
                            return Err(e)
                        }
                    };

                    result_token.value = (operand_value + op_value as f64).to_string();
                    result_token.token_type = Value {
                        value_type: DoublePurr
                    }
                }
                Wool => {
                    return Err(Error::new(ErrorKind::Other, "You cannot add Purr to Wool."))
                }
            }
        }
        DoublePurr => {
            let op_value: f64 = op.value.parse().expect("what");

            match operand_type {
                Purr => {
                    let operand_value = match get_doublepurr(op) {
                        Ok(value) => {
                            value
                        }
                        Err(e) => {
                            return Err(e)
                        }
                    };

                    result_token.value = (operand_value as f64 + op_value).to_string();
                    result_token.token_type = Value {
                        value_type: DoublePurr
                    }
                }
                DoublePurr => {
                    let operand_value = match get_doublepurr(op) {
                        Ok(value) => {
                            value
                        }
                        Err(e) => {
                            return Err(e)
                        }
                    };

                    result_token.value = (operand_value + op_value).to_string();
                    result_token.token_type = Value {
                        value_type: DoublePurr
                    }
                }
                Wool => {
                    return Err(Error::new(ErrorKind::Other, "You cannot add DoublePurr to Wool."))
                }
            }
        }
        Wool => {
            let mut op_value = op.value.clone();

            match operand_type {
                Purr => {
                    return Err(Error::new(ErrorKind::Other, "You cannot add Wool to Purr."))
                }
                DoublePurr => {
                    return Err(Error::new(ErrorKind::Other, "You cannot add Wool to DoublePurr."))
                }
                Wool => {
                    op_value.push_str(operand.value.as_str());
                }
            }
        }
    }

    Ok(result_token)
}