extern crate inquerest;

use inquerest::*;


#[test]
fn test_filters(){
    assert_eq!(
        	Ok(
                Params {
                    filters: vec![
                        Filter {
                            connector: None,
                            condition: Condition {
                                left: Operand::Column("age".to_owned()),
                                equality: Equality::LT,
                                right: Operand::Number(13)
                            },
                            sub_filters: vec![
                                Filter {
                                    connector: Some(
                                        Connector::AND
                                    ),
                                    condition: Condition {
                                        left: Operand::Column("student".to_owned()),
                                        equality: Equality::EQ,
                                        right: Operand::Boolean(true)
                                    },
                                    sub_filters: vec![
                                        Filter {
                                            connector: Some(
                                                Connector::OR
                                            ),
                                            condition: Condition {
                                                left: Operand::Column("gender".to_owned()),
                                                equality: Equality::EQ,
                                                right: Operand::Column("M".to_owned())
                                            },
                                            sub_filters: vec![]
                                        }
                                    ]
                                },
                                
                            ]
                        }
                    ],
                    equations: vec![]
                }
            )
        
        , params("age=lt.13&student=eq.true|gender=eq.M"));
}





#[test]
fn test_filters_equations(){
    assert_eq!(
        	Ok(
                Params {
                    filters: vec![
                        Filter {
                            connector: None,
                            condition: Condition {
                                left: Operand::Column("age".to_owned()),
                                equality: Equality::LT,
                                right: Operand::Number(13)
                            },
                            sub_filters: vec![
                                Filter {
                                    connector: Some(
                                        Connector::AND
                                    ),
                                    condition: Condition {
                                        left: Operand::Column("student".to_owned()),
                                        equality: Equality::EQ,
                                        right: Operand::Boolean(true)
                                    },
                                    sub_filters: vec![
                                        Filter {
                                            connector: Some(
                                                Connector::OR
                                            ),
                                            condition: Condition {
                                                left: Operand::Column("gender".to_owned()),
                                                equality: Equality::EQ,
                                                right: Operand::Column("M".to_owned())
                                            },
                                            sub_filters: vec![]
                                        }
                                    ]
                                },
                                
                            ]
                        }
                    ],
                    equations: vec![Equation { left: Operand::Column("x".to_owned()), right: Operand::Number(123) }]
                }
            )
        
        , params("age=lt.13&student=eq.true|gender=eq.M&x=123"));
}






#[test]
fn test_filters_equations2(){
    assert_eq!(
        	Ok(
                Params {
                    filters: vec![
                        Filter {
                            connector: None,
                            condition: Condition {
                                left: Operand::Column("age".to_owned()),
                                equality: Equality::LT,
                                right: Operand::Number(13)
                            },
                            sub_filters: vec![
                                Filter {
                                    connector: Some(
                                        Connector::AND
                                    ),
                                    condition: Condition {
                                        left: Operand::Column("student".to_owned()),
                                        equality: Equality::EQ,
                                        right: Operand::Boolean(true)
                                    },
                                    sub_filters: vec![
                                        Filter {
                                            connector: Some(
                                                Connector::OR
                                            ),
                                            condition: Condition {
                                                left: Operand::Column("gender".to_owned()),
                                                equality: Equality::EQ,
                                                right: Operand::Column("M".to_owned())
                                            },
                                            sub_filters: vec![]
                                        }
                                    ]
                                },
                                
                            ]
                        }
                    ],
                    equations: vec![
                        Equation { left: Operand::Column("x".to_owned()), right: Operand::Number(123) },
                        Equation { left: Operand::Column("title".to_owned()), right: Operand::Column("engr".to_owned()) }
                        ]
                }
            )
        
        , params("age=lt.13&student=eq.true|gender=eq.M&x=123&title=engr"));
}






