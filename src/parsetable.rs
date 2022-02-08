use crate::lexer::Token;
use crate::parser::State;
use crate::parser::Symbol;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct ActionTablePair {
    state: State,
    token: Token,
}

impl ActionTablePair {
    pub fn new(state: State, token: Token) -> ActionTablePair {
        ActionTablePair { state, token }
    }
}

pub enum Action {
    Shift(Token, State),
    Reduce(usize, Symbol), // Elements to pop for production -> Symbol
    Accept,
}
pub type ActionTable = HashMap<ActionTablePair, Action>;

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct GotoTablePair {
    state: State,
    symbol: Symbol,
}

impl GotoTablePair {
    pub fn new(state: State, symbol: Symbol) -> GotoTablePair {
        GotoTablePair { state, symbol }
    }
}
pub type GotoTable = HashMap<GotoTablePair, State>;

// ugly copy and paste from the table as built
// it would be nice to read these from a configuration file... like... json
// .....
// realistically yaml could be used here but the goal is to reimplement everything by hand
// this is not an efficient nor good way to do this

pub fn add_shift(table: &mut ActionTable, state: usize, token: Token, end_state: usize) {
    table.insert(
        ActionTablePair::new(State(state), token.clone()),
        Action::Shift(token, State(end_state)),
    );
}

pub fn add_reduce(
    table: &mut ActionTable,
    state: usize,
    token: Token,
    num_to_pop: usize,
    production: Symbol,
) {
    table.insert(
        ActionTablePair::new(State(state), token),
        Action::Reduce(num_to_pop, production),
    );
}

pub fn build_json_action_table() -> ActionTable {
    let mut table: ActionTable = HashMap::new();
    //
    add_shift(&mut table, 0, Token::BeginObject, 3);
    table.insert(ActionTablePair::new(State(1), Token::EOF), Action::Accept);

    add_reduce(&mut table, 3, Token::EndObject, 0, Symbol::Pairs);
    add_shift(&mut table, 3, Token::StringMatch, 5);

    add_shift(&mut table, 4, Token::EndObject, 7);

    add_shift(&mut table, 5, Token::NameSeparator, 8);

    add_reduce(&mut table, 6, Token::EndObject, 0, Symbol::PairsTail);
    add_shift(&mut table, 6, Token::ValueSeparator, 9);

    add_reduce(&mut table, 7, Token::EOF, 6, Symbol::Object);
    add_reduce(&mut table, 7, Token::EndObject, 6, Symbol::Object);
    add_reduce(&mut table, 7, Token::ValueSeparator, 6, Symbol::Object);
    add_reduce(&mut table, 7, Token::EndArray, 6, Symbol::Object);

    add_shift(&mut table, 8, Token::BeginObject, 3);
    add_shift(&mut table, 8, Token::StringMatch, 12);
    add_shift(&mut table, 8, Token::NumberMatch, 13);
    add_shift(&mut table, 8, Token::True, 14);
    add_shift(&mut table, 8, Token::False, 15);
    add_shift(&mut table, 8, Token::Null, 16);
    add_shift(&mut table, 8, Token::BeginArray, 20);

    add_reduce(&mut table, 9, Token::EndObject, 0, Symbol::Pairs);
    add_shift(&mut table, 9, Token::StringMatch, 5);

    add_reduce(&mut table, 10, Token::EndObject, 4, Symbol::Pairs);

    add_reduce(&mut table, 11, Token::EndObject, 6, Symbol::Pair);
    add_reduce(&mut table, 11, Token::ValueSeparator, 6, Symbol::Pair);

    for i in 12..=18 {
        add_reduce(&mut table, i, Token::EndObject, 2, Symbol::Value);
        add_reduce(&mut table, i, Token::ValueSeparator, 2, Symbol::Value);
        add_reduce(&mut table, i, Token::EndArray, 2, Symbol::Value);
    }

    add_reduce(&mut table, 19, Token::EndObject, 4, Symbol::PairsTail);

    add_shift(&mut table, 20, Token::BeginObject, 3);
    add_shift(&mut table, 20, Token::StringMatch, 12);
    add_shift(&mut table, 20, Token::NumberMatch, 13);
    add_shift(&mut table, 20, Token::True, 14);
    add_shift(&mut table, 20, Token::False, 15);
    add_shift(&mut table, 20, Token::Null, 16);
    add_shift(&mut table, 20, Token::BeginArray, 20);
    add_reduce(&mut table, 20, Token::EndArray, 0, Symbol::Elements);

    add_shift(&mut table, 21, Token::EndArray, 23);

    add_shift(&mut table, 22, Token::ValueSeparator, 25);
    add_reduce(&mut table, 22, Token::EndArray, 0, Symbol::ElementsTail);

    add_reduce(&mut table, 23, Token::EndObject, 6, Symbol::Array);
    add_reduce(&mut table, 23, Token::ValueSeparator, 6, Symbol::Array);
    add_reduce(&mut table, 23, Token::EndArray, 6, Symbol::Array);

    add_reduce(&mut table, 24, Token::EndArray, 4, Symbol::Elements);

    add_shift(&mut table, 25, Token::BeginObject, 3);
    add_shift(&mut table, 25, Token::StringMatch, 12);
    add_shift(&mut table, 25, Token::NumberMatch, 13);
    add_shift(&mut table, 25, Token::True, 14);
    add_shift(&mut table, 25, Token::False, 15);
    add_shift(&mut table, 25, Token::Null, 16);
    add_shift(&mut table, 25, Token::BeginArray, 20);
    add_reduce(&mut table, 25, Token::EndArray, 0, Symbol::Elements);

    add_reduce(&mut table, 26, Token::EndArray, 4, Symbol::ElementsTail);

    table
}

pub fn build_json_goto_table() -> GotoTable {
    let mut table: GotoTable = HashMap::new();

    let mut add = |state: usize, symbol: Symbol, end_state: usize| {
        table.insert(GotoTablePair::new(State(state), symbol), State(end_state));
    };

    add(0, Symbol::Object, 1);
    add(3, Symbol::Pairs, 4);
    add(3, Symbol::Pair, 6);
    add(6, Symbol::PairsTail, 10);
    add(8, Symbol::Object, 17);
    add(8, Symbol::Value, 11);
    add(8, Symbol::Array, 18);
    add(9, Symbol::Pairs, 19);
    add(9, Symbol::Pair, 6);
    add(9, Symbol::Pair, 6);
    add(20, Symbol::Object, 17);
    add(20, Symbol::Value, 18);
    add(20, Symbol::Array, 20);
    add(20, Symbol::Elements, 21);
    add(22, Symbol::ElementsTail, 24);
    add(25, Symbol::Object, 17);
    add(25, Symbol::Value, 18);
    add(25, Symbol::Array, 25);
    add(25, Symbol::Elements, 21);

    table
}
