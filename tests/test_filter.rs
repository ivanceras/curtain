extern crate inquerest;

use inquerest::*;

#[test]
fn test_simple(){
    assert_eq!(
        Ok(Filter{
                connector: None,
                condition: Condition{left:Operand::Column("student".to_owned()),
                                    equality:Equality::EQ,
                                    right: Operand::Boolean(true)
                            },
                sub_filters: vec![]
                }),
        filter("student=eq.true"))
}

#[test]
fn test_enclosed(){
    assert_eq!(
        Ok(Filter{
                connector: None,
                condition: Condition{left:Operand::Column("student".to_owned()),
                                    equality:Equality::EQ,
                                    right: Operand::Boolean(true)
                            },
                sub_filters: vec![]
                }),
        filter("(student=eq.true)"))
}
#[test]
fn and_filter(){
    assert_eq!(
       Ok(
           Filter { 
               connector: None, 
               condition: Condition { 
                       left: Operand::Column("student".to_owned()), 
                       equality: Equality::EQ, 
                       right: Operand::Boolean(true) }, 
                       sub_filters: vec![
                               Filter { 
                                   connector: Some(Connector::AND), 
                                   condition: Condition { 
                                               left: Operand::Column("age".to_owned()), 
                                               equality: Equality::LT, 
                                               right: Operand::Number(13) 
                                           }, 
                                   sub_filters: vec![] 
                               }
                           ] 
           }
       ),
        filter("student=eq.true&age=lt.13"))
}

#[test]
fn enclosed_and_filter(){
    assert_eq!(
       Ok(
           Filter { 
               connector: None, 
               condition: Condition { 
                       left: Operand::Column("student".to_owned()), 
                       equality: Equality::EQ, 
                       right: Operand::Boolean(true) }, 
                       sub_filters: vec![
                               Filter { 
                                   connector: Some(Connector::AND), 
                                   condition: Condition { 
                                               left: Operand::Column("age".to_owned()), 
                                               equality: Equality::LT, 
                                               right: Operand::Number(13) 
                                           }, 
                                   sub_filters: vec![] 
                               }
                           ] 
           }
       ),
        filter("(student=eq.true&age=lt.13)"))
}

#[test]
fn enclosed_or_filter(){
    assert_eq!(
       Ok(
           Filter { 
               connector: None, 
               condition: Condition { 
                       left: Operand::Column("student".to_owned()), 
                       equality: Equality::EQ, 
                       right: Operand::Boolean(true) }, 
                       sub_filters: vec![
                               Filter { 
                                   connector: Some(Connector::OR), 
                                   condition: Condition { 
                                               left: Operand::Column("age".to_owned()), 
                                               equality: Equality::LT, 
                                               right: Operand::Number(13) 
                                           }, 
                                   sub_filters: vec![] 
                               }
                           ] 
           }
       ),
        filter("(student=eq.true|age=lt.13)"))
}

#[test]
#[should_panic] // FIXME: should not panic!
fn enclosed_or_filter2(){
    assert_eq!(
       Ok(
           Filter { 
               connector: None, 
               condition: Condition { 
                       left: Operand::Column("student".to_owned()), 
                       equality: Equality::EQ, 
                       right: Operand::Boolean(true) }, 
                       sub_filters: vec![
                               Filter { 
                                   connector: Some(Connector::OR), 
                                   condition: Condition { 
                                               left: Operand::Column("age".to_owned()), 
                                               equality: Equality::LT, 
                                               right: Operand::Number(13)
                                           }, 
                                   sub_filters: vec![] 
                               }
                           ] 
           }
       ),
        filter("(student=eq.true|age=lt.13)|grade=lt.3"))
}

#[test]
fn and_and_filter(){
    println!("{:#?}", filter("student=eq.true&age=lt.13&grade=gte.3"));
    assert_eq!(
       Ok(
           Filter { 
               connector: None, 
               condition: Condition { 
                       left: Operand::Column("student".to_owned()), 
                       equality: Equality::EQ, 
                       right: Operand::Boolean(true) }, 
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
                                       connector: Some(Connector::AND), 
                                       condition: Condition { 
                                               left:  Operand::Column("grade".to_owned()), 
                                               equality: Equality::GTE, 
                                               right:  Operand::Number(3)
                                           }, 
                                       sub_filters: vec![] 
                                       }
                                   ] 
                               },
                              
                           ] 
           }
       ),
        filter("student=eq.true&age=lt.13&grade=gte.3"))
}