#[test]
fn test_filters2_equations2(){
    println!("{:#?}", params("age=lt.13&student=eq.true|gender=eq.M&age=lt.13&student=eq.true|gender=eq.M&x=123&title=engr"));
    assert_eq!(
        	Ok(
                Params {
                    filters: vec![
                        Filter {
                            connector: None,
                            condition: Condition {
                                left: Operand::Column("age".to_owned()),
                                equality: Equality::LT,
                                right: Operand::Number(13)
                            },
                            sub_filters: vec![
                                Filter {
                                    connector: Some(
                                        Connector::AND
                                    ),
                                    condition: Condition {
                                        left: Operand::Column("student".to_owned()),
                                        equality: Equality::EQ,
                                        right: Operand::Boolean(true)
                                    },
                                    sub_filters: vec![
                                        Filter {
                                            connector: Some(
                                                Connector::OR
                                            ),
                                            condition: Condition {
                                                left: Operand::Column("gender".to_owned()),
                                                equality: Equality::EQ,
                                                right: Operand::Column("M".to_owned())
                                            },
                                            sub_filters: vec![
                                                    Filter {
                                                        connector: Some(Connector::AND),
                                                        condition: Condition {
                                                            left: Operand::Column("age".to_owned()),
                                                            equality: Equality::LT,
                                                            right: Operand::Number(13)
                                                        },
                                                        sub_filters: vec![
                                                            Filter {
                                                                connector: Some(
                                                                    Connector::AND
                                                                ),
                                                                condition: Condition {
                                                                    left: Operand::Column("student".to_owned()),
                                                                    equality: Equality::EQ,
                                                                    right: Operand::Boolean(true)
                                                                },
                                                                sub_filters: vec![
                                                                    Filter {
                                                                        connector: Some(
                                                                            Connector::OR
                                                                        ),
                                                                        condition: Condition {
                                                                            left: Operand::Column("gender".to_owned()),
                                                                            equality: Equality::EQ,
                                                                            right: Operand::Column("M".to_owned())
                                                                        },
                                                                        sub_filters: vec![]
                                                                    }
                                                                ]
                                                            },
                                                            
                                                        ]
                                                    }
                                            ]
                                        }
                                    ]
                                },
                                
                            ]
                        },
                    ],
                    equations: vec![
                        Equation { left: Operand::Column("x".to_owned()), right: Operand::Number(123) },
                        Equation { left: Operand::Column("title".to_owned()), right: Operand::Column("engr".to_owned()) }
                        ]
                }
            )
        
        , params("age=lt.13&student=eq.true|gender=eq.M&age=lt.13&student=eq.true|gender=eq.M&x=123&title=engr"));
}









#[test]
fn test_equations_from_join_filter_groupby_having_orderby_limit_as_param(){
    println!("{:#?}", params("from=bazaar.person&left_join=person_student&on=student.id&age=lt.13&student=eq.true&gender=eq.M&group_by=sum(age)&order_by=desc&limit=100&offset=25&x=123&y=456"));
    assert_eq!(
        	Ok(
    Params {
        filters: vec![],
        equations: vec![
            Equation {
                left: Operand::Column(
                    "from".to_owned()
                ),
                right: Operand::Column(
                    "bazaar.person".to_owned()
                )
            },
            Equation {
                left: Operand::Column(
                    "left_join".to_owned()
                ),
                right: Operand::Column(
                    "person_student".to_owned()
                )
            },
            Equation {
                left: Operand::Column(
                    "on".to_owned()
                ),
                right: Operand::Column(
                    "student.id".to_owned()
                )
            },
            Equation {
                left: Operand::Column(
                    "age".to_owned()
                ),
                right: Operand::Column(
                    "lt.13".to_owned()
                )
            },
            Equation {
                left: Operand::Column(
                    "student".to_owned()
                ),
                right: Operand::Column(
                    "eq.true".to_owned()
                )
            },
            Equation {
                left: Operand::Column(
                    "gender".to_owned()
                ),
                right: Operand::Column(
                    "eq.M".to_owned()
                )
            },
            Equation {
                left: Operand::Column(
                    "group_by".to_owned()
                ),
                right: Operand::Function(
                    Function {
                        function: "sum".to_owned(),
                        params: vec![
                            Operand::Column(
                                "age".to_owned()
                            )
                        ]
                    }
                )
            },
            Equation {
                left: Operand::Column(
                    "order_by".to_owned()
                ),
                right: Operand::Column(
                    "desc".to_owned()
                )
            },
            Equation {
                left: Operand::Column(
                    "limit".to_owned()
                ),
                right: Operand::Number(
                    100
                )
            },
            Equation {
                left: Operand::Column(
                    "offset".to_owned()
                ),
                right: Operand::Number(
                    25
                )
            },
            Equation {
                left: Operand::Column(
                    "x".to_owned()
                ),
                right: Operand::Number(
                    123
                )
            },
            Equation {
                left: Operand::Column(
                    "y".to_owned()
                ),
                right: Operand::Number(
                    456
                )
            }
        ]
    }
)

        
        , params(
"from=bazaar.person&left_join=person_student&on=student.id&age=lt.13&student=eq.true&gender=eq.M&group_by=sum(age)&order_by=desc&limit=100&offset=25&x=123&y=456"));
}







