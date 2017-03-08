extern crate inquerest as iq;
extern crate rustorm;

use rustorm::query::{Select, Join, Filter, Condition, Connector, Equality, Operand, Modifier,
                     JoinType, ColumnName, Function, Direction, Range, NullsWhere, Order};
use rustorm::query::TableName;
use rustorm::dao::Value;
use rustorm::query::source::QuerySource;
use validator::DbElementValidator;

/// convert inquery to sql query

pub trait FromQuery {
    fn transform(&self, validator: &DbElementValidator) -> Select;
}


impl FromQuery for iq::Select {
    /// you would still want to check for validity and permission first
    /// before even blindly transforming the query
    fn transform(&self, validator: &DbElementValidator) -> Select {
        let mut q = Select::new();

        for ref fr in &self.from {
            match fr {
                &&iq::Operand::Column(ref column) => {
                    if validator.is_valid_table(column) {
                        let table = QuerySource::TableName(TableName::from_str(column));
                        q.from(&table);
                    }
                }
                _ => unimplemented!(),
            };
        }
        for j in &self.join {
            q.joins.push(j.transform(validator));
        }
        for f in &self.filters {
            q.add_filter(&f.transform(validator));
        }
        for g in &self.group_by {
            q.group_by.push(g.smart_transform(validator));
        }
        for h in &self.having {
            q.having.push(h.transform(validator));
        }
        for o in &self.order_by {
            q.order_by.push(o.transform(validator));
        }
        match &self.range {
            &Some(ref range) => q.range = range.transform(),
            &None => {}
        };
        q
    }
}

pub trait FromOrder {
    fn transform(&self, validator: &DbElementValidator) -> Order;
}

impl FromOrder for iq::Order {
    fn transform(&self, validator: &DbElementValidator) -> Order {
        let operand = self.operand.smart_transform(validator);
        let direction = match &self.direction {
            &Some(ref direction) => {
                match direction {
                    &iq::Direction::ASC => Some(Direction::ASC),
                    &iq::Direction::DESC => Some(Direction::DESC),
                }
            }
            &None => None,
        };
        let nulls_where = match &self.nulls_where {
            &Some(ref nulls_where) => {
                match nulls_where {
                    &iq::NullsWhere::FIRST => Some(NullsWhere::FIRST),
                    &iq::NullsWhere::LAST => Some(NullsWhere::LAST),
                }
            }
            &None => None,
        };
        Order {
            operand: operand,
            direction: direction,
            nulls_where: nulls_where,
        }
    }
}

pub trait FromRange {
    fn transform(&self) -> Range;
}

impl FromRange for iq::Range {
    fn transform(&self) -> Range {
        match *self {
            iq::Range::Page(ref page) => {
                let limit = page.page_size;
                let offset = page.page * page.page_size;
                Range {
                    limit: Some(limit as usize),
                    offset: Some(offset as usize),
                }
            }
            iq::Range::Limit(ref limit) => {
                let offset = match limit.offset {
                    Some(offset) => Some(offset as usize),
                    None => None,
                };
                Range {
                    limit: Some(limit.limit as usize),
                    offset: offset,
                }
            }
        }
    }
}

pub trait FromConnector {
    fn transform(&self) -> Connector;
}

impl FromConnector for iq::Connector {
    fn transform(&self) -> Connector {
        match *self {
            iq::Connector::AND => Connector::And,
            iq::Connector::OR => Connector::Or,
        }
    }
}

pub trait FromFilter {
    fn transform(&self, validator: &DbElementValidator) -> Filter;
}


impl FromFilter for iq::Filter {
    fn transform(&self, validator: &DbElementValidator) -> Filter {
        let mut sub_filters = vec![];
        for f in &self.sub_filters {
            sub_filters.push(f.transform(validator));
        }
        Filter {
            connector: match &self.connector {
                &Some(ref conn) => conn.transform(),
                &None => Connector::And,
            },
            condition: self.condition.transform(validator),
            sub_filters: sub_filters,
        }
    }
}

pub trait FromCondition {
    fn transform(&self, validator: &DbElementValidator) -> Condition;
}

impl FromCondition for iq::Condition {
    fn transform(&self, validator: &DbElementValidator) -> Condition {
        Condition {
            left: self.left.smart_transform(validator),
            equality: self.equality.transform(),
            right: self.right.smart_transform(validator),
        }
    }
}

pub trait FromEquality {
    fn transform(&self) -> Equality;
}

