extern crate inquerest as iq;
extern crate rustorm;

use rustorm::query::{Query, Join, Filter, Condition, Connector, 
    Equality, Operand, ToTableName, Modifier, JoinType,
    ColumnName, Function, Direction
    };

use rustorm::dao::Value;

/// convert inquery to sql query

pub trait FromQuery{
    fn transform(&self) -> Query;

}


impl FromQuery for iq::Query {
    
    /// you would still want to check for validity and permission first
    /// before even blindly transforming the query
    fn transform(&self) -> Query{
        let mut q = Query::new();
        
        for ref fr in &self.from{
            let table = match fr{
                &&iq::Operand::Column(ref column) => {
                    column
                }
                _ => { unimplemented!()}
            };
            q.from(table);
        }
        for j in &self.join{
            println!("join: {:#?}", j);
            println!("to_join: {:#?}", j.transform());
            q.join(j.transform());
        }
        for f in &self.filters{
            println!("filter: {:?}", f);
            q.add_filter(f.transform());
        }
        for g in &self.group_by{
            println!("group_by: {:?}", g);
            q.group_by.push(g.transform());
        }
        for h in &self.having{
            println!("having: {:?}", h);
            q.having.push(h.transform());
        }
        for o in &self.order_by{
            println!("order_by: {:?}", o);
            q.order_by.push(o.transform());
        }
        match &self.range{
            &Some(ref range) => {
                match range{
                    &iq::Range::Page(ref page) => {
                        q.set_page(page.page as usize);
                        q.set_page_size(page.page_size as usize);
                    },
                    &iq::Range::Limit(_) => {
                        unimplemented!();
                    }
                }
            },
            &None => {}
        };
        q
    }
    
}

pub trait FromOrder{
    fn transform(&self)->(String, Direction);
}

impl FromOrder for iq::Order{
    fn transform(&self)->(String, Direction){
        let direction = match self.direction{
            iq::Direction::ASC => Direction::ASC,
            iq::Direction::DESC => Direction::DESC,
        };
        (self.column.to_owned(), direction)
    }
}

pub trait FromConnector{
    fn transform(&self) -> Connector;
}

impl FromConnector for iq::Connector{
    
    fn transform(&self) -> Connector{
        match *self{
            iq::Connector::AND => Connector::And,
            iq::Connector::OR => Connector::Or,
        }
    }
}

pub trait FromFilter{
    fn transform(&self) -> Filter;
}


impl FromFilter for iq::Filter{
    
    fn transform(&self) -> Filter{
        let mut sub_filters = vec![];
        for f in &self.sub_filters{
            sub_filters.push(f.transform());
        }
        Filter{
            connector: match &self.connector{
                &Some(ref conn) => conn.transform(),
                &None => Connector::And
            },
            condition: self.condition.transform(),
            sub_filters: sub_filters,
        }
    }

}

pub trait FromCondition{
    fn transform(&self) -> Condition;
}

impl FromCondition for iq::Condition{
    
    fn transform(&self) -> Condition {
        Condition {
            left: self.left.transform(),
            equality: self.equality.transform(),
            right: self.right.transform(),
        }
    }
}

pub trait FromEquality{
    fn transform(&self)->Equality;
}

impl FromEquality for iq::Equality{
    
    fn transform(&self)->Equality{
        match *self{
            iq::Equality::EQ => Equality::EQ,
            iq::Equality::NEQ => Equality::NEQ,
            iq::Equality::LT => Equality::LT,
            iq::Equality::LTE => Equality::LTE,
            iq::Equality::GT => Equality::GT,
            iq::Equality::GTE => Equality::GTE,
            iq::Equality::IN => Equality::IN,
            iq::Equality::NOT_IN => Equality::NOT_IN,
            iq::Equality::LIKE => Equality::LIKE,
            iq::Equality::ILIKE => Equality::LIKE, //[FIXME]
            iq::Equality::IS => Equality::IS_NULL, //[FIXME]
            iq::Equality::IS_NOT => Equality::IS_NOT_NULL, //[FIXME]
        }
    }
}

pub trait FromOperand{
    fn transform(&self) -> Operand;
}

impl FromOperand for iq::Operand{
    
    fn transform(&self) -> Operand{
        match &self{
            &&iq::Operand::Column(ref column) => {
                Operand::ColumnName(ColumnName::from_str(column))
            },
            &&iq::Operand::Number(number) => {
                Operand::Value(Value::F64(number as f64))
            },
            &&iq::Operand::Boolean(boolean) => {
                Operand::Value(Value::Bool(boolean))
            },
            &&iq::Operand::Function(ref function) => {
                Operand::Function(function.transform())
            },
        }
    }
}

pub trait FromFunction{
    fn transform(&self) -> Function;
}
impl FromFunction for iq::Function{
    fn transform(&self) -> Function{
        let mut params = vec![];
        for p in &self.params{
            params.push(p.transform())
        }
        Function{
            function: self.function.to_owned(),
            params: params,
        }
    }
}


pub trait FromJoin{
    
    fn transform(&self) -> Join;
}

impl FromJoin for iq::Join{
    
    fn transform(&self) -> Join{
        
        Join{
            modifier: match &self.modifier{
                        &Some(ref modifier) => Some(modifier.transform()),
                        &None => None,
                    },
            join_type: match &self.join_type{
                        &Some(ref join_type) => Some(join_type.transform()),
                        &None => None
                    },
            table_name: match &self.table{
                    &iq::Operand::Column(ref column) => column.to_table_name(),
                    _ => unimplemented!() 
                },
            column1: self.column1.clone(),
            column2: self.column2.clone()
        }
    }
}


pub trait FromModifier{
    
    fn transform(&self) -> Modifier;
}

impl FromModifier for iq::Modifier{
    
    fn transform(&self) -> Modifier{
        match &self{
            &&iq::Modifier::LEFT => Modifier::LEFT,
            &&iq::Modifier::RIGHT => Modifier::RIGHT,
            &&iq::Modifier::FULL => Modifier::FULL,
        }
    }
}


pub trait FromJoinType{
    
    fn transform(&self) -> JoinType;
}


impl FromJoinType for iq::JoinType{
    
    fn transform(&self) -> JoinType{
        match &self{
            &&iq::JoinType::CROSS => JoinType::CROSS,
            &&iq::JoinType::INNER => JoinType::INNER,
            &&iq::JoinType::OUTER => JoinType::OUTER,
            &&iq::JoinType::NATURAL => JoinType::NATURAL,
        }
    }
}



