extern crate inquerest;

use inquerest::*;


#[test]
fn test_column(){
    assert_eq!(
        Ok(Operand::Column("age".to_owned())),
        operand("age"));
}

#[test]
fn test_number(){
    assert_eq!(
        Ok(123),
        number("123"));
}


#[test]
fn test_table_column(){
    assert_eq!(
        Ok(Operand::Column("person.age".to_owned())),
        operand("person.age"));
}


#[test]
fn test_function(){
    assert_eq!(
        Ok(Operand::Function(Function{
            function: "max".to_owned(),
            params: vec![Operand::Column("age".to_owned())], 
        })),
        operand("max(age)"));
}