impl FromEquality for iq::Equality {
    fn transform(&self) -> Equality {
        match *self {
            iq::Equality::EQ => Equality::EQ,
            iq::Equality::NEQ => Equality::NEQ,
            iq::Equality::LT => Equality::LT,
            iq::Equality::LTE => Equality::LTE,
            iq::Equality::GT => Equality::GT,
            iq::Equality::GTE => Equality::GTE,
            iq::Equality::IN => Equality::IN,
            iq::Equality::NOT_IN => Equality::NOT_IN,
            iq::Equality::LIKE => Equality::LIKE,
            iq::Equality::ILIKE => Equality::ILIKE, //
            iq::Equality::IS => Equality::IS_NULL, //[FIXME]
            iq::Equality::IS_NOT => Equality::IS_NOT_NULL, //[FIXME]
            iq::Equality::ST => Equality::LIKE
        }
    }
}

pub trait FromOperand {
    fn smart_transform(&self, validator: &DbElementValidator) -> Operand;
}


impl FromOperand for iq::Operand {
    /// using table, columns, function validator, a decision on how
    /// to trait each values accordingly
    fn smart_transform(&self, validator: &DbElementValidator) -> Operand {
        match &self {
            &&iq::Operand::Column(ref column) => {
                if validator.is_valid_column(column) {
                    Operand::ColumnName(ColumnName::from_str(column))
                } else {
                    Operand::Value(Value::String(column.to_owned()))
                }
            }
            &&iq::Operand::Value(ref value) => Operand::Value(Value::String(value.to_owned())),
            &&iq::Operand::Number(number) => Operand::Value(Value::F64(number as f64)),
            &&iq::Operand::Boolean(boolean) => Operand::Value(Value::Bool(boolean)),
            &&iq::Operand::Function(ref function) => {
                if validator.function_validator.is_valid_function_name(&function.function) {
                    Operand::QuerySource(QuerySource::Function(function.transform(validator)))
                } else {
                    panic!("Function is invalid");
                }
            }
        }
    }
}

pub trait FromFunction {
    fn transform(&self, validator: &DbElementValidator) -> Function;
}
impl FromFunction for iq::Function {
    fn transform(&self, validator: &DbElementValidator) -> Function {
        let mut params = vec![];
        for p in &self.params {
            params.push(p.smart_transform(validator))
        }
        Function {
            function: self.function.to_owned(),
            params: params,
        }
    }
}


pub trait FromJoin {
    fn transform(&self, validator: &DbElementValidator) -> Join;
}

impl FromJoin for iq::Join {
    fn transform(&self, validator: &DbElementValidator) -> Join {

        assert_eq!(self.column1.len(), self.column2.len());
        assert!(self.column1.len() > 0);
        let left0 = Operand::ColumnName(ColumnName::from_str(&self.column1[0]));
        let right0 = Operand::ColumnName(ColumnName::from_str(&self.column2[0]));
        let cond0 = Condition {
            left: left0,
            equality: Equality::EQ,
            right: right0,
        };

        let mut sub_filters = vec![];
        for i in 1..self.column1.len() {
            let left = Operand::ColumnName(ColumnName::from_str(&self.column1[i]));
            let right = Operand::ColumnName(ColumnName::from_str(&self.column2[i]));
            let filter = Filter {
                connector: Connector::And,
                condition: Condition {
                    left: left,
                    equality: Equality::EQ,
                    right: right,
                },
                sub_filters: vec![],
            };
            sub_filters.push(filter);
        }

        Join {
            modifier: match &self.modifier {
                &Some(ref modifier) => Some(modifier.transform()),
                &None => None,
            },
            join_type: match &self.join_type {
                &Some(ref join_type) => Some(join_type.transform()),
                &None => None,
            },
            table_name: match &self.table {
                &iq::Operand::Column(ref column) => TableName::from_str(column),
                _ => unimplemented!(), 
            },
            on: Filter {
                condition: cond0,
                sub_filters: sub_filters,
                connector: Connector::And,
            },
        }
    }
}


pub trait FromModifier {
    fn transform(&self) -> Modifier;
}

impl FromModifier for iq::Modifier {
    fn transform(&self) -> Modifier {
        match &self {
            &&iq::Modifier::LEFT => Modifier::LEFT,
            &&iq::Modifier::RIGHT => Modifier::RIGHT,
            &&iq::Modifier::FULL => Modifier::FULL,
        }
    }
}


pub trait FromJoinType {
    fn transform(&self) -> JoinType;
}


impl FromJoinType for iq::JoinType {
    fn transform(&self) -> JoinType {
        match &self {
            &&iq::JoinType::CROSS => JoinType::CROSS,
            &&iq::JoinType::INNER => JoinType::INNER,
            &&iq::JoinType::OUTER => JoinType::OUTER,
            &&iq::JoinType::NATURAL => JoinType::NATURAL,
        }
    }
}