#[test]
fn test_equations_from_join_filter_groupby_having_orderby_limit_as_param2(){
    println!("{:#?}", params(
"age=lt.13&student=eq.true|gender=eq.M&group_by=sum(age)&order_by=desc&limit=100&offset=25&x=123&y=456&from=bazaar.person&left_join=person_student&on=student.id"));

    assert_eq!(
        Ok(
Params {
        filters: vec![
            Filter {
                connector: None,
                condition: Condition {
                    left: Operand::Column(
                        "age".to_owned()
                    ),
                    equality: Equality::LT,
                    right: Operand::Number(
                        13
                    )
                },
                sub_filters: vec![
                    Filter {
                        connector: Some(
                            Connector::AND
                        ),
                        condition: Condition {
                            left: Operand::Column(
                                "student".to_owned()
                            ),
                            equality: Equality::EQ,
                            right: Operand::Boolean(
                                true
                            )
                        },
                        sub_filters: vec![
                            Filter {
                                connector: Some(
                                    Connector::OR
                                ),
                                condition: Condition {
                                    left: Operand::Column(
                                        "gender".to_owned()
                                    ),
                                    equality: Equality::EQ,
                                    right: Operand::Column(
                                        "M".to_owned()
                                    )
                                },
                                sub_filters: vec![]
                            }
                        ]
                    }
                ]
            }
        ],
        equations: vec![
            Equation {
                left: Operand::Column(
                    "group_by".to_owned()
                ),
                right: Operand::Function(
                    Function {
                        function: "sum".to_owned(),
                        params: vec![
                            Operand::Column(
                                "age".to_owned()
                            )
                        ]
                    }
                )
            },
            Equation {
                left: Operand::Column(
                    "order_by".to_owned()
                ),
                right: Operand::Column(
                    "desc".to_owned()
                )
            },
            Equation {
                left: Operand::Column(
                    "limit".to_owned()
                ),
                right: Operand::Number(
                    100
                )
            },
            Equation {
                left: Operand::Column(
                    "offset".to_owned()
                ),
                right: Operand::Number(
                    25
                )
            },
            Equation {
                left: Operand::Column(
                    "x".to_owned()
                ),
                right: Operand::Number(
                    123
                )
            },
            Equation {
                left: Operand::Column(
                    "y".to_owned()
                ),
                right: Operand::Number(
                    456
                )
            },
            Equation {
                left: Operand::Column(
                    "from".to_owned()
                ),
                right: Operand::Column(
                    "bazaar.person".to_owned()
                )
            },
            Equation {
                left: Operand::Column(
                    "left_join".to_owned()
                ),
                right: Operand::Column(
                    "person_student".to_owned()
                )
            },
            Equation {
                left: Operand::Column(
                    "on".to_owned()
                ),
                right: Operand::Column(
                    "student.id".to_owned()
                )
            }
        ]
    }
)
        
        , params(
"age=lt.13&student=eq.true|gender=eq.M&group_by=sum(age)&order_by=desc&limit=100&offset=25&x=123&y=456&from=bazaar.person&left_join=person_student&on=student.id"));
}