#[test]
fn or_filter(){
    assert_eq!(
       Ok(
           Filter { 
               connector: None, 
               condition: Condition { 
                       left: Operand::Column("student".to_owned()), 
                       equality: Equality::EQ, 
                       right: Operand::Boolean(true) }, 
                       sub_filters: vec![
                               Filter { 
                                   connector: Some(Connector::OR), 
                                   condition: Condition { 
                                               left: Operand::Column("age".to_owned()), 
                                               equality: Equality::LT, 
                                               right: Operand::Number(13) 
                                           }, 
                                   sub_filters: vec![] 
                               }
                           ] 
           }
       ),
        filter("student=eq.true|age=lt.13"))
}

#[test]
fn function_filter(){
    assert_eq!(
       Ok(
           Filter { 
               connector: None, 
               condition: Condition { 
                       left: Operand::Column("student".to_owned()), 
                       equality: Equality::EQ, 
                       right: Operand::Boolean(true) }, 
                       sub_filters: vec![
                               Filter { 
                                   connector: Some(Connector::OR), 
                                   condition: Condition { 
                                               left: Operand::Function(Function{
                                                               function: "max".to_owned(),
                                                               params: vec![Operand::Column("age".to_owned())]
                                                           }),
                                               equality: Equality::LT, 
                                               right: Operand::Number(13) 
                                           }, 
                                   sub_filters: vec![] 
                               }
                           ] 
           }
       ),
        filter("student=eq.true|max(age)=lt.13"))
}


#[test]
fn recursive_function_filter(){
    assert_eq!(
       Ok(
           Filter { 
               connector: None, 
               condition: Condition { 
                       left: Operand::Column("student".to_owned()), 
                       equality: Equality::EQ, 
                       right: Operand::Boolean(true) }, 
                       sub_filters: vec![
                               Filter { 
                                   connector: Some(Connector::OR), 
                                   condition: Condition { 
                                               left: Operand::Function(Function{
                                                               function: "max".to_owned(),
                                                               params: vec![Operand::Function(Function{
                                                                                        function: "sum".to_owned(),
                                                                                        params: vec![Operand::Column("age".to_owned())], 
                                                                                    })]
                                                           }),
                                               equality: Equality::LT, 
                                               right: Operand::Number(13) 
                                           }, 
                                   sub_filters: vec![] 
                               }
                           ] 
           }
       ),
        filter("student=eq.true|max(sum(age))=lt.13"))
}

#[test]
fn or_or_filter(){
    println!("{:#?}",filter("student=eq.true|age=lt.13|grade=gte.3"));
    assert_eq!(
       Ok(
           Filter { 
               connector: None, 
               condition: Condition { 
                       left: Operand::Column("student".to_owned()), 
                       equality: Equality::EQ, 
                       right: Operand::Boolean(true) }, 
                       sub_filters: vec![
                               Filter { 
                                   connector: Some(Connector::OR), 
                                   condition: Condition { 
                                               left: Operand::Column("age".to_owned()), 
                                               equality: Equality::LT, 
                                               right: Operand::Number(13) 
                                           }, 
                                   sub_filters: vec![
                                       Filter { 
                                           connector: Some(Connector::OR), 
                                           condition: Condition { 
                                                   left:  Operand::Column("grade".to_owned()), 
                                                   equality: Equality::GTE, 
                                                   right:  Operand::Number(3)
                                               }, 
                                           sub_filters: vec![] 
                                       }
                                   ] 
                               },
                               
                           ] 
           }
       ),
        filter("student=eq.true|age=lt.13|grade=gte.3"))
}


#[test]
fn and_or_filter(){
    println!("{:#?}",filter("student=eq.true&age=lt.13|grade=gte.3"));
    assert_eq!(
       Ok(
           Filter { 
               connector: None, 
               condition: Condition { 
                       left: Operand::Column("student".to_owned()), 
                       equality: Equality::EQ, 
                       right: Operand::Boolean(true) }, 
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
                                           connector: Some(Connector::OR), 
                                           condition: Condition { 
                                                   left:  Operand::Column("grade".to_owned()), 
                                                   equality: Equality::GTE, 
                                                   right:  Operand::Number(3)
                                               }, 
                                           sub_filters: vec![] 
                                       }
                                   ] 
                               },
                               
                           ] 
           }
       ),
        filter("student=eq.true&age=lt.13|grade=gte.3"))
}


