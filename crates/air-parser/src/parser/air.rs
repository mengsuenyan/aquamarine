// auto-generated: "lalrpop 0.19.1"
// sha256: 1893aaaa3cc9f281c81afab956e5d868b6d3124e663cdaa9f4652925d0d0f7
use crate::parser::ast::*;
use crate::parser::into_variable_and_path;
use crate::parser::lexer::LexerError;
use crate::parser::lexer::Token;
use lalrpop_util::ErrorRecovery;
use std::rc::Rc;
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;
#[allow(unused_imports)]
use self::__lalrpop_util::state_machine as __state_machine;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__AIR {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use crate::parser::ast::*;
    use crate::parser::into_variable_and_path;
    use crate::parser::lexer::LexerError;
    use crate::parser::lexer::Token;
    use lalrpop_util::ErrorRecovery;
    use std::rc::Rc;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub enum __Symbol<'input>
     {
        Variant0(Token<'input>),
        Variant1(&'input str),
        Variant2((&'input str, usize)),
        Variant3(__lalrpop_util::ErrorRecovery<usize, Token<'input>, LexerError>),
        Variant4(CallArgValue<'input>),
        Variant5(::std::vec::Vec<CallArgValue<'input>>),
        Variant6(Box<Instruction<'input>>),
        Variant7(Vec<CallArgValue<'input>>),
        Variant8(FunctionPart<'input>),
        Variant9(IterableValue<'input>),
        Variant10(MatchableValue<'input>),
        Variant11(CallOutputValue<'input>),
        Variant12(::std::option::Option<CallOutputValue<'input>>),
        Variant13(PeerPart<'input>),
    }
    const __ACTION: &[i8] = &[
        // State 0
        28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29,
        // State 1
        10, 0, 0, 0, 0, 34, 35, 36, 37, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 39, 0, 40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 41, 0, 42, 43, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 41, 0, 42, 43, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29,
        // State 6
        28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29,
        // State 7
        28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29,
        // State 8
        17, 0, 0, 0, 0, 34, 35, 36, 37, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 34, 35, 36, 37, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 41, 0, 42, 43, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 41, 0, 42, 43, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29,
        // State 13
        28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29,
        // State 14
        28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29,
        // State 15
        0, 0, 23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 34, 35, 36, 37, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 34, 35, 36, 37, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29,
        // State 19
        28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29,
        // State 20
        28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29,
        // State 21
        0, 61, 0, 0, 62, 63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 66, 0, 34, 35, 36, 37, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 34, 35, 36, 37, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 74, 0, 34, 35, 36, 37, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 3, 4, 5, 30, 31, 6, 7, 8, 0,
        // State 28
        -27, -27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -27,
        // State 29
        0, 0, 0, 0, 0, 44, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        -37, 0, 0, 0, 0, -37, -37, -37, -37, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        -38, 0, 0, 0, 0, -38, -38, -38, -38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        -11, -11, -11, -11, 0, -11, -11, -11, -11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        -13, -13, -13, -13, 0, -13, -13, -13, -13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        -12, -12, -12, -12, 0, -12, -12, -12, -12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        -10, -10, -10, -10, 0, -10, -10, -10, -10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, -28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, -29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        -30, 0, 0, 0, 0, -30, 0, -30, -30, 0, 0, 0, 0, 0, 0, 0, 0, 0, -30,
        // State 41
        -32, 0, 0, 0, 0, -32, 0, -32, -32, 0, 0, 0, 0, 0, 0, 0, 0, 0, -32,
        // State 42
        -31, 0, 0, 0, 0, -31, 0, -31, -31, 0, 0, 0, 0, 0, 0, 0, 0, 0, -31,
        // State 43
        0, 48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 44
        -21, -21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -21,
        // State 45
        0, -16, -16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 46
        0, 0, -14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 47
        -23, -23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -23,
        // State 48
        0, 57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 49
        0, 58, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 50
        0, 59, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 51
        0, -40, 0, 0, 0, -40, -40, -40, -40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 52
        0, 68, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 53
        0, 69, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 54
        0, 70, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 55
        0, 71, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 56
        -20, -20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -20,
        // State 57
        -19, -19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -19,
        // State 58
        -24, -24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -24,
        // State 59
        0, 72, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 60
        -18, -18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -18,
        // State 61
        0, -34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 62
        0, -33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 63
        0, 0, 0, -4, 0, -4, -4, -4, -4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 64
        0, 0, 0, -7, 0, -7, -7, -7, -7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 65
        0, -8, 0, 0, -8, -8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 66
        0, 75, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 67
        -39, 0, 0, 0, 0, -39, -39, -39, -39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 68
        -22, -22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -22,
        // State 69
        -25, -25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -25,
        // State 70
        -26, -26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -26,
        // State 71
        -17, -17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -17,
        // State 72
        0, 0, 0, -5, 0, -5, -5, -5, -5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 73
        0, -9, 0, 0, -9, -9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 74
        0, 0, -15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 19 + integer]
    }
    const __EOF_ACTION: &[i8] = &[
        // State 0
        0,
        // State 1
        0,
        // State 2
        0,
        // State 3
        0,
        // State 4
        0,
        // State 5
        0,
        // State 6
        0,
        // State 7
        0,
        // State 8
        0,
        // State 9
        0,
        // State 10
        0,
        // State 11
        0,
        // State 12
        0,
        // State 13
        0,
        // State 14
        0,
        // State 15
        0,
        // State 16
        0,
        // State 17
        0,
        // State 18
        0,
        // State 19
        0,
        // State 20
        0,
        // State 21
        0,
        // State 22
        0,
        // State 23
        0,
        // State 24
        0,
        // State 25
        -41,
        // State 26
        -6,
        // State 27
        0,
        // State 28
        -27,
        // State 29
        0,
        // State 30
        0,
        // State 31
        0,
        // State 32
        0,
        // State 33
        0,
        // State 34
        0,
        // State 35
        0,
        // State 36
        0,
        // State 37
        0,
        // State 38
        0,
        // State 39
        0,
        // State 40
        0,
        // State 41
        0,
        // State 42
        0,
        // State 43
        0,
        // State 44
        -21,
        // State 45
        0,
        // State 46
        0,
        // State 47
        -23,
        // State 48
        0,
        // State 49
        0,
        // State 50
        0,
        // State 51
        0,
        // State 52
        0,
        // State 53
        0,
        // State 54
        0,
        // State 55
        0,
        // State 56
        -20,
        // State 57
        -19,
        // State 58
        -24,
        // State 59
        0,
        // State 60
        -18,
        // State 61
        0,
        // State 62
        0,
        // State 63
        0,
        // State 64
        0,
        // State 65
        0,
        // State 66
        0,
        // State 67
        0,
        // State 68
        -22,
        // State 69
        -25,
        // State 70
        -26,
        // State 71
        -17,
        // State 72
        0,
        // State 73
        0,
        // State 74
        0,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            2 => 24,
            3 => 25,
            4 => match state {
                24 => 72,
                _ => 63,
            },
            5 => 21,
            6 => match state {
                8 | 23 => 45,
                16..=17 => 51,
                22 | 24 => 64,
                _ => 31,
            },
            7 => 15,
            8 => match state {
                23 => 66,
                _ => 46,
            },
            9 => match state {
                6 => 13,
                7 => 14,
                0 => 26,
                12 => 48,
                13 => 49,
                14 => 50,
                18 => 53,
                19 => 54,
                20 => 55,
                _ => 12,
            },
            10 => 37,
            11 => match state {
                4 => 11,
                10 => 19,
                11 => 20,
                _ => 10,
            },
            12 => 59,
            14 => match state {
                1 => 32,
                _ => 17,
            },
            15 => 8,
            16 => match state {
                17 => 52,
                _ => 23,
            },
            _ => 0,
        }
    }
    fn __expected_tokens(__state: i8) -> Vec<::std::string::String> {
        const __TERMINAL: &[&str] = &[
            r###""(""###,
            r###"")""###,
            r###""[""###,
            r###""]""###,
            r###"Accumulator"###,
            r###"Alphanumeric"###,
            r###"InitPeerId"###,
            r###"JsonPath"###,
            r###"Literal"###,
            r###"call"###,
            r###"fold"###,
            r###"match_"###,
            r###"mismatch"###,
            r###"next"###,
            r###"null"###,
            r###"par"###,
            r###"seq"###,
            r###"xor"###,
        ];
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            let next_state = __action(__state, index);
            if next_state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub struct __StateMachine<'err, 'input>
    where 'input: 'err
    {
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
        __phantom: ::std::marker::PhantomData<(&'err (), &'input ())>,
    }
    impl<'err, 'input> __state_machine::ParserDefinition for __StateMachine<'err, 'input>
    where 'input: 'err
    {
        type Location = usize;
        type Error = LexerError;
        type Token = Token<'input>;
        type TokenIndex = usize;
        type Symbol = __Symbol<'input>;
        type Success = Box<Instruction<'input>>;
        type StateIndex = i8;
        type Action = i8;
        type ReduceIndex = i8;
        type NonterminalIndex = usize;

        #[inline]
        fn start_location(&self) -> Self::Location {
              Default::default()
        }

        #[inline]
        fn start_state(&self) -> Self::StateIndex {
              0
        }

        #[inline]
        fn token_to_index(&self, token: &Self::Token) -> Option<usize> {
            __token_to_integer(token, ::std::marker::PhantomData::<(&(), &())>)
        }

        #[inline]
        fn action(&self, state: i8, integer: usize) -> i8 {
            __action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i8) -> i8 {
            __action(state, 19 - 1)
        }

        #[inline]
        fn eof_action(&self, state: i8) -> i8 {
            __EOF_ACTION[state as usize]
        }

        #[inline]
        fn goto(&self, state: i8, nt: usize) -> i8 {
            __goto(state, nt)
        }

        fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {
            __token_to_symbol(token_index, token, ::std::marker::PhantomData::<(&(), &())>)
        }

        fn expected_tokens(&self, state: i8) -> Vec<String> {
            __expected_tokens(state)
        }

        #[inline]
        fn uses_error_recovery(&self) -> bool {
            true
        }

        #[inline]
        fn error_recovery_symbol(
            &self,
            recovery: __state_machine::ErrorRecovery<Self>,
        ) -> Self::Symbol {
            __Symbol::Variant3(recovery)
        }

        fn reduce(
            &mut self,
            action: i8,
            start_location: Option<&Self::Location>,
            states: &mut Vec<i8>,
            symbols: &mut Vec<__state_machine::SymbolTriple<Self>>,
        ) -> Option<__state_machine::ParseResult<Self>> {
            __reduce(
                self.input,
                self.errors,
                action,
                start_location,
                states,
                symbols,
                ::std::marker::PhantomData::<(&(), &())>,
            )
        }

        fn simulate_reduce(&self, action: i8) -> __state_machine::SimulatedReduce<Self> {
            __simulate_reduce(action, ::std::marker::PhantomData::<(&(), &())>)
        }
    }
    fn __token_to_integer<
        'err,
        'input,
    >(
        __token: &Token<'input>,
        _: ::std::marker::PhantomData<(&'err (), &'input ())>,
    ) -> Option<usize>
    {
        match *__token {
            Token::OpenRoundBracket if true => Some(0),
            Token::CloseRoundBracket if true => Some(1),
            Token::OpenSquareBracket if true => Some(2),
            Token::CloseSquareBracket if true => Some(3),
            Token::Accumulator(_) if true => Some(4),
            Token::Alphanumeric(_) if true => Some(5),
            Token::InitPeerId if true => Some(6),
            Token::JsonPath(_, _) if true => Some(7),
            Token::StringLiteral(_) if true => Some(8),
            Token::Call if true => Some(9),
            Token::Fold if true => Some(10),
            Token::Match if true => Some(11),
            Token::MisMatch if true => Some(12),
            Token::Next if true => Some(13),
            Token::Null if true => Some(14),
            Token::Par if true => Some(15),
            Token::Seq if true => Some(16),
            Token::Xor if true => Some(17),
            _ => None,
        }
    }
    fn __token_to_symbol<
        'err,
        'input,
    >(
        __token_index: usize,
        __token: Token<'input>,
        _: ::std::marker::PhantomData<(&'err (), &'input ())>,
    ) -> __Symbol<'input>
    {
        match __token_index {
            0 | 1 | 2 | 3 | 6 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 => __Symbol::Variant0(__token),
            4 | 5 | 8 => match __token {
                Token::Accumulator(__tok0) | Token::Alphanumeric(__tok0) | Token::StringLiteral(__tok0) if true => __Symbol::Variant1(__tok0),
                _ => unreachable!(),
            },
            7 => match __token {
                Token::JsonPath(__tok0, __tok1) if true => __Symbol::Variant2((__tok0, __tok1)),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    fn __simulate_reduce<
        'err,
        'input,
    >(
        __reduce_index: i8,
        _: ::std::marker::PhantomData<(&'err (), &'input ())>,
    ) -> __state_machine::SimulatedReduce<__StateMachine<'err, 'input>>
    where
        'input: 'err,
    {
        match __reduce_index {
            0 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 0,
                }
            }
            1 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 1,
                }
            }
            2 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 1,
                }
            }
            3 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 2,
                }
            }
            4 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 2,
                }
            }
            5 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 3,
                }
            }
            6 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 4,
                }
            }
            7 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 5,
                }
            }
            8 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 5,
                }
            }
            9 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 6,
                }
            }
            10 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 6,
                }
            }
            11 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 6,
                }
            }
            12 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 6,
                }
            }
            13 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 7,
                }
            }
            14 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 7,
                }
            }
            15 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 8,
                }
            }
            16 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 7,
                    nonterminal_produced: 9,
                }
            }
            17 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 6,
                    nonterminal_produced: 9,
                }
            }
            18 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 9,
                }
            }
            19 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 9,
                }
            }
            20 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 9,
                }
            }
            21 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 6,
                    nonterminal_produced: 9,
                }
            }
            22 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 9,
                }
            }
            23 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 5,
                    nonterminal_produced: 9,
                }
            }
            24 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 6,
                    nonterminal_produced: 9,
                }
            }
            25 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 6,
                    nonterminal_produced: 9,
                }
            }
            26 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 9,
                }
            }
            27 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 10,
                }
            }
            28 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 10,
                }
            }
            29 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 11,
                }
            }
            30 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 11,
                }
            }
            31 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 11,
                }
            }
            32 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 12,
                }
            }
            33 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 12,
                }
            }
            34 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 13,
                }
            }
            35 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 13,
                }
            }
            36 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 14,
                }
            }
            37 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 15,
                }
            }
            38 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 15,
                }
            }
            39 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 16,
                }
            }
            40 => __state_machine::SimulatedReduce::Accept,
            _ => panic!("invalid reduction index {}", __reduce_index)
        }
    }
    #[derive(Clone)]
    pub struct AIRParser {
        _priv: (),
    }

    impl AIRParser {
        pub fn new() -> AIRParser {
            AIRParser {
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            'err,
            'input,
            __TOKEN: __ToTriple<'err, 'input, >,
            __TOKENS: IntoIterator<Item=__TOKEN>,
        >(
            &self,
            input: &'input str,
            errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
            __tokens0: __TOKENS,
        ) -> Result<Box<Instruction<'input>>, __lalrpop_util::ParseError<usize, Token<'input>, LexerError>>
        {
            let __tokens = __tokens0.into_iter();
            let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
            __state_machine::Parser::drive(
                __StateMachine {
                    input,
                    errors,
                    __phantom: ::std::marker::PhantomData::<(&(), &())>,
                },
                __tokens,
            )
        }
    }
    fn __accepts<
        'err,
        'input,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
        __error_state: i8,
        __states: & [i8],
        __opt_integer: Option<usize>,
        _: ::std::marker::PhantomData<(&'err (), &'input ())>,
    ) -> bool
    {
        let mut __states = __states.to_vec();
        __states.push(__error_state);
        loop {
            let mut __states_len = __states.len();
            let __top = __states[__states_len - 1];
            let __action = match __opt_integer {
                None => __EOF_ACTION[__top as usize],
                Some(__integer) => __action(__top, __integer),
            };
            if __action == 0 { return false; }
            if __action > 0 { return true; }
            let (__to_pop, __nt) = match __simulate_reduce(-(__action + 1), ::std::marker::PhantomData::<(&(), &())>) {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop, nonterminal_produced
                } => (states_to_pop, nonterminal_produced),
                __state_machine::SimulatedReduce::Accept => return true,
            };
            __states_len -= __to_pop;
            __states.truncate(__states_len);
            let __top = __states[__states_len - 1];
            let __next_state = __goto(__top, __nt);
            __states.push(__next_state);
        }
    }
    pub(crate) fn __reduce<
        'err,
        'input,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'err (), &'input ())>,
    ) -> Option<Result<Box<Instruction<'input>>,__lalrpop_util::ParseError<usize, Token<'input>, LexerError>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(input, errors, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            1 => {
                __reduce1(input, errors, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            2 => {
                __reduce2(input, errors, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            3 => {
                __reduce3(input, errors, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            4 => {
                __reduce4(input, errors, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            5 => {
                __reduce5(input, errors, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            6 => {
                __reduce6(input, errors, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            7 => {
                __reduce7(input, errors, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            8 => {
                __reduce8(input, errors, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            9 => {
                __reduce9(input, errors, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            10 => {
                __reduce10(input, errors, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            11 => {
                __reduce11(input, errors, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            12 => {
                __reduce12(input, errors, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            13 => {
                __reduce13(input, errors, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            14 => {
                __reduce14(input, errors, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            15 => {
                __reduce15(input, errors, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            16 => {
                __reduce16(input, errors, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            17 => {
                __reduce17(input, errors, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            18 => {
                __reduce18(input, errors, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            19 => {
                __reduce19(input, errors, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            20 => {
                __reduce20(input, errors, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            21 => {
                __reduce21(input, errors, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            22 => {
                __reduce22(input, errors, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            23 => {
                __reduce23(input, errors, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            24 => {
                __reduce24(input, errors, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            25 => {
                __reduce25(input, errors, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            26 => {
                __reduce26(input, errors, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            27 => {
                __reduce27(input, errors, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            28 => {
                __reduce28(input, errors, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            29 => {
                __reduce29(input, errors, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            30 => {
                __reduce30(input, errors, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            31 => {
                __reduce31(input, errors, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            32 => {
                __reduce32(input, errors, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            33 => {
                __reduce33(input, errors, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            34 => {
                __reduce34(input, errors, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            35 => {
                __reduce35(input, errors, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            36 => {
                __reduce36(input, errors, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            37 => {
                __reduce37(input, errors, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            38 => {
                __reduce38(input, errors, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            39 => {
                __reduce39(input, errors, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&(), &())>)
            }
            40 => {
                // __AIR = AIR => ActionFn(0);
                let __sym0 = __pop_Variant6(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(input, errors, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        let __state = *__states.last().unwrap();
        let __next_state = __goto(__state, __nonterminal);
        __states.push(__next_state);
        None
    }
    #[inline(never)]
    fn __symbol_type_mismatch() -> ! {
        panic!("symbol type mismatch")
    }
    fn __pop_Variant2<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (&'input str, usize), usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Instruction<'input>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, CallArgValue<'input>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant11<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, CallOutputValue<'input>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant11(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant8<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, FunctionPart<'input>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant8(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant9<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, IterableValue<'input>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant9(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant10<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, MatchableValue<'input>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant10(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant13<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, PeerPart<'input>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant13(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant0<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Token<'input>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant7<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<CallArgValue<'input>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant3<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, __lalrpop_util::ErrorRecovery<usize, Token<'input>, LexerError>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant12<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<CallOutputValue<'input>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant12(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant5<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<CallArgValue<'input>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant1<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    pub(crate) fn __reduce0<
        'err,
        'input,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'err (), &'input ())>,
    ) -> (usize, usize)
    {
        // (<Arg>) = Arg => ActionFn(34);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action34::<>(input, errors, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 0)
    }
    pub(crate) fn __reduce1<
        'err,
        'input,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'err (), &'input ())>,
    ) -> (usize, usize)
    {
        // (<Arg>)* =  => ActionFn(32);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action32::<>(input, errors, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (0, 1)
    }
    pub(crate) fn __reduce2<
        'err,
        'input,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'err (), &'input ())>,
    ) -> (usize, usize)
    {
        // (<Arg>)* = (<Arg>)+ => ActionFn(33);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action33::<>(input, errors, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 1)
    }
    pub(crate) fn __reduce3<
        'err,
        'input,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'err (), &'input ())>,
    ) -> (usize, usize)
    {
        // (<Arg>)+ = Arg => ActionFn(39);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action39::<>(input, errors, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 2)
    }
    pub(crate) fn __reduce4<
        'err,
        'input,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'err (), &'input ())>,
    ) -> (usize, usize)
    {
        // (<Arg>)+ = (<Arg>)+, Arg => ActionFn(40);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action40::<>(input, errors, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 2)
    }
    pub(crate) fn __reduce5<
        'err,
        'input,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'err (), &'input ())>,
    ) -> (usize, usize)
    {
        // AIR = Instr => ActionFn(1);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1::<>(input, errors, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 3)
    }
    pub(crate) fn __reduce6<
        'err,
        'input,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'err (), &'input ())>,
    ) -> (usize, usize)
    {
        // Arg = CallArgValue => ActionFn(22);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action22::<>(input, errors, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 4)
    }
    pub(crate) fn __reduce7<
        'err,
        'input,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'err (), &'input ())>,
    ) -> (usize, usize)
    {
        // Args = "[", "]" => ActionFn(41);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action41::<>(input, errors, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (2, 5)
    }
    pub(crate) fn __reduce8<
        'err,
        'input,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'err (), &'input ())>,
    ) -> (usize, usize)
    {
        // Args = "[", (<Arg>)+, "]" => ActionFn(42);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action42::<>(input, errors, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (3, 5)
    }
    pub(crate) fn __reduce9<
        'err,
        'input,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'err (), &'input ())>,
    ) -> (usize, usize)
    {
        // CallArgValue = Literal => ActionFn(23);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action23::<>(input, errors, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce10<
        'err,
        'input,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'err (), &'input ())>,
    ) -> (usize, usize)
    {
        // CallArgValue = Alphanumeric => ActionFn(24);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action24::<>(input, errors, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce11<
        'err,
        'input,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'err (), &'input ())>,
    ) -> (usize, usize)
    {
        // CallArgValue = JsonPath => ActionFn(25);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action25::<>(input, errors, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce12<
        'err,
        'input,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'err (), &'input ())>,
    ) -> (usize, usize)
    {
        // CallArgValue = InitPeerId => ActionFn(26);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action26::<>(input, errors, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce13<
        'err,
        'input,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'err (), &'input ())>,
    ) -> (usize, usize)
    {
        // FPart = Function => ActionFn(13);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action13::<>(input, errors, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 7)
    }
    pub(crate) fn __reduce14<
        'err,
        'input,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'err (), &'input ())>,
    ) -> (usize, usize)
    {
        // FPart = "(", ServiceId, Function, ")" => ActionFn(14);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action14::<>(input, errors, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (4, 7)
    }
    pub(crate) fn __reduce15<
        'err,
        'input,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'err (), &'input ())>,
    ) -> (usize, usize)
    {
        // Function = CallArgValue => ActionFn(19);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action19::<>(input, errors, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 8)
    }
    pub(crate) fn __reduce16<
        'err,
        'input,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'err (), &'input ())>,
    ) -> (usize, usize)
    {
        // Instr = "(", call, PeerPart, FPart, Args, Output, ")" => ActionFn(43);
        assert!(__symbols.len() >= 7);
        let __sym6 = __pop_Variant0(__symbols);
        let __sym5 = __pop_Variant11(__symbols);
        let __sym4 = __pop_Variant7(__symbols);
        let __sym3 = __pop_Variant8(__symbols);
        let __sym2 = __pop_Variant13(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym6.2.clone();
        let __nt = super::__action43::<>(input, errors, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (7, 9)
    }
    pub(crate) fn __reduce17<
        'err,
        'input,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'err (), &'input ())>,
    ) -> (usize, usize)
    {
        // Instr = "(", call, PeerPart, FPart, Args, ")" => ActionFn(44);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant7(__symbols);
        let __sym3 = __pop_Variant8(__symbols);
        let __sym2 = __pop_Variant13(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action44::<>(input, errors, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (6, 9)
    }
    pub(crate) fn __reduce18<
        'err,
        'input,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'err (), &'input ())>,
    ) -> (usize, usize)
    {
        // Instr = "(", seq, Instr, Instr, ")" => ActionFn(3);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant6(__symbols);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action3::<>(input, errors, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (5, 9)
    }
    pub(crate) fn __reduce19<
        'err,
        'input,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'err (), &'input ())>,
    ) -> (usize, usize)
    {
        // Instr = "(", par, Instr, Instr, ")" => ActionFn(4);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant6(__symbols);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action4::<>(input, errors, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (5, 9)
    }
    pub(crate) fn __reduce20<
        'err,
        'input,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'err (), &'input ())>,
    ) -> (usize, usize)
    {
        // Instr = "(", null, ")" => ActionFn(5);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action5::<>(input, errors, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 9)
    }
    pub(crate) fn __reduce21<
        'err,
        'input,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'err (), &'input ())>,
    ) -> (usize, usize)
    {
        // Instr = "(", fold, Iterable, Alphanumeric, Instr, ")" => ActionFn(6);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant6(__symbols);
        let __sym3 = __pop_Variant1(__symbols);
        let __sym2 = __pop_Variant9(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action6::<>(input, errors, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (6, 9)
    }
    pub(crate) fn __reduce22<
        'err,
        'input,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'err (), &'input ())>,
    ) -> (usize, usize)
    {
        // Instr = "(", next, Alphanumeric, ")" => ActionFn(7);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action7::<>(input, errors, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (4, 9)
    }
    pub(crate) fn __reduce23<
        'err,
        'input,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'err (), &'input ())>,
    ) -> (usize, usize)
    {
        // Instr = "(", xor, Instr, Instr, ")" => ActionFn(8);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant6(__symbols);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action8::<>(input, errors, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (5, 9)
    }
    pub(crate) fn __reduce24<
        'err,
        'input,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'err (), &'input ())>,
    ) -> (usize, usize)
    {
        // Instr = "(", match_, Matchable, Matchable, Instr, ")" => ActionFn(9);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant6(__symbols);
        let __sym3 = __pop_Variant10(__symbols);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action9::<>(input, errors, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (6, 9)
    }
    pub(crate) fn __reduce25<
        'err,
        'input,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'err (), &'input ())>,
    ) -> (usize, usize)
    {
        // Instr = "(", mismatch, Matchable, Matchable, Instr, ")" => ActionFn(10);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant6(__symbols);
        let __sym3 = __pop_Variant10(__symbols);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action10::<>(input, errors, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (6, 9)
    }
    pub(crate) fn __reduce26<
        'err,
        'input,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'err (), &'input ())>,
    ) -> (usize, usize)
    {
        // Instr = error => ActionFn(11);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action11::<>(input, errors, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce27<
        'err,
        'input,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'err (), &'input ())>,
    ) -> (usize, usize)
    {
        // Iterable = Alphanumeric => ActionFn(27);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action27::<>(input, errors, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 10)
    }
    pub(crate) fn __reduce28<
        'err,
        'input,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'err (), &'input ())>,
    ) -> (usize, usize)
    {
        // Iterable = JsonPath => ActionFn(28);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action28::<>(input, errors, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 10)
    }
    pub(crate) fn __reduce29<
        'err,
        'input,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'err (), &'input ())>,
    ) -> (usize, usize)
    {
        // Matchable = Alphanumeric => ActionFn(29);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action29::<>(input, errors, __sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce30<
        'err,
        'input,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'err (), &'input ())>,
    ) -> (usize, usize)
    {
        // Matchable = Literal => ActionFn(30);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action30::<>(input, errors, __sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce31<
        'err,
        'input,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'err (), &'input ())>,
    ) -> (usize, usize)
    {
        // Matchable = JsonPath => ActionFn(31);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action31::<>(input, errors, __sym0);
        __symbols.push((__start, __Symbol::Variant10(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce32<
        'err,
        'input,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'err (), &'input ())>,
    ) -> (usize, usize)
    {
        // Output = Alphanumeric => ActionFn(17);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action17::<>(input, errors, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 12)
    }
    pub(crate) fn __reduce33<
        'err,
        'input,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'err (), &'input ())>,
    ) -> (usize, usize)
    {
        // Output = Accumulator => ActionFn(18);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action18::<>(input, errors, __sym0);
        __symbols.push((__start, __Symbol::Variant11(__nt), __end));
        (1, 12)
    }
    pub(crate) fn __reduce34<
        'err,
        'input,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'err (), &'input ())>,
    ) -> (usize, usize)
    {
        // Output? = Output => ActionFn(35);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action35::<>(input, errors, __sym0);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (1, 13)
    }
    pub(crate) fn __reduce35<
        'err,
        'input,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'err (), &'input ())>,
    ) -> (usize, usize)
    {
        // Output? =  => ActionFn(36);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action36::<>(input, errors, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant12(__nt), __end));
        (0, 13)
    }
    pub(crate) fn __reduce36<
        'err,
        'input,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'err (), &'input ())>,
    ) -> (usize, usize)
    {
        // PeerId = CallArgValue => ActionFn(20);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action20::<>(input, errors, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce37<
        'err,
        'input,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'err (), &'input ())>,
    ) -> (usize, usize)
    {
        // PeerPart = PeerId => ActionFn(15);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action15::<>(input, errors, __sym0);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (1, 15)
    }
    pub(crate) fn __reduce38<
        'err,
        'input,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'err (), &'input ())>,
    ) -> (usize, usize)
    {
        // PeerPart = "(", PeerId, ServiceId, ")" => ActionFn(16);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action16::<>(input, errors, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant13(__nt), __end));
        (4, 15)
    }
    pub(crate) fn __reduce39<
        'err,
        'input,
    >(
        input: &'input str,
        errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'err (), &'input ())>,
    ) -> (usize, usize)
    {
        // ServiceId = CallArgValue => ActionFn(21);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action21::<>(input, errors, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 16)
    }
}
pub use self::__parse__AIR::AIRParser;

#[allow(unused_variables)]
fn __action0<
    'err,
    'input,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
    (_, __0, _): (usize, Box<Instruction<'input>>, usize),
) -> Box<Instruction<'input>>
{
    __0
}

#[allow(unused_variables)]
fn __action1<
    'err,
    'input,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
    (_, __0, _): (usize, Box<Instruction<'input>>, usize),
) -> Box<Instruction<'input>>
{
    __0
}

#[allow(unused_variables)]
fn __action2<
    'err,
    'input,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
    (_, _, _): (usize, Token<'input>, usize),
    (_, _, _): (usize, Token<'input>, usize),
    (_, p, _): (usize, PeerPart<'input>, usize),
    (_, f, _): (usize, FunctionPart<'input>, usize),
    (_, args, _): (usize, Vec<CallArgValue<'input>>, usize),
    (_, output, _): (usize, ::std::option::Option<CallOutputValue<'input>>, usize),
    (_, _, _): (usize, Token<'input>, usize),
) -> Box<Instruction<'input>>
{
    {
        let output = output.unwrap_or(CallOutputValue::None);
        let args = Rc::new(args);
        Box::new(Instruction::Call(Call{peer_part: p, function_part: f, args, output}))
    }
}

#[allow(unused_variables)]
fn __action3<
    'err,
    'input,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
    (_, _, _): (usize, Token<'input>, usize),
    (_, _, _): (usize, Token<'input>, usize),
    (_, l, _): (usize, Box<Instruction<'input>>, usize),
    (_, r, _): (usize, Box<Instruction<'input>>, usize),
    (_, _, _): (usize, Token<'input>, usize),
) -> Box<Instruction<'input>>
{
    Box::new(Instruction::Seq(Seq(l, r)))
}

#[allow(unused_variables)]
fn __action4<
    'err,
    'input,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
    (_, _, _): (usize, Token<'input>, usize),
    (_, _, _): (usize, Token<'input>, usize),
    (_, l, _): (usize, Box<Instruction<'input>>, usize),
    (_, r, _): (usize, Box<Instruction<'input>>, usize),
    (_, _, _): (usize, Token<'input>, usize),
) -> Box<Instruction<'input>>
{
    Box::new(Instruction::Par(Par(l, r)))
}

#[allow(unused_variables)]
fn __action5<
    'err,
    'input,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
    (_, __0, _): (usize, Token<'input>, usize),
    (_, __1, _): (usize, Token<'input>, usize),
    (_, __2, _): (usize, Token<'input>, usize),
) -> Box<Instruction<'input>>
{
    Box::new(Instruction::Null(Null))
}

#[allow(unused_variables)]
fn __action6<
    'err,
    'input,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
    (_, _, _): (usize, Token<'input>, usize),
    (_, _, _): (usize, Token<'input>, usize),
    (_, iterable, _): (usize, IterableValue<'input>, usize),
    (_, iterator, _): (usize, &'input str, usize),
    (_, i, _): (usize, Box<Instruction<'input>>, usize),
    (_, _, _): (usize, Token<'input>, usize),
) -> Box<Instruction<'input>>
{
    {
        let instruction = Rc::new(*i);
        Box::new(Instruction::Fold(Fold{ iterable, iterator, instruction }))
    }
}

#[allow(unused_variables)]
fn __action7<
    'err,
    'input,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
    (_, _, _): (usize, Token<'input>, usize),
    (_, _, _): (usize, Token<'input>, usize),
    (_, i, _): (usize, &'input str, usize),
    (_, _, _): (usize, Token<'input>, usize),
) -> Box<Instruction<'input>>
{
    Box::new(Instruction::Next(Next(i)))
}

#[allow(unused_variables)]
fn __action8<
    'err,
    'input,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
    (_, _, _): (usize, Token<'input>, usize),
    (_, _, _): (usize, Token<'input>, usize),
    (_, l, _): (usize, Box<Instruction<'input>>, usize),
    (_, r, _): (usize, Box<Instruction<'input>>, usize),
    (_, _, _): (usize, Token<'input>, usize),
) -> Box<Instruction<'input>>
{
    Box::new(Instruction::Xor(Xor(l, r)))
}

#[allow(unused_variables)]
fn __action9<
    'err,
    'input,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
    (_, _, _): (usize, Token<'input>, usize),
    (_, _, _): (usize, Token<'input>, usize),
    (_, l, _): (usize, MatchableValue<'input>, usize),
    (_, r, _): (usize, MatchableValue<'input>, usize),
    (_, i, _): (usize, Box<Instruction<'input>>, usize),
    (_, _, _): (usize, Token<'input>, usize),
) -> Box<Instruction<'input>>
{
    {
        let match_ = Match { left_value: l, right_value: r, instruction: i};
        Box::new(Instruction::Match(match_))
    }
}

#[allow(unused_variables)]
fn __action10<
    'err,
    'input,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
    (_, _, _): (usize, Token<'input>, usize),
    (_, _, _): (usize, Token<'input>, usize),
    (_, l, _): (usize, MatchableValue<'input>, usize),
    (_, r, _): (usize, MatchableValue<'input>, usize),
    (_, i, _): (usize, Box<Instruction<'input>>, usize),
    (_, _, _): (usize, Token<'input>, usize),
) -> Box<Instruction<'input>>
{
    {
        let mismatch = MisMatch { left_value: l, right_value: r, instruction: i};
        Box::new(Instruction::MisMatch(mismatch))
     }
}

#[allow(unused_variables)]
fn __action11<
    'err,
    'input,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
    (_, __0, _): (usize, __lalrpop_util::ErrorRecovery<usize, Token<'input>, LexerError>, usize),
) -> Box<Instruction<'input>>
{
    { errors.push(__0); Box::new(Instruction::Error) }
}

#[allow(unused_variables)]
fn __action12<
    'err,
    'input,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
    (_, _, _): (usize, Token<'input>, usize),
    (_, args, _): (usize, ::std::vec::Vec<CallArgValue<'input>>, usize),
    (_, _, _): (usize, Token<'input>, usize),
) -> Vec<CallArgValue<'input>>
{
    args
}

#[allow(unused_variables)]
fn __action13<
    'err,
    'input,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
    (_, f, _): (usize, CallArgValue<'input>, usize),
) -> FunctionPart<'input>
{
    FunctionPart::FuncName(f)
}

#[allow(unused_variables)]
fn __action14<
    'err,
    'input,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
    (_, _, _): (usize, Token<'input>, usize),
    (_, sid, _): (usize, CallArgValue<'input>, usize),
    (_, f, _): (usize, CallArgValue<'input>, usize),
    (_, _, _): (usize, Token<'input>, usize),
) -> FunctionPart<'input>
{
    FunctionPart::ServiceIdWithFuncName(sid, f)
}

#[allow(unused_variables)]
fn __action15<
    'err,
    'input,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
    (_, pid, _): (usize, CallArgValue<'input>, usize),
) -> PeerPart<'input>
{
    PeerPart::PeerPk(pid)
}

#[allow(unused_variables)]
fn __action16<
    'err,
    'input,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
    (_, _, _): (usize, Token<'input>, usize),
    (_, pid, _): (usize, CallArgValue<'input>, usize),
    (_, sid, _): (usize, CallArgValue<'input>, usize),
    (_, _, _): (usize, Token<'input>, usize),
) -> PeerPart<'input>
{
    PeerPart::PeerPkWithServiceId(pid, sid)
}

#[allow(unused_variables)]
fn __action17<
    'err,
    'input,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
    (_, s, _): (usize, &'input str, usize),
) -> CallOutputValue<'input>
{
    CallOutputValue::Scalar(s)
}

#[allow(unused_variables)]
fn __action18<
    'err,
    'input,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
    (_, a, _): (usize, &'input str, usize),
) -> CallOutputValue<'input>
{
    CallOutputValue::Accumulator(a)
}

#[allow(unused_variables)]
fn __action19<
    'err,
    'input,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
    (_, __0, _): (usize, CallArgValue<'input>, usize),
) -> CallArgValue<'input>
{
    __0
}

#[allow(unused_variables)]
fn __action20<
    'err,
    'input,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
    (_, __0, _): (usize, CallArgValue<'input>, usize),
) -> CallArgValue<'input>
{
    __0
}

#[allow(unused_variables)]
fn __action21<
    'err,
    'input,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
    (_, __0, _): (usize, CallArgValue<'input>, usize),
) -> CallArgValue<'input>
{
    __0
}

#[allow(unused_variables)]
fn __action22<
    'err,
    'input,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
    (_, __0, _): (usize, CallArgValue<'input>, usize),
) -> CallArgValue<'input>
{
    __0
}

#[allow(unused_variables)]
fn __action23<
    'err,
    'input,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
    (_, s, _): (usize, &'input str, usize),
) -> CallArgValue<'input>
{
    CallArgValue::Literal(s)
}

#[allow(unused_variables)]
fn __action24<
    'err,
    'input,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
    (_, s, _): (usize, &'input str, usize),
) -> CallArgValue<'input>
{
    CallArgValue::Variable(s)
}

#[allow(unused_variables)]
fn __action25<
    'err,
    'input,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
    (_, v, _): (usize, (&'input str, usize), usize),
) -> CallArgValue<'input>
{
    {
        let (variable, path) = into_variable_and_path(v.0, v.1);
        CallArgValue::JsonPath { variable, path }
    }
}

#[allow(unused_variables)]
fn __action26<
    'err,
    'input,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
    (_, __0, _): (usize, Token<'input>, usize),
) -> CallArgValue<'input>
{
    CallArgValue::InitPeerId
}

#[allow(unused_variables)]
fn __action27<
    'err,
    'input,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
    (_, s, _): (usize, &'input str, usize),
) -> IterableValue<'input>
{
    IterableValue::Variable(s)
}

#[allow(unused_variables)]
fn __action28<
    'err,
    'input,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
    (_, v, _): (usize, (&'input str, usize), usize),
) -> IterableValue<'input>
{
    {
        let (variable, path) = into_variable_and_path(v.0, v.1);
        IterableValue::JsonPath { variable, path }
    }
}

#[allow(unused_variables)]
fn __action29<
    'err,
    'input,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
    (_, s, _): (usize, &'input str, usize),
) -> MatchableValue<'input>
{
    MatchableValue::Variable(s)
}

#[allow(unused_variables)]
fn __action30<
    'err,
    'input,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
    (_, s, _): (usize, &'input str, usize),
) -> MatchableValue<'input>
{
    MatchableValue::Literal(s)
}

#[allow(unused_variables)]
fn __action31<
    'err,
    'input,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
    (_, v, _): (usize, (&'input str, usize), usize),
) -> MatchableValue<'input>
{
    {
        let (variable, path) = into_variable_and_path(v.0, v.1);
        MatchableValue::JsonPath { variable, path }
    }
}

#[allow(unused_variables)]
fn __action32<
    'err,
    'input,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<CallArgValue<'input>>
{
    vec![]
}

#[allow(unused_variables)]
fn __action33<
    'err,
    'input,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
    (_, v, _): (usize, ::std::vec::Vec<CallArgValue<'input>>, usize),
) -> ::std::vec::Vec<CallArgValue<'input>>
{
    v
}

#[allow(unused_variables)]
fn __action34<
    'err,
    'input,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
    (_, __0, _): (usize, CallArgValue<'input>, usize),
) -> CallArgValue<'input>
{
    __0
}

#[allow(unused_variables)]
fn __action35<
    'err,
    'input,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
    (_, __0, _): (usize, CallOutputValue<'input>, usize),
) -> ::std::option::Option<CallOutputValue<'input>>
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action36<
    'err,
    'input,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::option::Option<CallOutputValue<'input>>
{
    None
}

#[allow(unused_variables)]
fn __action37<
    'err,
    'input,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
    (_, __0, _): (usize, CallArgValue<'input>, usize),
) -> ::std::vec::Vec<CallArgValue<'input>>
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action38<
    'err,
    'input,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
    (_, v, _): (usize, ::std::vec::Vec<CallArgValue<'input>>, usize),
    (_, e, _): (usize, CallArgValue<'input>, usize),
) -> ::std::vec::Vec<CallArgValue<'input>>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action39<
    'err,
    'input,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
    __0: (usize, CallArgValue<'input>, usize),
) -> ::std::vec::Vec<CallArgValue<'input>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action34(
        input,
        errors,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action37(
        input,
        errors,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action40<
    'err,
    'input,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
    __0: (usize, ::std::vec::Vec<CallArgValue<'input>>, usize),
    __1: (usize, CallArgValue<'input>, usize),
) -> ::std::vec::Vec<CallArgValue<'input>>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action34(
        input,
        errors,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action38(
        input,
        errors,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action41<
    'err,
    'input,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
    __0: (usize, Token<'input>, usize),
    __1: (usize, Token<'input>, usize),
) -> Vec<CallArgValue<'input>>
{
    let __start0 = __0.2.clone();
    let __end0 = __1.0.clone();
    let __temp0 = __action32(
        input,
        errors,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action12(
        input,
        errors,
        __0,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action42<
    'err,
    'input,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
    __0: (usize, Token<'input>, usize),
    __1: (usize, ::std::vec::Vec<CallArgValue<'input>>, usize),
    __2: (usize, Token<'input>, usize),
) -> Vec<CallArgValue<'input>>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action33(
        input,
        errors,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action12(
        input,
        errors,
        __0,
        __temp0,
        __2,
    )
}

#[allow(unused_variables)]
fn __action43<
    'err,
    'input,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
    __0: (usize, Token<'input>, usize),
    __1: (usize, Token<'input>, usize),
    __2: (usize, PeerPart<'input>, usize),
    __3: (usize, FunctionPart<'input>, usize),
    __4: (usize, Vec<CallArgValue<'input>>, usize),
    __5: (usize, CallOutputValue<'input>, usize),
    __6: (usize, Token<'input>, usize),
) -> Box<Instruction<'input>>
{
    let __start0 = __5.0.clone();
    let __end0 = __5.2.clone();
    let __temp0 = __action35(
        input,
        errors,
        __5,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action2(
        input,
        errors,
        __0,
        __1,
        __2,
        __3,
        __4,
        __temp0,
        __6,
    )
}

#[allow(unused_variables)]
fn __action44<
    'err,
    'input,
>(
    input: &'input str,
    errors: &'err mut Vec<ErrorRecovery<usize, Token<'input>, LexerError>>,
    __0: (usize, Token<'input>, usize),
    __1: (usize, Token<'input>, usize),
    __2: (usize, PeerPart<'input>, usize),
    __3: (usize, FunctionPart<'input>, usize),
    __4: (usize, Vec<CallArgValue<'input>>, usize),
    __5: (usize, Token<'input>, usize),
) -> Box<Instruction<'input>>
{
    let __start0 = __4.2.clone();
    let __end0 = __5.0.clone();
    let __temp0 = __action36(
        input,
        errors,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action2(
        input,
        errors,
        __0,
        __1,
        __2,
        __3,
        __4,
        __temp0,
        __5,
    )
}

pub trait __ToTriple<'err, 'input, > {
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, LexerError>>;
}

impl<'err, 'input, > __ToTriple<'err, 'input, > for (usize, Token<'input>, usize) {
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, LexerError>> {
        Ok(value)
    }
}
impl<'err, 'input, > __ToTriple<'err, 'input, > for Result<(usize, Token<'input>, usize), LexerError> {
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, LexerError>> {
        match value {
            Ok(v) => Ok(v),
            Err(error) => Err(__lalrpop_util::ParseError::User { error }),
        }
    }
}
