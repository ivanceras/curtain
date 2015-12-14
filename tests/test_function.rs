extern crate inquerest;

use inquerest::*;

#[test]
fn test_min(){
    assert_eq!(
        Ok(Function{
            function: "min".to_owned(),
            params: vec![Operand::Column("age".to_owned())], 
        }),
        function("min(age)"));
}

#[test]
fn test_max(){
    assert_eq!(
        Ok(Function{
            function: "max".to_owned(),
            params: vec![Operand::Column("age".to_owned())], 
        }),
        function("max(age)"));
}

#[test]
fn test_sum(){
    assert_eq!(
        Ok(Function{
            function: "sum".to_owned(),
            params: vec![Operand::Column("age".to_owned())], 
        }),
        function("sum(age)"));
}


#[test]
fn test_max_sum(){
    assert_eq!(
        Ok(
            Function{
                function:"max".to_owned(),
                params: vec![Operand::Function(Function{
                                function: "sum".to_owned(),
                                params: vec![Operand::Column("age".to_owned())], 
                            })]
            }
        ),
        function("max(sum(age))"));
}
