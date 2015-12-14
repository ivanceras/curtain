extern crate inquerest;

use inquerest::*;

#[test]
fn test_group(){
    assert_eq!(
        Ok(vec![Operand::Column("age".to_owned())]),
        group_by("group_by=age"));
}

#[test]
fn test_group2(){
    assert_eq!(
        Ok(vec![
            Operand::Column("age".to_owned()),
            Operand::Column("grade".to_owned()),
            ]),
        group_by("group_by=age,grade"));
}

#[test]
fn test_group3(){
    assert_eq!(
        Ok(vec![
            Operand::Column("age".to_owned()),
            Operand::Column("grade".to_owned()),
            Operand::Column("gender".to_owned()),
            ]),
        group_by("group_by=age,grade,gender"));
}

#[test]
fn test_group_sum(){
    assert_eq!(
        Ok(vec![
            Operand::Function(
                        Function{
                                function: "sum".to_owned(),
                                params: vec![Operand::Column("age".to_owned())]
                            }
                    ),
            Operand::Column("grade".to_owned()),
            Operand::Column("gender".to_owned()),
            ]),
        group_by("group_by=sum(age),grade,gender"));
}
