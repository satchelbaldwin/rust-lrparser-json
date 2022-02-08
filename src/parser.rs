use crate::lexer::{Lexer, Token};
use crate::parsetable::{self, Action, ActionTablePair, GotoTablePair};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Symbol {
    Object,
    Pair,
    Pairs,
    PairsTail,
    Value,
    Array,
    Elements,
    ElementsTail,
    EOF,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct State(pub usize);

#[derive(Debug)]
pub enum StackValue {
    StackState(State),
    StackSymbol(Symbol),
    StackToken(Token),
}

pub type Stack = Vec<StackValue>;

pub struct Parser {
    goto: parsetable::GotoTable,
    action: parsetable::ActionTable,
    state: State,
    stack: Stack,
    lexer: Lexer,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Parser {
        let mut p = Parser {
            goto: parsetable::build_json_goto_table(),
            action: parsetable::build_json_action_table(),
            state: State(0),
            stack: Vec::new(),
            lexer,
        };
        p.stack.push(StackValue::StackState(State(0)));
        p
    }

    pub fn step(&mut self) -> Result<(), &str> {
        println!("step:");
        println!("  state: {:?}", self.state);
        println!("  stack: {:?}", self.stack);

        let last_symbol = self.stack.last();
        if last_symbol.is_none() {
            return Err("No symbol on stack to goto.");
        }
        match last_symbol {
            Some(StackValue::StackSymbol(s)) => {
                let goto_context = GotoTablePair::new(self.state, s.clone());
                match self.goto.get(&goto_context) {
                    Some(state) => {
                        println!(
                            "    goto action: to state {:?} from symbol {:?}",
                            self.state, s
                        );
                        self.stack.push(StackValue::StackState(state.clone()));
                        self.state = *state;
                        return self.step();
                    }
                    None => {}
                }
            }
            _ => {}
        }

        let token = self.lexer.next_token(false);
        println!("  token: {:?}", token);

        if let Some(mut token) = token {
            match token.clone() {
                Token::String(s) => {
                    token = Token::StringMatch;
                }
                Token::Number(n) => {
                    token = Token::NumberMatch;
                }
                _ => {}
            }
            let context = ActionTablePair::new(self.state, token.clone());
            println!("    action table lookup: {:?}", context);
            match self.action.get(&context) {
                Some(action) => match action {
                    Action::Accept => Ok(()),
                    Action::Shift(t, s) => {
                        println!(
                            "      shift action: token: {:?} state: {:?}",
                            context, self.state
                        );
                        self.stack.push(StackValue::StackToken(t.clone()));
                        self.stack.push(StackValue::StackState(s.clone()));
                        self.state = *s;
                        let _ = self.lexer.next_token(true);
                        self.step()
                    }
                    Action::Reduce(n, s) => {
                        println!("      reduce action: popped {:?} to produce {:?}", n, s);
                        let i = self.stack.len() - n;
                        let mut popped: Stack = self.stack.drain(i..).collect();
                        match self.stack.last() {
                            Some(StackValue::StackState(state)) => {
                                self.state = state.clone();
                            }
                            _ => {
                                println!("No prior state on stack after reduce.")
                            }
                        }
                        self.stack.push(StackValue::StackSymbol(s.clone()));
                        self.step()
                    }
                },
                // No action, check goto
                None => Err("Error: None found."),
            }
        } else {
            Err("Error: No token from lexer.")
        }
    }
}
