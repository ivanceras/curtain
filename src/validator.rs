use global::Context;
use window_service::window_api;
use std::collections::HashSet;
use rustorm::dao::Dao;

/// Validates column names, function names, table names used in the query
/// if they are valid db elements to avoid sql injection
/// intended column that has no counterpart column in the database will be
/// treathed as string value

pub struct DbElementValidator{
	all_schema_names: HashSet<String>,
	all_table_names: HashSet<String>,
	all_column_names: HashSet<String>,
	pub function_validator: FunctionValidator,
}

impl DbElementValidator{

	pub fn from_context(context: &mut Context)->Self{
		let all_table_names = window_api::all_table_names(context);
		let all_column_names = window_api::all_column_names(context);
		let all_schema_names = window_api::all_schema_names(context);
		Self::new(all_schema_names, all_table_names, all_column_names)
	}
	
	pub fn new(all_schema_names: HashSet<String>, all_table_names: HashSet<String>, all_column_names: HashSet<String>)->Self{
		DbElementValidator{
			all_schema_names: all_schema_names,
			all_table_names: all_table_names,
			all_column_names: all_column_names,
			function_validator: FunctionValidator::new(),
		}
	}

	pub fn is_valid_table(&self, arg: &str)->bool{
		if arg.contains("."){
			let splinters:Vec<&str> = arg.split(".").collect();
			if splinters.len() == 2{
				let schema = splinters[0];
				let table = splinters[1];
				if self.all_schema_names.contains(schema) && self.all_table_names.contains(table){
					return true;
				}
			}
		}
		self.all_table_names.contains(arg)

	}

	pub fn is_valid_column(&self, arg: &str)->bool{
		if arg.contains("."){
			let splinters:Vec<&str> = arg.split(".").collect();
			if splinters.len() == 2{
				let table = splinters[0];
				let column = splinters[1];
				if self.all_table_names.contains(table) && self.all_column_names.contains(column){
					return true;
				}
			}
		}
		self.all_column_names.contains(arg)
	}

    /// just check if the keys are valid column
    /// not checking if the column belongs to the specific table
    /// for the dao
    fn is_valid_dao(&self, dao: &Dao) -> bool{
       for key in dao.keys(){
            if self.is_valid_column(&key){
                //valid column
            }else{
                return false;// early return if 1 is not a valid column
            }
       }
       true
    }
}

pub struct FunctionValidator{
	all_function: Vec<String>,
	whitelisted_function: Vec<String>,
	blacklisted_function: Vec<String>,
	allow_all: bool, //if true all functions are allowed, if false only functions in whitelist are allowed.
	all_and_whitelist: bool,// if true all functions are allowed and also user-defined function in the whitelist.
	whitelist_only: bool, // allow the whitelist only
}

impl FunctionValidator{
	
	pub fn new()->Self{
		FunctionValidator{
			all_function: get_all_supported_functions(),
			whitelisted_function: vec![],
			blacklisted_function: vec![],
			allow_all: true,
			all_and_whitelist: true,
			whitelist_only: false,
		}
	}

	pub fn new_with_whitelist(whitelist: Vec<String>)->Self{
		FunctionValidator{
			all_function: get_all_supported_functions(),
			whitelisted_function: whitelist,
			blacklisted_function: vec![],
			allow_all: true,
			all_and_whitelist: true,
			whitelist_only: false,
		}	
	}

	pub fn is_valid_function_name(&self, arg: &String)->bool{
		if !self.blacklisted_function.is_empty(){
			if self.blacklisted_function.contains(arg){
				return false;
			}
		}
		if self.allow_all || self.all_and_whitelist {
			if self.all_function.contains(arg){
				return true;
			}
			if self.whitelisted_function.contains(arg){
				return true;
			}
		}
		if self.whitelist_only{
			return self.whitelisted_function.contains(arg)
		}
		false
	}

}

fn get_all_supported_functions()->Vec<String>{
    vec![
        "sum".to_owned(),
        "max".to_owned(),
        "min".to_owned(),
        "now".to_owned(),
        "count".to_owned(),
        "avg".to_owned(),
    ]
}



