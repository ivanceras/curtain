extern crate inquerest;

use inquerest::*;

#[test]
fn test_desc(){
    assert_eq!(
        Ok(Order{
            column: "age".to_owned(), 
            direction: Direction::DESC, 
        }),
        order("age.desc"));
}


#[test]
fn test_asc(){
    assert_eq!(
        Ok(Order{
            column: "age".to_owned(), 
            direction: Direction::ASC, 
        }),
        order("age.asc"));
}


#[test]
fn test_table_column_order(){
    assert_eq!(
        Ok(Order{
            column: "person.age".to_owned(), 
            direction: Direction::DESC, 
        }),
        order("person.age.desc"));
}


#[test]
fn test_order_by(){
    assert_eq!(
        Ok(vec![Order{
            column: "age".to_owned(), 
            direction: Direction::ASC, 
        }]),
        order_by("order_by=age.asc"));
}

#[test]
fn test_order_by2(){
    assert_eq!(
        Ok(vec![Order{
                column: "age".to_owned(), 
                direction: Direction::ASC, 
            },
            Order{
                column: "grade".to_owned(), 
                direction: Direction::DESC, 
            }
            ]),
        order_by("order_by=age.asc,grade.desc"));
}


#[test]
fn test_order_by3(){
    assert_eq!(
        Ok(vec![Order{
                column: "age".to_owned(), 
                direction: Direction::DESC, 
            },
            Order{
                column: "grade".to_owned(), 
                direction: Direction::DESC, 
            },
            Order{
                column: "height".to_owned(), 
                direction: Direction::ASC, 
            }
            ]),
        order_by("order_by=age.desc,grade.desc,height.asc"));
}


#[test]
fn test_order_by4(){
    assert_eq!(
        Ok(vec![Order{
                column: "person.age".to_owned(), 
                direction: Direction::DESC, 
            },
            Order{
                column: "student.grade".to_owned(), 
                direction: Direction::DESC, 
            },
            Order{
                column: "height".to_owned(), 
                direction: Direction::ASC, 
            }
            ]),
        order_by("order_by=person.age.desc,student.grade.desc,height.asc"));
}