#[test]
fn or_and_filter(){
    assert_eq!(
       Ok(
           Filter { 
               connector: None, 
               condition: Condition { 
                       left: Operand::Column("student".to_owned()), 
                       equality: Equality::EQ, 
                       right: Operand::Boolean(true) }, 
                       sub_filters: vec![
                               Filter { 
                                   connector: Some(Connector::OR), 
                                   condition: Condition { 
                                               left: Operand::Column("age".to_owned()), 
                                               equality: Equality::LT, 
                                               right: Operand::Number(13) 
                                           }, 
                                   sub_filters: vec![
                                           Filter { 
                                               connector: Some(Connector::AND), 
                                               condition: Condition { 
                                                       left:  Operand::Column("grade".to_owned()), 
                                                       equality: Equality::GTE, 
                                                       right:  Operand::Number(3)
                                                   }, 
                                               sub_filters: vec![] 
                                           }
                                   ] 
                               },
                               
                           ] 
           }
       ),
        filter("student=eq.true|age=lt.13&grade=gte.3"))
}


#[test]
fn enclosed_or_and_filter(){
    assert_eq!(
       Ok(
           Filter { 
               connector: None, 
               condition: Condition { 
                       left: Operand::Column("student".to_owned()), 
                       equality: Equality::EQ, 
                       right: Operand::Boolean(true) }, 
                       sub_filters: vec![
                               Filter { 
                                   connector: Some(Connector::OR), 
                                   condition: Condition { 
                                               left: Operand::Column("age".to_owned()), 
                                               equality: Equality::LT, 
                                               right: Operand::Number(13) 
                                           }, 
                                   sub_filters: vec![
                                           Filter { 
                                               connector: Some(Connector::AND), 
                                               condition: Condition { 
                                                       left:  Operand::Column("grade".to_owned()), 
                                                       equality: Equality::GTE, 
                                                       right:  Operand::Number(3)
                                                   }, 
                                               sub_filters: vec![] 
                                           }
                                   ] 
                               },
                               
                           ] 
           }
       ),
        filter("student=eq.true|(age=lt.13&grade=gte.3)"))
}


#[test]
fn enclosed_or_and_filter2(){
    assert_eq!(
       Ok(
           Filter { 
               connector: None, 
               condition: Condition { 
                       left: Operand::Column("student".to_owned()), 
                       equality: Equality::EQ, 
                       right: Operand::Boolean(true) }, 
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
                                               connector: Some(Connector::OR), 
                                               condition: Condition { 
                                                       left:  Operand::Column("grade".to_owned()), 
                                                       equality: Equality::GTE, 
                                                       right:  Operand::Number(3)
                                                   }, 
                                               sub_filters: vec![] 
                                           }
                                   ] 
                               },
                               
                           ] 
           }
       ),
        filter("student=eq.true&(age=lt.13|grade=gte.3)"))
}

#[test]
fn or_and_filter_with_function(){
    assert_eq!(
       Ok(
           Filter { 
               connector: None, 
               condition: Condition { 
                       left: Operand::Column("student".to_owned()), 
                       equality: Equality::EQ, 
                       right: Operand::Boolean(true) }, 
                       sub_filters: vec![
                               Filter { 
                                   connector: Some(Connector::OR), 
                                   condition: Condition { 
                                               left: Operand::Column("age".to_owned()), 
                                               equality: Equality::LT, 
                                               right: Operand::Number(13) 
                                           }, 
                                   sub_filters: vec![
                                       Filter { 
                                               connector: Some(Connector::AND), 
                                               condition: Condition { 
                                                       left:  Operand::Function(Function{
                                                                   function:"min".to_owned(),
                                                                   params: vec![Operand::Column("grade".to_owned())]
                                                               }), 
                                                       equality: Equality::GTE, 
                                                       right:  Operand::Number(3)
                                                   }, 
                                               sub_filters: vec![] 
                                           }
                                   ] 
                               },
                               
                           ] 
           }
       ),
        filter("student=eq.true|age=lt.13&min(grade)=gte.3"))
}

#[test]
fn equal_filter(){
    assert_eq!(filter("student=eq.true"),filter("((student=eq.true))"))
}

#[test]
fn equal_and_and(){
    assert_eq!(filter("student=eq.true&(age=lt.13&grade=gte.3)"),filter("student=eq.true&age=lt.13&grade=gte.3"))
}

#[test]
fn equal_or_or(){
    assert_eq!(filter("student=eq.true|(age=lt.13|grade=gte.3)"),filter("student=eq.true|age=lt.13|grade=gte.3"))
}


#[test]
fn equal_enclosed_or(){
    assert_eq!(filter("(student=eq.true|age=lt.13)"),filter("student=eq.true|age=lt.13"))
}

#[test]
fn equal_enclosed_and(){
    assert_eq!(filter("(student=eq.true&age=lt.13)"),filter("student=eq.true&age=lt.13"))
}


