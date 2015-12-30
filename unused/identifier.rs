use window::Tab;
use std::collections::HashMap;

/// identifier values of each record of a table
/// This is used in has_ones, has_many lookup fields
/// [FIXME] should be renamed to Identifiable
#[derive(Clone)]
pub struct Identifier{
    /// the primary key value of this record
    /// the selected value is checked against this
    id:String, 
    /// the identifier values
    values:Vec<String>,
    
}


impl Identifier{
    
    /// return the default formatter value
    /// {1}-{2}-{3}...{n} 
    fn get_default_formatted(&self)->String{
        self.format_values("-")
    }
    
     fn get_comma_formatted(&self)->String{
        self.format_values(", ")
    }
    
    fn format_values(&self, separator:&str)->String{
         let mut s = String::new();
        let mut do_sep= false;
        for i in &self.values{
            if do_sep{ s.push_str(separator) } else { do_sep=true; }
            s.push_str(&i);
        }
        s
    }
    
}

/// identifier values of each table, so we don't have to retrieve it everytime
/// Lazy loading
/// This is used in has_ones and has_many(direct/indirect)
struct IdentifierRepo{
    map:HashMap<(String, String), Vec<Identifier>>,
}

impl IdentifierRepo{
    
    /// retrive all the identifier list
    fn get_identifier_list(&self, table:&String, schema:&String)->Option<&Vec<Identifier>>{
        for (key, val) in self.map.iter(){
            let (table_name, schema_name) = key.clone();
            if table == &table_name && schema == &schema_name{
                return Some(val);
            }
        }
        None
    }
    /// get the formatted value of the identifier list
    fn get_identifier_from_table(&self, table:&String, schema:&String, key:&String)->Option<String>{
        let identifier_list = self.get_identifier_list(table, schema);
        match identifier_list{
            Some(list) => Self::get_identifier_from_list(&list, key),
            Nono => None
        }
    }
    
    fn get_identifier_from_list(identifier_list:&Vec<Identifier>, key:&String)->Option<String>{
        for i in identifier_list{
            if &i.id == key{
                return Some(i.get_default_formatted());
            }
        }
        None
    }
}
///extract the identifiers of this table
/// there should be basic tabs for each table
fn extract_identifiers(table:&String, schema:&String, all_tabs:&Vec<Tab>)->Identifier{
    //get the tab associated with this table,
    //the fields of the tab is need for determining which columns are identifiers
    panic!("not yet!");
}

#[test]
fn test_format(){
    let i = Identifier{id:"1".to_string(), values:vec!["hello".to_string(), "world".to_string()]};
    let formatted = i.get_default_formatted();
    assert_eq!(formatted, "hello-world".to_string());
    let formatted = i.get_comma_formatted();
    assert_eq!(formatted, "hello, world".to_string());
}