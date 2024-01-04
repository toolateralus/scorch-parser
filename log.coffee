
running 1 test
test tests::test_main ... FAILED

failures:

---- tests::test_main stdout ----
[src/parser/declaration.rs:169] target_t.clone() = Identifier(
    "Array",
)
[src/parser/declaration.rs:169] target_t.clone() = Identifier(
    "Array",
)
[src/parser/declaration.rs:169] target_t.clone() = Identifier(
    "Array",
)
[src/parser/keyword.rs:94] "starting function declaration." = "starting function declaration."
[src/parser/keyword.rs:110] "end function declaration." = "end function declaration."
[src/parser/keyword.rs:94] "starting function declaration." = "starting function declaration."
[src/parser/keyword.rs:110] "end function declaration." = "end function declaration."
thread 'tests::test_main' panicked at src/lib.rs:43:13:
Program(
    [
        DeclStmt {
            target_type: Identifier(
                "dynamic",
            ),
            target_id: Identifier(
                "ff_addition",
            ),
            expression: Some(
                BinaryOperation {
                    lhs: Double(
                        5.3,
                    ),
                    op: Add,
                    rhs: Double(
                        6.2,
                    ),
                },
            ),
            mutable: false,
        },
        DeclStmt {
            target_type: Identifier(
                "dynamic",
            ),
            target_id: Identifier(
                "x",
            ),
            expression: Some(
                BinaryOperation {
                    lhs: Identifier(
                        "Class",
                    ),
                    op: New,
                    rhs: Tuple(
                        [],
                    ),
                },
            ),
            mutable: false,
        },
        AssignStmnt {
            id: BinaryOperation {
                lhs: Identifier(
                    "x",
                ),
                op: Dot,
                rhs: BinaryOperation {
                    lhs: Identifier(
                        "members",
                    ),
                    op: Dot,
                    rhs: Identifier(
                        "owner_id",
                    ),
                },
            },
            expression: Identifier(
                "none",
            ),
        },
        AssignStmnt {
            id: BinaryOperation {
                lhs: Identifier(
                    "x",
                ),
                op: Dot,
                rhs: BinaryOperation {
                    lhs: Identifier(
                        "cached",
                    ),
                    op: Dot,
                    rhs: Identifier(
                        "last",
                    ),
                },
            },
            expression: BinaryOperation {
                lhs: Identifier(
                    "x",
                ),
                op: Dot,
                rhs: BinaryOperation {
                    lhs: Identifier(
                        "y",
                    ),
                    op: OpenParenthesis,
                    rhs: Tuple(
                        [],
                    ),
                },
            },
        },
        DeclStmt {
            target_type: Identifier(
                "dynamic",
            ),
            target_id: Identifier(
                "xaryu",
            ),
            expression: Some(
                BinaryOperation {
                    lhs: Identifier(
                        "Xar",
                    ),
                    op: New,
                    rhs: Tuple(
                        [
                            BinaryOperation {
                                lhs: BinaryOperation {
                                    lhs: Identifier(
                                        "x",
                                    ),
                                    op: Dot,
                                    rhs: BinaryOperation {
                                        lhs: Identifier(
                                            "y",
                                        ),
                                        op: Dot,
                                        rhs: Identifier(
                                            "z",
                                        ),
                                    },
                                },
                                op: Equals,
                                rhs: BinaryOperation {
                                    lhs: Identifier(
                                        "z",
                                    ),
                                    op: Dot,
                                    rhs: Identifier(
                                        "x",
                                    ),
                                },
                            },
                        ],
                    ),
                },
            ),
            mutable: false,
        },
        BinaryOperation {
            lhs: Identifier(
                "assert",
            ),
            op: OpenParenthesis,
            rhs: Tuple(
                [
                    BinaryOperation {
                        lhs: BinaryOperation {
                            lhs: Identifier(
                                "x",
                            ),
                            op: Dot,
                            rhs: BinaryOperation {
                                lhs: Identifier(
                                    "funcy_test",
                                ),
                                op: OpenParenthesis,
                                rhs: Tuple(
                                    [],
                                ),
                            },
                        },
                        op: Equals,
                        rhs: Int(
                            100,
                        ),
                    },
                    String(
                        "failed to call funcy_test",
                    ),
                ],
            ),
        },
        DeclStmt {
            target_type: Identifier(
                "dynamic",
            ),
            target_id: Identifier(
                "ff_addition",
            ),
            expression: Some(
                BinaryOperation {
                    lhs: Double(
                        5.3,
                    ),
                    op: Add,
                    rhs: Double(
                        6.2,
                    ),
                },
            ),
            mutable: false,
        },
        DeclStmt {
            target_type: Identifier(
                "dynamic",
            ),
            target_id: Identifier(
                "ff_subtraction",
            ),
            expression: Some(
                BinaryOperation {
                    lhs: Double(
                        5.3,
                    ),
                    op: Subtract,
                    rhs: Double(
                        6.2,
                    ),
                },
            ),
            mutable: false,
        },
        DeclStmt {
            target_type: Identifier(
                "dynamic",
            ),
            target_id: Identifier(
                "ff_multiplcation",
            ),
            expression: Some(
                BinaryOperation {
                    lhs: Double(
                        5.3,
                    ),
                    op: Multiply,
                    rhs: Double(
                        6.2,
                    ),
                },
            ),
            mutable: false,
        },
        DeclStmt {
            target_type: Identifier(
                "dynamic",
            ),
            target_id: Identifier(
                "ff_division",
            ),
            expression: Some(
                BinaryOperation {
                    lhs: Double(
                        5.3,
                    ),
                    op: Divide,
                    rhs: Double(
                        6.2,
                    ),
                },
            ),
            mutable: false,
        },
        DeclStmt {
            target_type: Identifier(
                "dynamic",
            ),
            target_id: Identifier(
                "ff_parenthesis_1",
            ),
            expression: Some(
                BinaryOperation {
                    lhs: Expression(
                        BinaryOperation {
                            lhs: Double(
                                5.3,
                            ),
                            op: Add,
                            rhs: Double(
                                6.2,
                            ),
                        },
                    ),
                    op: Multiply,
                    rhs: Double(
                        2.5,
                    ),
                },
            ),
            mutable: false,
        },
        DeclStmt {
            target_type: Identifier(
                "dynamic",
            ),
            target_id: Identifier(
                "ff_parenthesis_2",
            ),
            expression: Some(
                BinaryOperation {
                    lhs: Double(
                        5.3,
                    ),
                    op: Subtract,
                    rhs: Expression(
                        BinaryOperation {
                            lhs: Double(
                                6.2,
                            ),
                            op: Multiply,
                            rhs: Double(
                                3.1,
                            ),
                        },
                    ),
                },
            ),
            mutable: false,
        },
        DeclStmt {
            target_type: Identifier(
                "dynamic",
            ),
            target_id: Identifier(
                "ff_complex_1",
            ),
            expression: Some(
                BinaryOperation {
                    lhs: Expression(
                        BinaryOperation {
                            lhs: Double(
                                5.3,
                            ),
                            op: Add,
                            rhs: Double(
                                6.2,
                            ),
                        },
                    ),
                    op: Divide,
                    rhs: Expression(
                        BinaryOperation {
                            lhs: Double(
                                3.1,
                            ),
                            op: Subtract,
                            rhs: Double(
                                2.0,
                            ),
                        },
                    ),
                },
            ),
            mutable: false,
        },
        DeclStmt {
            target_type: Identifier(
                "dynamic",
            ),
            target_id: Identifier(
                "ff_complex_2",
            ),
            expression: Some(
                BinaryOperation {
                    lhs: Double(
                        5.3,
                    ),
                    op: Add,
                    rhs: BinaryOperation {
                        lhs: Expression(
                            BinaryOperation {
                                lhs: Double(
                                    6.2,
                                ),
                                op: Multiply,
                                rhs: Double(
                                    3.1,
                                ),
                            },
                        ),
                        op: Divide,
                        rhs: Double(
                            2.5,
                        ),
                    },
                },
            ),
            mutable: false,
        },
        DeclStmt {
            target_type: Identifier(
                "dynamic",
            ),
            target_id: Identifier(
                "ff_complex_3",
            ),
            expression: Some(
                BinaryOperation {
                    lhs: BinaryOperation {
                        lhs: Expression(
                            BinaryOperation {
                                lhs: Double(
                                    5.3,
                                ),
                                op: Subtract,
                                rhs: Double(
                                    6.2,
                                ),
                            },
                        ),
                        op: Multiply,
                        rhs: Double(
                            2.5,
                        ),
                    },
                    op: Divide,
                    rhs: Double(
                        3.1,
                    ),
                },
            ),
            mutable: false,
        },
        DeclStmt {
            target_type: Identifier(
                "dynamic",
            ),
            target_id: Identifier(
                "ff_complex_4",
            ),
            expression: Some(
                BinaryOperation {
                    lhs: BinaryOperation {
                        lhs: Double(
                            5.3,
                        ),
                        op: Divide,
                        rhs: Expression(
                            BinaryOperation {
                                lhs: Double(
                                    6.2,
                                ),
                                op: Add,
                                rhs: Double(
                                    3.1,
                                ),
                            },
                        ),
                    },
                    op: Multiply,
                    rhs: Double(
                        2.5,
                    ),
                },
            ),
            mutable: false,
        },
        DeclStmt {
            target_type: Identifier(
                "dynamic",
            ),
            target_id: Identifier(
                "ff_complex_5",
            ),
            expression: Some(
                BinaryOperation {
                    lhs: BinaryOperation {
                        lhs: Double(
                            5.3,
                        ),
                        op: Add,
                        rhs: Double(
                            6.2,
                        ),
                    },
                    op: Subtract,
                    rhs: BinaryOperation {
                        lhs: BinaryOperation {
                            lhs: Double(
                                3.1,
                            ),
                            op: Multiply,
                            rhs: Double(
                                2.0,
                            ),
                        },
                        op: Divide,
                        rhs: Double(
                            1.5,
                        ),
                    },
                },
            ),
            mutable: false,
        },
        DeclStmt {
            target_type: Identifier(
                "dynamic",
            ),
            target_id: Identifier(
                "ff_complex_6",
            ),
            expression: Some(
                BinaryOperation {
                    lhs: BinaryOperation {
                        lhs: Expression(
                            BinaryOperation {
                                lhs: Expression(
                                    BinaryOperation {
                                        lhs: Double(
                                            5.3,
                                        ),
                                        op: Multiply,
                                        rhs: Double(
                                            2.5,
                                        ),
                                    },
                                ),
                                op: Subtract,
                                rhs: Double(
                                    6.2,
                                ),
                            },
                        ),
                        op: Divide,
                        rhs: Double(
                            3.1,
                        ),
                    },
                    op: Add,
                    rhs: Double(
                        1.0,
                    ),
                },
            ),
            mutable: false,
        },
        DeclStmt {
            target_type: Identifier(
                "dynamic",
            ),
            target_id: Identifier(
                "empty_implicit",
            ),
            expression: Some(
                Array {
                    typename: Identifier(
                        "array",
                    ),
                    elements: [],
                    init_capacity: 0,
                    mutable: true,
                    elements_mutable: true,
                },
            ),
            mutable: false,
        },
        BinaryOperation {
            lhs: Identifier(
                "assert",
            ),
            op: OpenParenthesis,
            rhs: Tuple(
                [
                    BinaryOperation {
                        lhs: BinaryOperation {
                            lhs: Identifier(
                                "len",
                            ),
                            op: OpenParenthesis,
                            rhs: Tuple(
                                [
                                    Identifier(
                                        "empty_implicit",
                                    ),
                                ],
                            ),
                        },
                        op: Equals,
                        rhs: Int(
                            0,
                        ),
                    },
                    String(
                        "empty_implicit array failed to be empty",
                    ),
                ],
            ),
        },
        DeclStmt {
            target_type: Identifier(
                "Array",
            ),
            target_id: Identifier(
                "empty_explicit",
            ),
            expression: Some(
                Array {
                    typename: Identifier(
                        "array",
                    ),
                    elements: [],
                    init_capacity: 0,
                    mutable: true,
                    elements_mutable: true,
                },
            ),
            mutable: false,
        },
        BinaryOperation {
            lhs: Identifier(
                "assert",
            ),
            op: OpenParenthesis,
            rhs: Tuple(
                [
                    BinaryOperation {
                        lhs: BinaryOperation {
                            lhs: Identifier(
                                "len",
                            ),
                            op: OpenParenthesis,
                            rhs: Tuple(
                                [
                                    Identifier(
                                        "empty_explicit",
                                    ),
                                ],
                            ),
                        },
                        op: Equals,
                        rhs: Int(
                            0,
                        ),
                    },
                    String(
                        "empty_explicit array failed to be empty",
                    ),
                ],
            ),
        },
        DeclStmt {
            target_type: Identifier(
                "dynamic",
            ),
            target_id: Identifier(
                "single_float_implicit",
            ),
            expression: Some(
                Array {
                    typename: Identifier(
                        "array",
                    ),
                    elements: [
                        Double(
                            1.0,
                        ),
                    ],
                    init_capacity: 1,
                    mutable: true,
                    elements_mutable: true,
                },
            ),
            mutable: false,
        },
        BinaryOperation {
            lhs: Identifier(
                "assert",
            ),
            op: OpenParenthesis,
            rhs: Tuple(
                [
                    BinaryOperation {
                        lhs: BinaryOperation {
                            lhs: Identifier(
                                "len",
                            ),
                            op: OpenParenthesis,
                            rhs: Tuple(
                                [
                                    Identifier(
                                        "single_float_implicit",
                                    ),
                                ],
                            ),
                        },
                        op: Equals,
                        rhs: Int(
                            1,
                        ),
                    },
                    String(
                        "single_float_implicit array failed to have length of 1",
                    ),
                ],
            ),
        },
        DeclStmt {
            target_type: Identifier(
                "Array",
            ),
            target_id: Identifier(
                "single_float_explicit",
            ),
            expression: Some(
                Array {
                    typename: Identifier(
                        "array",
                    ),
                    elements: [
                        Double(
                            1.0,
                        ),
                    ],
                    init_capacity: 1,
                    mutable: true,
                    elements_mutable: true,
                },
            ),
            mutable: false,
        },
        BinaryOperation {
            lhs: Identifier(
                "assert",
            ),
            op: OpenParenthesis,
            rhs: Tuple(
                [
                    BinaryOperation {
                        lhs: BinaryOperation {
                            lhs: Identifier(
                                "len",
                            ),
                            op: OpenParenthesis,
                            rhs: Tuple(
                                [
                                    Identifier(
                                        "single_float_explicit",
                                    ),
                                ],
                            ),
                        },
                        op: Equals,
                        rhs: Int(
                            1,
                        ),
                    },
                    String(
                        "single_float_explicit array failed to have length of 1",
                    ),
                ],
            ),
        },
        DeclStmt {
            target_type: Identifier(
                "dynamic",
            ),
            target_id: Identifier(
                "plural_float_implicit",
            ),
            expression: Some(
                Array {
                    typename: Identifier(
                        "array",
                    ),
                    elements: [
                        Double(
                            1.0,
                        ),
                        Double(
                            2.0,
                        ),
                    ],
                    init_capacity: 2,
                    mutable: true,
                    elements_mutable: true,
                },
            ),
            mutable: false,
        },
        BinaryOperation {
            lhs: Identifier(
                "assert",
            ),
            op: OpenParenthesis,
            rhs: Tuple(
                [
                    BinaryOperation {
                        lhs: BinaryOperation {
                            lhs: Identifier(
                                "len",
                            ),
                            op: OpenParenthesis,
                            rhs: Tuple(
                                [
                                    Identifier(
                                        "plural_float_implicit",
                                    ),
                                ],
                            ),
                        },
                        op: Equals,
                        rhs: Int(
                            2,
                        ),
                    },
                    String(
                        "plural_float_implicit array failed to have length of 2",
                    ),
                ],
            ),
        },
        DeclStmt {
            target_type: Identifier(
                "Array",
            ),
            target_id: Identifier(
                "plural_float_explicit",
            ),
            expression: Some(
                Array {
                    typename: Identifier(
                        "array",
                    ),
                    elements: [
                        Double(
                            1.0,
                        ),
                        Double(
                            2.0,
                        ),
                    ],
                    init_capacity: 2,
                    mutable: true,
                    elements_mutable: true,
                },
            ),
            mutable: false,
        },
        BinaryOperation {
            lhs: Identifier(
                "assert",
            ),
            op: OpenParenthesis,
            rhs: Tuple(
                [
                    BinaryOperation {
                        lhs: BinaryOperation {
                            lhs: Identifier(
                                "len",
                            ),
                            op: OpenParenthesis,
                            rhs: Tuple(
                                [
                                    Identifier(
                                        "plural_float_explicit",
                                    ),
                                ],
                            ),
                        },
                        op: Equals,
                        rhs: Int(
                            2,
                        ),
                    },
                    String(
                        "plural_float_explicit array failed to have length of 2",
                    ),
                ],
            ),
        },
        DeclStmt {
            target_type: Identifier(
                "dynamic",
            ),
            target_id: Identifier(
                "assignment",
            ),
            expression: Some(
                Identifier(
                    "single_float_implicit",
                ),
            ),
            mutable: false,
        },
        BinaryOperation {
            lhs: Identifier(
                "assert",
            ),
            op: OpenParenthesis,
            rhs: Tuple(
                [
                    BinaryOperation {
                        lhs: BinaryOperation {
                            lhs: Identifier(
                                "len",
                            ),
                            op: OpenParenthesis,
                            rhs: Tuple(
                                [
                                    Identifier(
                                        "assignment",
                                    ),
                                ],
                            ),
                        },
                        op: Equals,
                        rhs: Int(
                            1,
                        ),
                    },
                    String(
                        "assignment array failed to have length of 1",
                    ),
                ],
            ),
        },
        DeclStmt {
            target_type: Identifier(
                "dynamic",
            ),
            target_id: Identifier(
                "single_element_access",
            ),
            expression: Some(
                BinaryOperation {
                    lhs: Identifier(
                        "single_float_implicit",
                    ),
                    op: OpenBracket,
                    rhs: Int(
                        0,
                    ),
                },
            ),
            mutable: false,
        },
        BinaryOperation {
            lhs: Identifier(
                "assert",
            ),
            op: OpenParenthesis,
            rhs: Tuple(
                [
                    BinaryOperation {
                        lhs: Identifier(
                            "single_element_access",
                        ),
                        op: Equals,
                        rhs: Double(
                            1.0,
                        ),
                    },
                    String(
                        "single_element_access failed to equal expected value",
                    ),
                ],
            ),
        },
        DeclStmt {
            target_type: Identifier(
                "dynamic",
            ),
            target_id: Identifier(
                "first_element_access",
            ),
            expression: Some(
                BinaryOperation {
                    lhs: Identifier(
                        "plural_float_implicit",
                    ),
                    op: OpenBracket,
                    rhs: Int(
                        0,
                    ),
                },
            ),
            mutable: false,
        },
        BinaryOperation {
            lhs: Identifier(
                "assert",
            ),
            op: OpenParenthesis,
            rhs: Tuple(
                [
                    BinaryOperation {
                        lhs: Identifier(
                            "first_element_access",
                        ),
                        op: Equals,
                        rhs: Double(
                            1.0,
                        ),
                    },
                    String(
                        "first_element_access failed to equal expected value",
                    ),
                ],
            ),
        },
        DeclStmt {
            target_type: Identifier(
                "dynamic",
            ),
            target_id: Identifier(
                "second_element_access",
            ),
            expression: Some(
                BinaryOperation {
                    lhs: Identifier(
                        "plural_float_implicit",
                    ),
                    op: OpenBracket,
                    rhs: Int(
                        1,
                    ),
                },
            ),
            mutable: false,
        },
        BinaryOperation {
            lhs: Identifier(
                "assert",
            ),
            op: OpenParenthesis,
            rhs: Tuple(
                [
                    BinaryOperation {
                        lhs: Identifier(
                            "second_element_access",
                        ),
                        op: Equals,
                        rhs: Double(
                            2.0,
                        ),
                    },
                    String(
                        "second_element_access failed to equal expected value",
                    ),
                ],
            ),
        },
        DeclStmt {
            target_type: Identifier(
                "dynamic",
            ),
            target_id: Identifier(
                "accessor_assignment",
            ),
            expression: Some(
                Array {
                    typename: Identifier(
                        "array",
                    ),
                    elements: [
                        Double(
                            1.0,
                        ),
                        Double(
                            2.0,
                        ),
                    ],
                    init_capacity: 2,
                    mutable: true,
                    elements_mutable: true,
                },
            ),
            mutable: false,
        },
        BinaryOperation {
            lhs: Identifier(
                "assert",
            ),
            op: OpenParenthesis,
            rhs: Tuple(
                [
                    BinaryOperation {
                        lhs: BinaryOperation {
                            lhs: Identifier(
                                "len",
                            ),
                            op: OpenParenthesis,
                            rhs: Tuple(
                                [
                                    Identifier(
                                        "accessor_assignment",
                                    ),
                                ],
                            ),
                        },
                        op: Equals,
                        rhs: Int(
                            2,
                        ),
                    },
                    String(
                        "accessor_assignment array failed to have length of 2",
                    ),
                ],
            ),
        },
        AssignStmnt {
            id: BinaryOperation {
                lhs: Identifier(
                    "accessor_assignment",
                ),
                op: OpenBracket,
                rhs: Int(
                    0,
                ),
            },
            expression: Double(
                3.0,
            ),
        },
        BinaryOperation {
            lhs: Identifier(
                "assert",
            ),
            op: OpenParenthesis,
            rhs: Tuple(
                [
                    BinaryOperation {
                        lhs: BinaryOperation {
                            lhs: Identifier(
                                "accessor_assignment",
                            ),
                            op: OpenBracket,
                            rhs: Int(
                                0,
                            ),
                        },
                        op: Equals,
                        rhs: Double(
                            3.0,
                        ),
                    },
                    String(
                        "accessor_assignment[0] failed to equal expected value",
                    ),
                ],
            ),
        },
        DeclStmt {
            target_type: Identifier(
                "dynamic",
            ),
            target_id: Identifier(
                "x",
            ),
            expression: Some(
                Double(
                    100.0,
                ),
            ),
            mutable: false,
        },
        BinaryOperation {
            lhs: Identifier(
                "assert",
            ),
            op: OpenParenthesis,
            rhs: Tuple(
                [
                    BinaryOperation {
                        lhs: Double(
                            100.0,
                        ),
                        op: Equals,
                        rhs: Identifier(
                            "x",
                        ),
                    },
                    String(
                        "x failed to equal expected value",
                    ),
                ],
            ),
        },
        DeclStmt {
            target_type: Identifier(
                "dynamic",
            ),
            target_id: Identifier(
                "vvz",
            ),
            expression: Some(
                NegOp(
                    Int(
                        10,
                    ),
                ),
            ),
            mutable: false,
        },
        BinaryOperation {
            lhs: Identifier(
                "assert",
            ),
            op: OpenParenthesis,
            rhs: Tuple(
                [
                    BinaryOperation {
                        lhs: NegOp(
                            Int(
                                10,
                            ),
                        ),
                        op: Equals,
                        rhs: Identifier(
                            "vvz",
                        ),
                    },
                    String(
                        "x failed to equal expected value",
                    ),
                ],
            ),
        },
        DeclStmt {
            target_type: Identifier(
                "dynamic",
            ),
            target_id: Identifier(
                "not_true",
            ),
            expression: Some(
                NotOp(
                    Bool(
                        true,
                    ),
                ),
            ),
            mutable: false,
        },
        BinaryOperation {
            lhs: Identifier(
                "assert",
            ),
            op: OpenParenthesis,
            rhs: Tuple(
                [
                    BinaryOperation {
                        lhs: Bool(
                            false,
                        ),
                        op: Equals,
                        rhs: Identifier(
                            "not_true",
                        ),
                    },
                    String(
                        "not_true failed to equal expected value",
                    ),
                ],
            ),
        },
        DeclStmt {
            target_type: Identifier(
                "dynamic",
            ),
            target_id: Identifier(
                "not_false",
            ),
            expression: Some(
                NotOp(
                    Bool(
                        false,
                    ),
                ),
            ),
            mutable: false,
        },
        BinaryOperation {
            lhs: Identifier(
                "assert",
            ),
            op: OpenParenthesis,
            rhs: Tuple(
                [
                    BinaryOperation {
                        lhs: Bool(
                            true,
                        ),
                        op: Equals,
                        rhs: Identifier(
                            "not_false",
                        ),
                    },
                    String(
                        "not_false failed to equal expected value",
                    ),
                ],
            ),
        },
        DeclStmt {
            target_type: Identifier(
                "dynamic",
            ),
            target_id: Identifier(
                "my_Bool",
            ),
            expression: Some(
                Bool(
                    true,
                ),
            ),
            mutable: false,
        },
        BinaryOperation {
            lhs: Identifier(
                "assert",
            ),
            op: OpenParenthesis,
            rhs: Tuple(
                [
                    BinaryOperation {
                        lhs: Bool(
                            true,
                        ),
                        op: Equals,
                        rhs: Identifier(
                            "my_Bool",
                        ),
                    },
                    String(
                        "my_Bool failed to equal expected value",
                    ),
                ],
            ),
        },
        DeclStmt {
            target_type: Identifier(
                "dynamic",
            ),
            target_id: Identifier(
                "first_name",
            ),
            expression: Some(
                String(
                    "Cyitlec",
                ),
            ),
            mutable: false,
        },
        BinaryOperation {
            lhs: Identifier(
                "assert",
            ),
            op: OpenParenthesis,
            rhs: Tuple(
                [
                    BinaryOperation {
                        lhs: String(
                            "Cyitlec",
                        ),
                        op: Equals,
                        rhs: Identifier(
                            "first_name",
                        ),
                    },
                    String(
                        "first_name failed to equal expected value",
                    ),
                ],
            ),
        },
        DeclStmt {
            target_type: Identifier(
                "dynamic",
            ),
            target_id: Identifier(
                "last_name",
            ),
            expression: Some(
                String(
                    "Kivals",
                ),
            ),
            mutable: false,
        },
        BinaryOperation {
            lhs: Identifier(
                "assert",
            ),
            op: OpenParenthesis,
            rhs: Tuple(
                [
                    BinaryOperation {
                        lhs: String(
                            "Kivals",
                        ),
                        op: Equals,
                        rhs: Identifier(
                            "last_name",
                        ),
                    },
                    String(
                        "last_name failed to equal expected value",
                    ),
                ],
            ),
        },
        DeclStmt {
            target_type: Identifier(
                "dynamic",
            ),
            target_id: Identifier(
                "full_name",
            ),
            expression: Some(
                BinaryOperation {
                    lhs: BinaryOperation {
                        lhs: Identifier(
                            "first_name",
                        ),
                        op: Add,
                        rhs: String(
                            " ",
                        ),
                    },
                    op: Add,
                    rhs: Identifier(
                        "last_name",
                    ),
                },
            ),
            mutable: false,
        },
        BinaryOperation {
            lhs: Identifier(
                "assert",
            ),
            op: OpenParenthesis,
            rhs: Tuple(
                [
                    BinaryOperation {
                        lhs: String(
                            "Cyitlec Kivals",
                        ),
                        op: Equals,
                        rhs: Identifier(
                            "full_name",
                        ),
                    },
                    String(
                        "full_name failed to equal expected value",
                    ),
                ],
            ),
        },
        DeclStmt {
            target_type: Identifier(
                "dynamic",
            ),
            target_id: Identifier(
                "birthplace",
            ),
            expression: Some(
                String(
                    "The\nMost\nAmerican\nAmerican \nCity \nin \nAmerica",
                ),
            ),
            mutable: false,
        },
        BinaryOperation {
            lhs: Identifier(
                "assert",
            ),
            op: OpenParenthesis,
            rhs: Tuple(
                [
                    BinaryOperation {
                        lhs: String(
                            "The\nMost\nAmerican\nAmerican \nCity \nin \nAmerica",
                        ),
                        op: Equals,
                        rhs: Identifier(
                            "birthplace",
                        ),
                    },
                    BinaryOperation {
                        lhs: String(
                            "birthplace failed to equal expected value, instead got",
                        ),
                        op: Add,
                        rhs: BinaryOperation {
                            lhs: Identifier(
                                "tostr",
                            ),
                            op: OpenParenthesis,
                            rhs: Tuple(
                                [
                                    Identifier(
                                        "birthplace",
                                    ),
                                ],
                            ),
                        },
                    },
                ],
            ),
        },
        DeclStmt {
            target_type: Identifier(
                "dynamic",
            ),
            target_id: Identifier(
                "age",
            ),
            expression: Some(
                String(
                    "mid 20s",
                ),
            ),
            mutable: false,
        },
        DeclStmt {
            target_type: Identifier(
                "dynamic",
            ),
            target_id: Identifier(
                "height",
            ),
            expression: Some(
                String(
                    "6 feet, 3.758168 inches",
                ),
            ),
            mutable: false,
        },
        DeclStmt {
            target_type: Identifier(
                "dynamic",
            ),
            target_id: Identifier(
                "status",
            ),
            expression: Some(
                String(
                    "failed",
                ),
            ),
            mutable: false,
        },
        FuncDeclStmnt {
            id: Identifier(
                "implicit_fn_test_params",
            ),
            body: Block(
                [
                    BinaryOperation {
                        lhs: Identifier(
                            "println",
                        ),
                        op: OpenParenthesis,
                        rhs: Tuple(
                            [
                                String(
                                    "recursive func ",
                                ),
                                Identifier(
                                    "a",
                                ),
                            ],
                        ),
                    },
                    AssignStmnt {
                        id: Identifier(
                            "status",
                        ),
                        expression: String(
                            "passed",
                        ),
                    },
                ],
            ),
            params: [
                ParamDecl {
                    varname: Identifier(
                        "a",
                    ),
                    typename: Identifier(
                        "int",
                    ),
                },
            ],
            return_t: Identifier(
                "none",
            ),
            mutable: false,
        },
        FuncDeclStmnt {
            id: Identifier(
                "implicit_fn_test_no_params",
            ),
            body: Block(
                [
                    AssignStmnt {
                        id: Identifier(
                            "status",
                        ),
                        expression: String(
                            "passed",
                        ),
                    },
                    BinaryOperation {
                        lhs: Identifier(
                            "println",
                        ),
                        op: OpenParenthesis,
                        rhs: Tuple(
                            [
                                String(
                                    "recursive func paramless",
                                ),
                            ],
                        ),
                    },
                ],
            ),
            params: [],
            return_t: Identifier(
                "none",
            ),
            mutable: false,
        },
        BinaryOperation {
            lhs: Identifier(
                "implicit_fn_test_params",
            ),
            op: OpenParenthesis,
            rhs: Tuple(
                [
                    String(
                        "my argument",
                    ),
                ],
            ),
        },
        BinaryOperation {
            lhs: Identifier(
                "assert",
            ),
            op: OpenParenthesis,
            rhs: Tuple(
                [
                    BinaryOperation {
                        lhs: Identifier(
                            "status",
                        ),
                        op: Equals,
                        rhs: String(
                            "passed",
                        ),
                    },
                    String(
                        "failed to pass functions.scorch",
                    ),
                ],
            ),
        },
        BinaryOperation {
            lhs: Identifier(
                "implicit_fn_test_no_params",
            ),
            op: OpenParenthesis,
            rhs: Tuple(
                [],
            ),
        },
        DeclStmt {
            target_type: Identifier(
                "dynamic",
            ),
            target_id: Identifier(
                "status",
            ),
            expression: Some(
                String(
                    "failed",
                ),
            ),
            mutable: false,
        },
        DeclStmt {
            target_type: Identifier(
                "dynamic",
            ),
            target_id: Identifier(
                "fail_status",
            ),
            expression: Some(
                String(
                    "failed",
                ),
            ),
            mutable: false,
        },
        DeclStmt {
            target_type: Identifier(
                "dynamic",
            ),
            target_id: Identifier(
                "pass_status",
            ),
            expression: Some(
                String(
                    "passed",
                ),
            ),
            mutable: false,
        },
        IfStmnt {
            condition: Bool(
                false,
            ),
            block: Block(
                [
                    AssignStmnt {
                        id: Identifier(
                            "status",
                        ),
                        expression: Identifier(
                            "fail_status",
                        ),
                    },
                ],
            ),
            else_stmnt: Some(
                ElseStmnt {
                    condition: Some(
                        Bool(
                            false,
                        ),
                    ),
                    block: Block(
                        [
                            AssignStmnt {
                                id: Identifier(
                                    "status",
                                ),
                                expression: Identifier(
                                    "fail_status",
                                ),
                            },
                        ],
                    ),
                    else_stmnt: Some(
                        ElseStmnt {
                            condition: None,
                            block: Block(
                                [
                                    AssignStmnt {
                                        id: Identifier(
                                            "status",
                                        ),
                                        expression: Identifier(
                                            "pass_status",
                                        ),
                                    },
                                ],
                            ),
                            else_stmnt: None,
                        },
                    ),
                },
            ),
        },
        BinaryOperation {
            lhs: Identifier(
                "assert",
            ),
            op: OpenParenthesis,
            rhs: Tuple(
                [
                    BinaryOperation {
                        lhs: Identifier(
                            "status",
                        ),
                        op: Equals,
                        rhs: String(
                            "passed",
                        ),
                    },
                    String(
                        "failed to pass if_else.scorch",
                    ),
                ],
            ),
        },
        IfStmnt {
            condition: Bool(
                true,
            ),
            block: Block(
                [
                    AssignStmnt {
                        id: Identifier(
                            "status",
                        ),
                        expression: Identifier(
                            "pass_status",
                        ),
                    },
                ],
            ),
            else_stmnt: Some(
                ElseStmnt {
                    condition: Some(
                        Bool(
                            false,
                        ),
                    ),
                    block: Block(
                        [
                            AssignStmnt {
                                id: Identifier(
                                    "status",
                                ),
                                expression: Identifier(
                                    "fail_status",
                                ),
                            },
                        ],
                    ),
                    else_stmnt: Some(
                        ElseStmnt {
                            condition: None,
                            block: Block(
                                [
                                    AssignStmnt {
                                        id: Identifier(
                                            "status",
                                        ),
                                        expression: Identifier(
                                            "fail_status",
                                        ),
                                    },
                                ],
                            ),
                            else_stmnt: None,
                        },
                    ),
                },
            ),
        },
        BinaryOperation {
            lhs: Identifier(
                "assert",
            ),
            op: OpenParenthesis,
            rhs: Tuple(
                [
                    BinaryOperation {
                        lhs: Identifier(
                            "status",
                        ),
                        op: Equals,
                        rhs: String(
                            "passed",
                        ),
                    },
                    String(
                        "failed to pass if_else.scorch",
                    ),
                ],
            ),
        },
        DeclStmt {
            target_type: Identifier(
                "dynamic",
            ),
            target_id: Identifier(
                "xx",
            ),
            expression: Some(
                Bool(
                    false,
                ),
            ),
            mutable: false,
        },
        DeclStmt {
            target_type: Identifier(
                "dynamic",
            ),
            target_id: Identifier(
                "xy",
            ),
            expression: Some(
                Bool(
                    true,
                ),
            ),
            mutable: false,
        },
        IfStmnt {
            condition: BinaryOperation {
                lhs: Identifier(
                    "xx",
                ),
                op: NotEquals,
                rhs: Identifier(
                    "xy",
                ),
            },
            block: Block(
                [
                    AssignStmnt {
                        id: Identifier(
                            "status",
                        ),
                        expression: Identifier(
                            "pass_status",
                        ),
                    },
                ],
            ),
            else_stmnt: Some(
                ElseStmnt {
                    condition: None,
                    block: Block(
                        [
                            AssignStmnt {
                                id: Identifier(
                                    "status",
                                ),
                                expression: Identifier(
                                    "fail_status",
                                ),
                            },
                        ],
                    ),
                    else_stmnt: None,
                },
            ),
        },
        BinaryOperation {
            lhs: Identifier(
                "assert",
            ),
            op: OpenParenthesis,
            rhs: Tuple(
                [
                    BinaryOperation {
                        lhs: Identifier(
                            "status",
                        ),
                        op: Equals,
                        rhs: String(
                            "passed",
                        ),
                    },
                    String(
                        "failed to pass if_else.scorch",
                    ),
                ],
            ),
        },
        IfStmnt {
            condition: BinaryOperation {
                lhs: Identifier(
                    "xx",
                ),
                op: Equals,
                rhs: Identifier(
                    "xy",
                ),
            },
            block: Block(
                [
                    AssignStmnt {
                        id: Identifier(
                            "status",
                        ),
                        expression: Identifier(
                            "fail_status",
                        ),
                    },
                ],
            ),
            else_stmnt: Some(
                ElseStmnt {
                    condition: None,
                    block: Block(
                        [
                            AssignStmnt {
                                id: Identifier(
                                    "status",
                                ),
                                expression: Identifier(
                                    "pass_status",
                                ),
                            },
                        ],
                    ),
                    else_stmnt: None,
                },
            ),
        },
        DeclStmt {
            target_type: Identifier(
                "dynamic",
            ),
            target_id: Identifier(
                "xxy",
            ),
            expression: Some(
                Int(
                    5,
                ),
            ),
            mutable: false,
        },
        DeclStmt {
            target_type: Identifier(
                "dynamic",
            ),
            target_id: Identifier(
                "xyy",
            ),
            expression: Some(
                Int(
                    10,
                ),
            ),
            mutable: false,
        },
        BinaryOperation {
            lhs: Identifier(
                "assert",
            ),
            op: OpenParenthesis,
            rhs: Tuple(
                [
                    BinaryOperation {
                        lhs: Identifier(
                            "status",
                        ),
                        op: Equals,
                        rhs: String(
                            "passed",
                        ),
                    },
                    String(
                        "failed to pass if_else.scorch",
                    ),
                ],
            ),
        },
        IfStmnt {
            condition: BinaryOperation {
                lhs: BinaryOperation {
                    lhs: Identifier(
                        "xxy",
                    ),
                    op: LeftAngle,
                    rhs: Identifier(
                        "xyy",
                    ),
                },
                op: LogicalAnd,
                rhs: Expression(
                    BinaryOperation {
                        lhs: Identifier(
                            "xx",
                        ),
                        op: NotEquals,
                        rhs: Identifier(
                            "xy",
                        ),
                    },
                ),
            },
            block: Block(
                [
                    AssignStmnt {
                        id: Identifier(
                            "status",
                        ),
                        expression: Identifier(
                            "pass_status",
                        ),
                    },
                ],
            ),
            else_stmnt: Some(
                ElseStmnt {
                    condition: None,
                    block: Block(
                        [
                            AssignStmnt {
                                id: Identifier(
                                    "status",
                                ),
                                expression: Identifier(
                                    "fail_status",
                                ),
                            },
                        ],
                    ),
                    else_stmnt: None,
                },
            ),
        },
        BinaryOperation {
            lhs: Identifier(
                "assert",
            ),
            op: OpenParenthesis,
            rhs: Tuple(
                [
                    BinaryOperation {
                        lhs: Identifier(
                            "status",
                        ),
                        op: Equals,
                        rhs: String(
                            "passed",
                        ),
                    },
                    String(
                        "failed to pass if_else.scorch",
                    ),
                ],
            ),
        },
        IfStmnt {
            condition: BinaryOperation {
                lhs: BinaryOperation {
                    lhs: Identifier(
                        "xxy",
                    ),
                    op: RightAngle,
                    rhs: Identifier(
                        "xyy",
                    ),
                },
                op: LogicalAnd,
                rhs: Expression(
                    BinaryOperation {
                        lhs: Identifier(
                            "xx",
                        ),
                        op: NotEquals,
                        rhs: Identifier(
                            "xy",
                        ),
                    },
                ),
            },
            block: Block(
                [
                    AssignStmnt {
                        id: Identifier(
                            "status",
                        ),
                        expression: Identifier(
                            "fail_status",
                        ),
                    },
                ],
            ),
            else_stmnt: Some(
                ElseStmnt {
                    condition: None,
                    block: Block(
                        [
                            AssignStmnt {
                                id: Identifier(
                                    "status",
                                ),
                                expression: Identifier(
                                    "pass_status",
                                ),
                            },
                        ],
                    ),
                    else_stmnt: None,
                },
            ),
        },
        BinaryOperation {
            lhs: Identifier(
                "assert",
            ),
            op: OpenParenthesis,
            rhs: Tuple(
                [
                    BinaryOperation {
                        lhs: Identifier(
                            "status",
                        ),
                        op: Equals,
                        rhs: String(
                            "passed",
                        ),
                    },
                    String(
                        "failed to pass if_else.scorch",
                    ),
                ],
            ),
        },
        DeclStmt {
            target_type: Identifier(
                "dynamic",
            ),
            target_id: Identifier(
                "result",
            ),
            expression: Some(
                Bool(
                    false,
                ),
            ),
            mutable: false,
        },
        DeclStmt {
            target_type: Identifier(
                "dynamic",
            ),
            target_id: Identifier(
                "result1",
            ),
            expression: Some(
                Bool(
                    false,
                ),
            ),
            mutable: false,
        },
        DeclStmt {
            target_type: Identifier(
                "dynamic",
            ),
            target_id: Identifier(
                "result2",
            ),
            expression: Some(
                Bool(
                    false,
                ),
            ),
            mutable: false,
        },
        DeclStmt {
            target_type: Identifier(
                "dynamic",
            ),
            target_id: Identifier(
                "i",
            ),
            expression: Some(
                Int(
                    0,
                ),
            ),
            mutable: false,
        },
        BinaryOperation {
            lhs: Identifier(
                "assert",
            ),
            op: OpenParenthesis,
            rhs: Tuple(
                [
                    Identifier(
                        "result1",
                    ),
                    String(
                        "Test: Implicitly declared iterator while with condition failed.",
                    ),
                ],
            ),
        },
        WhileStmnt {
            condition: None,
            block: Block(
                [
                    ReturnStmnt(
                        None,
                    ),
                    BinaryOperation {
                        lhs: Identifier(
                            "println",
                        ),
                        op: OpenParenthesis,
                        rhs: Tuple(
                            [
                                String(
                                    "test: while without condition failing.. result is this infinite loop. please exit.",
                                ),
                            ],
                        ),
                    },
                ],
            ),
        },
        DeclStmt {
            target_type: Identifier(
                "dynamic",
            ),
            target_id: Identifier(
                "i",
            ),
            expression: Some(
                Int(
                    0,
                ),
            ),
            mutable: false,
        },
        WhileStmnt {
            condition: Some(
                BinaryOperation {
                    lhs: Identifier(
                        "i",
                    ),
                    op: LeftAngle,
                    rhs: Int(
                        10000,
                    ),
                },
            ),
            block: Block(
                [
                    AssignStmnt {
                        id: Identifier(
                            "i",
                        ),
                        expression: BinaryOperation {
                            lhs: Identifier(
                                "i",
                            ),
                            op: Add,
                            rhs: Int(
                                1,
                            ),
                        },
                    },
                    AssignStmnt {
                        id: Identifier(
                            "result",
                        ),
                        expression: Bool(
                            true,
                        ),
                    },
                ],
            ),
        },
        BinaryOperation {
            lhs: Identifier(
                "assert",
            ),
            op: OpenParenthesis,
            rhs: Tuple(
                [
                    BinaryOperation {
                        lhs: Identifier(
                            "result",
                        ),
                        op: LogicalAnd,
                        rhs: BinaryOperation {
                            lhs: Identifier(
                                "i",
                            ),
                            op: Equals,
                            rhs: Int(
                                10000,
                            ),
                        },
                    },
                    BinaryOperation {
                        lhs: String(
                            "Test: Cached while with condition failed, expected 10000 got ",
                        ),
                        op: Add,
                        rhs: BinaryOperation {
                            lhs: Identifier(
                                "tostr",
                            ),
                            op: OpenParenthesis,
                            rhs: Tuple(
                                [
                                    Identifier(
                                        "i",
                                    ),
                                ],
                            ),
                        },
                    },
                ],
            ),
        },
        WhileStmnt {
            condition: Some(
                BinaryOperation {
                    lhs: Identifier(
                        "z",
                    ),
                    op: LeftAngle,
                    rhs: Int(
                        250000,
                    ),
                },
            ),
            block: Block(
                [
                    IfStmnt {
                        condition: BinaryOperation {
                            lhs: Identifier(
                                "z",
                            ),
                            op: Equals,
                            rhs: Int(
                                249999,
                            ),
                        },
                        block: Block(
                            [
                                AssignStmnt {
                                    id: Identifier(
                                        "result1",
                                    ),
                                    expression: Bool(
                                        true,
                                    ),
                                },
                                ReturnStmnt(
                                    None,
                                ),
                            ],
                        ),
                        else_stmnt: None,
                    },
                ],
            ),
        },
        AssignStmnt {
            id: Identifier(
                "result2",
            ),
            expression: Bool(
                true,
            ),
        },
        BinaryOperation {
            lhs: Identifier(
                "assert",
            ),
            op: OpenParenthesis,
            rhs: Tuple(
                [
                    Identifier(
                        "result2",
                    ),
                    String(
                        "Test: Repeat without condition failed.",
                    ),
                ],
            ),
        },
        DeclStmt {
            target_type: Identifier(
                "dynamic",
            ),
            target_id: Identifier(
                "rel_t1",
            ),
            expression: Some(
                BinaryOperation {
                    lhs: Int(
                        5,
                    ),
                    op: LeftAngle,
                    rhs: Int(
                        10,
                    ),
                },
            ),
            mutable: false,
        },
        BinaryOperation {
            lhs: Identifier(
                "assert",
            ),
            op: OpenParenthesis,
            rhs: Tuple(
                [
                    BinaryOperation {
                        lhs: Identifier(
                            "rel_t1",
                        ),
                        op: Equals,
                        rhs: Bool(
                            true,
                        ),
                    },
                    String(
                        "5 < 10 returned true",
                    ),
                ],
            ),
        },
        DeclStmt {
            target_type: Identifier(
                "dynamic",
            ),
            target_id: Identifier(
                "rel_t2",
            ),
            expression: Some(
                BinaryOperation {
                    lhs: Int(
                        5,
                    ),
                    op: RightAngle,
                    rhs: Int(
                        10,
                    ),
                },
            ),
            mutable: false,
        },
        BinaryOperation {
            lhs: Identifier(
                "assert",
            ),
            op: OpenParenthesis,
            rhs: Tuple(
                [
                    BinaryOperation {
                        lhs: Identifier(
                            "rel_t2",
                        ),
                        op: Equals,
                        rhs: Bool(
                            false,
                        ),
                    },
                    String(
                        "5 > 10 returned true",
                    ),
                ],
            ),
        },
        DeclStmt {
            target_type: Identifier(
                "dynamic",
            ),
            target_id: Identifier(
                "rel_t3",
            ),
            expression: Some(
                BinaryOperation {
                    lhs: Int(
                        5,
                    ),
                    op: LessThanEquals,
                    rhs: Int(
                        10,
                    ),
                },
            ),
            mutable: false,
        },
        BinaryOperation {
            lhs: Identifier(
                "assert",
            ),
            op: OpenParenthesis,
            rhs: Tuple(
                [
                    BinaryOperation {
                        lhs: Identifier(
                            "rel_t3",
                        ),
                        op: Equals,
                        rhs: Bool(
                            true,
                        ),
                    },
                    String(
                        "5 <= 10 returned false",
                    ),
                ],
            ),
        },
        DeclStmt {
            target_type: Identifier(
                "dynamic",
            ),
            target_id: Identifier(
                "rel_t4",
            ),
            expression: Some(
                BinaryOperation {
                    lhs: Int(
                        5,
                    ),
                    op: GreaterThanEquals,
                    rhs: Int(
                        10,
                    ),
                },
            ),
            mutable: false,
        },
        BinaryOperation {
            lhs: Identifier(
                "assert",
            ),
            op: OpenParenthesis,
            rhs: Tuple(
                [
                    BinaryOperation {
                        lhs: Identifier(
                            "rel_t4",
                        ),
                        op: Equals,
                        rhs: Bool(
                            false,
                        ),
                    },
                    String(
                        "5 >= 10 returned true",
                    ),
                ],
            ),
        },
        DeclStmt {
            target_type: Identifier(
                "dynamic",
            ),
            target_id: Identifier(
                "rel_t5",
            ),
            expression: Some(
                BinaryOperation {
                    lhs: Int(
                        5,
                    ),
                    op: Equals,
                    rhs: Int(
                        10,
                    ),
                },
            ),
            mutable: false,
        },
        BinaryOperation {
            lhs: Identifier(
                "assert",
            ),
            op: OpenParenthesis,
            rhs: Tuple(
                [
                    BinaryOperation {
                        lhs: Identifier(
                            "rel_t5",
                        ),
                        op: Equals,
                        rhs: Bool(
                            false,
                        ),
                    },
                    String(
                        "5 == 10 returned true",
                    ),
                ],
            ),
        },
        DeclStmt {
            target_type: Identifier(
                "dynamic",
            ),
            target_id: Identifier(
                "rel_t6",
            ),
            expression: Some(
                BinaryOperation {
                    lhs: Int(
                        5,
                    ),
                    op: NotEquals,
                    rhs: Int(
                        10,
                    ),
                },
            ),
            mutable: false,
        },
        BinaryOperation {
            lhs: Identifier(
                "assert",
            ),
            op: OpenParenthesis,
            rhs: Tuple(
                [
                    BinaryOperation {
                        lhs: Identifier(
                            "rel_t6",
                        ),
                        op: Equals,
                        rhs: Bool(
                            true,
                        ),
                    },
                    String(
                        "5 != 10 returned false",
                    ),
                ],
            ),
        },
        DeclStmt {
            target_type: Identifier(
                "dynamic",
            ),
            target_id: Identifier(
                "rel_t7",
            ),
            expression: Some(
                BinaryOperation {
                    lhs: Int(
                        5,
                    ),
                    op: Equals,
                    rhs: Int(
                        5,
                    ),
                },
            ),
            mutable: false,
        },
        BinaryOperation {
            lhs: Identifier(
                "assert",
            ),
            op: OpenParenthesis,
            rhs: Tuple(
                [
                    BinaryOperation {
                        lhs: Identifier(
                            "rel_t7",
                        ),
                        op: Equals,
                        rhs: Bool(
                            true,
                        ),
                    },
                    String(
                        "5 == 5 returned false",
                    ),
                ],
            ),
        },
        DeclStmt {
            target_type: Identifier(
                "dynamic",
            ),
            target_id: Identifier(
                "rel_t8",
            ),
            expression: Some(
                BinaryOperation {
                    lhs: Int(
                        5,
                    ),
                    op: NotEquals,
                    rhs: Int(
                        5,
                    ),
                },
            ),
            mutable: false,
        },
        BinaryOperation {
            lhs: Identifier(
                "assert",
            ),
            op: OpenParenthesis,
            rhs: Tuple(
                [
                    BinaryOperation {
                        lhs: Identifier(
                            "rel_t8",
                        ),
                        op: Equals,
                        rhs: Bool(
                            false,
                        ),
                    },
                    String(
                        "5 != 5 returned true",
                    ),
                ],
            ),
        },
        DeclStmt {
            target_type: Identifier(
                "dynamic",
            ),
            target_id: Identifier(
                "rel_t9",
            ),
            expression: Some(
                BinaryOperation {
                    lhs: Int(
                        5,
                    ),
                    op: LessThanEquals,
                    rhs: Int(
                        5,
                    ),
                },
            ),
            mutable: false,
        },
        BinaryOperation {
            lhs: Identifier(
                "assert",
            ),
            op: OpenParenthesis,
            rhs: Tuple(
                [
                    BinaryOperation {
                        lhs: Identifier(
                            "rel_t9",
                        ),
                        op: Equals,
                        rhs: Bool(
                            true,
                        ),
                    },
                    String(
                        "5 <= 5 returned false",
                    ),
                ],
            ),
        },
        DeclStmt {
            target_type: Identifier(
                "dynamic",
            ),
            target_id: Identifier(
                "rel_t10",
            ),
            expression: Some(
                BinaryOperation {
                    lhs: Int(
                        5,
                    ),
                    op: GreaterThanEquals,
                    rhs: Int(
                        5,
                    ),
                },
            ),
            mutable: false,
        },
        BinaryOperation {
            lhs: Identifier(
                "assert",
            ),
            op: OpenParenthesis,
            rhs: Tuple(
                [
                    BinaryOperation {
                        lhs: Identifier(
                            "rel_t10",
                        ),
                        op: Equals,
                        rhs: Bool(
                            true,
                        ),
                    },
                    String(
                        "5 >= 5 returned false",
                    ),
                ],
            ),
        },
        DeclStmt {
            target_type: Identifier(
                "dynamic",
            ),
            target_id: Identifier(
                "rel_t11",
            ),
            expression: Some(
                BinaryOperation {
                    lhs: Int(
                        5,
                    ),
                    op: LeftAngle,
                    rhs: Int(
                        5,
                    ),
                },
            ),
            mutable: false,
        },
        BinaryOperation {
            lhs: Identifier(
                "assert",
            ),
            op: OpenParenthesis,
            rhs: Tuple(
                [
                    BinaryOperation {
                        lhs: Identifier(
                            "rel_t11",
                        ),
                        op: Equals,
                        rhs: Bool(
                            false,
                        ),
                    },
                    String(
                        "5 < 5 returned true",
                    ),
                ],
            ),
        },
        DeclStmt {
            target_type: Identifier(
                "dynamic",
            ),
            target_id: Identifier(
                "rel_t12",
            ),
            expression: Some(
                Expression(
                    BinaryOperation {
                        lhs: Expression(
                            BinaryOperation {
                                lhs: Int(
                                    5,
                                ),
                                op: LeftAngle,
                                rhs: Int(
                                    5,
                                ),
                            },
                        ),
                        op: LogicalAnd,
                        rhs: Expression(
                            BinaryOperation {
                                lhs: Int(
                                    5,
                                ),
                                op: RightAngle,
                                rhs: Int(
                                    5,
                                ),
                            },
                        ),
                    },
                ),
            ),
            mutable: false,
        },
        BinaryOperation {
            lhs: Identifier(
                "assert",
            ),
            op: OpenParenthesis,
            rhs: Tuple(
                [
                    BinaryOperation {
                        lhs: Identifier(
                            "rel_t12",
                        ),
                        op: Equals,
                        rhs: Bool(
                            false,
                        ),
                    },
                    String(
                        "5 < 5 && 5 > 5 returned true",
                    ),
                ],
            ),
        },
        DeclStmt {
            target_type: Identifier(
                "dynamic",
            ),
            target_id: Identifier(
                "status",
            ),
            expression: Some(
                Bool(
                    false,
                ),
            ),
            mutable: false,
        },
        StructDecl {
            id: Identifier(
                "Vec2",
            ),
            block: Block(
                [
                    DeclStmt {
                        target_type: Identifier(
                            "dynamic",
                        ),
                        target_id: Identifier(
                            "x",
                        ),
                        expression: Some(
                            Double(
                                0.0,
                            ),
                        ),
                        mutable: false,
                    },
                    DeclStmt {
                        target_type: Identifier(
                            "dynamic",
                        ),
                        target_id: Identifier(
                            "y",
                        ),
                        expression: Some(
                            Double(
                                0.0,
                            ),
                        ),
                        mutable: false,
                    },
                ],
            ),
        },
        DeclStmt {
            target_type: Identifier(
                "dynamic",
            ),
            target_id: Identifier(
                "vector2",
            ),
            expression: Some(
                BinaryOperation {
                    lhs: Identifier(
                        "Vec2",
                    ),
                    op: New,
                    rhs: Tuple(
                        [
                            Double(
                                0.0,
                            ),
                            Double(
                                1250.05,
                            ),
                        ],
                    ),
                },
            ),
            mutable: false,
        },
        BinaryOperation {
            lhs: Identifier(
                "assert_eq",
            ),
            op: OpenParenthesis,
            rhs: Tuple(
                [
                    Double(
                        0.0,
                    ),
                    BinaryOperation {
                        lhs: Identifier(
                            "vector2",
                        ),
                        op: Dot,
                        rhs: Identifier(
                            "x",
                        ),
                    },
                    String(
                        "Vec2.x failed to equal expected value",
                    ),
                ],
            ),
        },
        BinaryOperation {
            lhs: Identifier(
                "assert_eq",
            ),
            op: OpenParenthesis,
            rhs: Tuple(
                [
                    Double(
                        1250.05,
                    ),
                    BinaryOperation {
                        lhs: Identifier(
                            "vector2",
                        ),
                        op: Dot,
                        rhs: Identifier(
                            "y",
                        ),
                    },
                    String(
                        "Vec2.y failed to equal expected value",
                    ),
                ],
            ),
        },
        StructDecl {
            id: Identifier(
                "Vec3",
            ),
            block: Block(
                [
                    DeclStmt {
                        target_type: Identifier(
                            "dynamic",
                        ),
                        target_id: Identifier(
                            "xy",
                        ),
                        expression: Some(
                            BinaryOperation {
                                lhs: Identifier(
                                    "Vec2",
                                ),
                                op: New,
                                rhs: Tuple(
                                    [
                                        Double(
                                            0.0,
                                        ),
                                        Double(
                                            0.0,
                                        ),
                                    ],
                                ),
                            },
                        ),
                        mutable: false,
                    },
                    DeclStmt {
                        target_type: Identifier(
                            "dynamic",
                        ),
                        target_id: Identifier(
                            "z",
                        ),
                        expression: Some(
                            Double(
                                0.0,
                            ),
                        ),
                        mutable: false,
                    },
                ],
            ),
        },
        DeclStmt {
            target_type: Identifier(
                "dynamic",
            ),
            target_id: Identifier(
                "vector3",
            ),
            expression: Some(
                BinaryOperation {
                    lhs: Identifier(
                        "Vec3",
                    ),
                    op: New,
                    rhs: Tuple(
                        [
                            BinaryOperation {
                                lhs: Identifier(
                                    "Vec2",
                                ),
                                op: New,
                                rhs: Tuple(
                                    [
                                        Double(
                                            0.0,
                                        ),
                                        Double(
                                            1250.05,
                                        ),
                                    ],
                                ),
                            },
                            Double(
                                100.0,
                            ),
                        ],
                    ),
                },
            ),
            mutable: false,
        },
        DeclStmt {
            target_type: Identifier(
                "dynamic",
            ),
            target_id: Identifier(
                "xy",
            ),
            expression: Some(
                BinaryOperation {
                    lhs: Identifier(
                        "vector3",
                    ),
                    op: Dot,
                    rhs: Identifier(
                        "xy",
                    ),
                },
            ),
            mutable: false,
        },
        BinaryOperation {
            lhs: Identifier(
                "assert_eq",
            ),
            op: OpenParenthesis,
            rhs: Tuple(
                [
                    Double(
                        0.0,
                    ),
                    BinaryOperation {
                        lhs: Identifier(
                            "xy",
                        ),
                        op: Dot,
                        rhs: Identifier(
                            "x",
                        ),
                    },
                    String(
                        "Vec3.xy.x failed to equal expected value",
                    ),
                ],
            ),
        },
        BinaryOperation {
            lhs: Identifier(
                "assert_eq",
            ),
            op: OpenParenthesis,
            rhs: Tuple(
                [
                    Double(
                        1250.05,
                    ),
                    BinaryOperation {
                        lhs: Identifier(
                            "xy",
                        ),
                        op: Dot,
                        rhs: Identifier(
                            "y",
                        ),
                    },
                    String(
                        "Vec3.xy.y failed to equal expected value",
                    ),
                ],
            ),
        },
        BinaryOperation {
            lhs: Identifier(
                "assert_eq",
            ),
            op: OpenParenthesis,
            rhs: Tuple(
                [
                    Double(
                        100.0,
                    ),
                    BinaryOperation {
                        lhs: Identifier(
                            "vector3",
                        ),
                        op: Dot,
                        rhs: Identifier(
                            "z",
                        ),
                    },
                    String(
                        "Vec3.z failed to equal expected value",
                    ),
                ],
            ),
        },
        AssignStmnt {
            id: Identifier(
                "status",
            ),
            expression: Bool(
                true,
            ),
        },
    ],
)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::test_main

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.32s

