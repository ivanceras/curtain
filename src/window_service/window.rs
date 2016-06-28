use rustorm::table::Table;
use rustorm::table::Column;

use std::collections::HashMap;
use rustc_serialize::json;
use rustorm::query::Query;

use rustorm::database::DatabaseDev;
use window_service::window_api;

/// detailed reference of a field
/// 
pub enum Reference{
    /// the field is treated as just plain data in the table
    Data(String),
    /// it is a predefined list of values, like enum list in database
    List,
    /// This field is a referring column to table lookup from some other table
    /// The string is the table
    DirectTableLookup(String),
    /// There's a linking table and the table to linked to this field
    IndirectTableLookUp(String, String)
}


/// visual presentation of column of a table
/// directly corresponds to a column of a table
/// [FIXME] need more properties here that is extracted from the functions of columns such as the referred tables, Foreign Tables
#[derive(RustcDecodable, RustcEncodable)]
#[derive(Debug)]
#[derive(Clone)]
pub struct Field{
    
    /// derived from column.name or colum.description.@Displayname(value)
    pub name:String,
    
    /// the column name this field corresponds to
    pub column:String,
    
    /// the complete name of the field in the form of {schema}.{table}.{column}
    pub complete_name: String,
    /// this corresponds to a primary column
    pub is_keyfield: bool,
    /// this will be the bases of how to display the field in the UI
    /// such as Date will be displayed with a date picker
    pub data_type:String,
    
    ///reference needed such as if it is a color, person, contact numer, etc.
    /// example values: Table, List, Amount, Quantity, Date, Button(if is an action button)
    /// The column anotated with @Button
    /// its up for the UI to intelligently display the field on the client side
    /// such as displaying a country with the flag at the side of the name
    /// person lookup with a search field and a person logo
    /// Where is the reference stored for @TableDirectHasOne, @TableExt, @TableInDirectHasMany
    /// @ID if this is the ID of the record
    pub reference:String,
    /// suplements the information stored in the reference
    pub reference_value:Option<String>,
    /// short concise description of this field
    pub description:Option<String>,
    ///more information about this field, help text for users
    pub info:Option<String>,
    /// whether or not this field, contributes to the records identity
    /// the distinctive display name will be derived from fields marked a identifier
    pub is_significant:bool,
    /// order by significance, 10, 20, 30.. etc
    /// first significant will be displayed when in compact list
    pub significance_priority: Option<u32>,
    /// whether or not this field, will have to be included in the searching of records
    /// same as is_selection_column
    pub include_in_search:bool,
    /// is the field mandatory for the user to fill up., derived from  not_null is colum
    /// intel tag:@Required/@Mandatory
    pub is_mandatory:bool,

    ///ordering of the fields, when displayed on the UI
    pub seq_no:u32,

    /// marks the field as non-significant but has other uses such as auditing
    /// ie. the updated, created, updated_by, created_by columns are used in auditing of records, but not so very important in daily usage
    pub is_auxilliary: bool,
    /// should be same line or no
    pub is_same_line:bool,
    /// determine if the field should be displayed or not
    /// an intel can run through the columns to see which ones should be in there
    /// which ones should be NOT in there and which ones are optional or not
    /// columns such as client_id, organization_id, created, createdby, updated, updatedby
    /// any other columns that is vaguely known to be displayed or not can use the @Show, or @Hide anotation in its comment
    pub is_displayed:bool,
    ///whether or not this field is editable
    pub is_readonly:bool,
    ///whether or not possible matching values will be displayed while the user is typing
    pub is_autocomplete:bool,
   
    /// a dyanmic code to determine whether this field will be displayed or not
    /// example $active == true
    pub display_logic:Option<String>,
    /// the preffered length of the text this field will occupy 
    pub display_length:Option<u32>, 
    /// this can be displayed in the placeholder value, by evaluating the defaults
    /// such as now() and try to come up with a value, which is the current date
    pub default_value:Option<String>,
}

impl Field{
    
    ///derive a field from a column
    ///[FIXME] A more smarter way decision on other fields,
    /// by regexing to the column description and look for intel
    /// is_display can be run with intel runner
    pub fn from_column(column:&Column, table:&Table, tables:&Vec<Table>)->Field{
        let complete_name = format!("{}.{}",table.complete_name(),column.name);
        let mut field = Field{
            name:column.displayname(),
            column:column.name.clone(),
            complete_name: complete_name,
            is_keyfield:column.is_primary,
            data_type:format!("{:?}", column.data_type),
            reference:column.db_data_type.clone(),
            reference_value:None,
            description:column.comment.clone(),
            info:None,
            is_significant: false,
            significance_priority: None,
            include_in_search:column.is_unique,
            is_mandatory:column.is_primary || column.is_unique,
            seq_no:0,
            is_auxilliary:false,
            is_same_line:false,
            is_displayed:true,
            is_readonly:false,
            is_autocomplete:false,
            display_logic:None,
            display_length:None, 
            default_value:None,
        };
        field.update_field_info(column, table)
    }
    
    /// TODO: If this column refers to a foreign column, the column name will be the condensed table name
    pub fn from_has_one_column(column:&Column, table:&Table, has_one:&Table, tables:&Vec<Table>)->Field{
        let complete_name = format!("{}.{}",table.complete_name(),column.name);
        let mut field = Field{
            name: column.clean_lookupname(table,has_one),//has_one.condensed_displayname(table),
            column:column.name.clone(),
            complete_name: complete_name,
            is_keyfield:column.is_primary,
            data_type: format!("{:?}",column.data_type),
            reference:"Table".to_string(),
            reference_value:Some(has_one.complete_name()),
            description:column.comment.clone(),
            info:None,
            is_significant: false,
            significance_priority: None,
            include_in_search:column.is_unique,
            is_mandatory:column.is_primary || column.is_unique,
            seq_no:0,
            is_auxilliary:false,
            is_same_line:false,
            is_displayed:true,
            is_readonly:false,
            is_autocomplete:false,
            display_logic:None,
            display_length:None, 
            default_value:None,
        };
        field.update_field_info(column, table)
    }
    

    fn update_field_info(mut self, column:&Column, table: &Table)->Field{
        match column.name.as_ref(){
            "client" =>{
                    println!("client matched...");
                    self.is_significant = false;
                    self.seq_no = 100;
                    self.is_displayed = false;
                    self.display_length = Some(20);
                    self.is_readonly = true;
                    self.is_auxilliary = false;
                    self
                },
            "organization" =>{
                    self.is_significant = false;
                    self.seq_no = 120;
                    self.is_displayed = false;
                    self.display_length = Some(20);
                    self.is_readonly = true;
                    self.is_auxilliary = false;
                    self
                },

            "name" =>{
                    self.is_significant = true;
                    self.significance_priority = Some(10);
                    self.seq_no = 200;
                    self.is_displayed = true;
                    self.display_length = Some(20);
                    self.is_readonly = false;
                    self.is_auxilliary = false;
                    self
                },
            "value" =>{
                    self.is_significant = true;
                    self.significance_priority = Some(20);
                    self.seq_no = 210;
                    self.is_displayed = true;
                    self.display_length = Some(20);
                    self.is_readonly = false;
                    self.is_auxilliary = false;
                    self
                },
            "code" =>{
                    self.is_significant = true;
                    self.significance_priority = Some(30);
                    self.seq_no = 220;
                    self.is_displayed = true;
                    self.display_length = Some(20);
                    self.is_readonly = false;
                    self.is_auxilliary = false;
                    self
                },
            "description" =>{
                    self.is_significant = true;
                    self.significance_priority = Some(40);
                    self.seq_no = 230;
                    self.is_displayed = true;
                    self.display_length = Some(100);
                    self.is_readonly = false;
                    self.is_auxilliary = false;
                    self
                },
            "active" =>{
                    self.is_significant = false;
                    self.significance_priority = None;
                    self.seq_no = 240;
                    self.is_displayed = true;
                    self.display_length = Some(100);
                    self.is_readonly = false;
                    self.is_auxilliary = true;
                    self
                },
            "created" =>{
                    self.is_significant = false;
                    self.significance_priority = None;
                    self.seq_no = 300;
                    self.is_displayed = false;
                    self.display_length = Some(20);
                    self.is_readonly = true;
                    self.is_auxilliary = true;
                    self
                },
            "created_by" =>{
                    self.is_significant = false;
                    self.significance_priority = None;
                    self.seq_no = 310;
                    self.is_displayed = false;
                    self.display_length = Some(20);
                    self.is_readonly = true;
                    self.is_auxilliary = true;
                    self
                },
            "updated" =>{
                    self.is_significant = false;
                    self.significance_priority = None;
                    self.seq_no = 320;
                    self.is_displayed = false;
                    self.display_length = Some(20);
                    self.is_readonly = true;
                    self.is_auxilliary = true;
                    self
                },
            "updated_by" =>{
                    self.is_significant = false;
                    self.significance_priority = None;
                    self.seq_no = 330;
                    self.is_displayed = false;
                    self.display_length = Some(20);
                    self.is_readonly = true;
                    self.is_auxilliary = true;
                    self
                },
            _ => {
                    if column.is_unique{
                        self.is_significant = true;
                        self.significance_priority = Some(100);
                        self.seq_no = 215;
                        self.is_displayed =true;
                        self.display_length = Some(20);
                        self.is_readonly = false;
                        self.is_auxilliary = false;
                        self.info = Some("This is a unique column, but uniques are given with lower significance".to_string());

                        self
                    }
                    else if column.is_primary{
                        self.is_significant = false;
                        self.significance_priority = None;
                        self.seq_no = 10;
                        self.is_displayed = false;
                        self.display_length = Some(20);
                        self.is_readonly = false;
                        self.is_auxilliary = false;
                        self.info = Some("This is a primary field".to_string());
                        self
                    }
                    else{
                        if column.name == table.name{
                            self.is_significant = true;
                            self.significance_priority = Some(1);
                            self.seq_no = 1;
                            self.is_displayed = true;
                            self.display_length = Some(20);
                            self.is_readonly = false;
                            self.is_auxilliary = false;
                            self.info = Some ("This is significant field because it pertains to the table".to_string());
                            self
                        }
                        else if table.name.contains(&column.name){
                            self.is_significant = true;
                            self.significance_priority = Some(15) ;
                            self.seq_no = 15;
                            self.display_length = Some(20);
                            self.is_readonly = false;
                            self.is_auxilliary = false;
                            self.info = Some("This is significant because this field pertain to the table".to_string());
                            self
                        }
                        else if column.name.contains(&table.name){
                            self.is_significant = true;
                            self.significance_priority  = Some(19);
                            self.seq_no = 19;
                            self.display_length = Some(20);
                            self.is_readonly = false;
                            self.is_auxilliary = false;
                            self.info = Some("Alright, this is a long column and is recognized as significant".to_string());
                            self
                        }
                        else if column.name.contains("name"){
                            self.is_significant = true;
                            self.significance_priority = Some (500);
                            self.seq_no = 500;
                            self.is_readonly = false;
                            self.is_auxilliary = false;
                            self.info = Some("If no other significant columns, then this will do".to_string());
                            self
                        }
                        else{ 
                            self.is_significant = false;
                            self.seq_no = 1000;
                            self.is_displayed =true;
                            self.display_length = Some(20);
                            self.is_readonly = false;
                            self.is_auxilliary = false;
                            self
                        }
                    }
            }
        } 
    }


}


/// Tab is a visual presentation of a table
/// [FIXME] how are the filters, joins expressed between tab to other tabs
/// When a user open a tab, a list of 10 values will be listed
/// [TODO] add an additional identifer formatting when views as a lookup from other table
/// formatting is only available when there are more than 1 identifier columns
/// the default formatting is {1}-{2}
/// format for user table will be {1}, {2}, i.e 1 = lastname, 2 = firstname
/// TODO: possibility of adding indirect extension table: An extension table whose primary key is the primary key of the extension table which is also a foreign of the primary table lookup
#[derive(RustcDecodable, RustcEncodable)]
#[derive(Debug)]
#[derive(Clone)]
pub struct Tab{
    /// derive from table.displayname()
    pub name:String,
    /// if the contained table is owned by some other table
    pub is_owned: bool,
    /// extension tables' fields will have to be listed along side
    /// with the main tables
    pub is_extension:bool,
    /// has one tables will have 1 field, displaying the identifiers of the referred tables
    pub is_has_one:bool,
    /// can be from direct referring table or indirect linker table
    pub is_has_many:bool,
    /// determines if the derived relation is direct on in_direct
    /// has_one + direct or has_many + direct
    /// has_many + indirect, 
    /// there is no such thing as (has_one + indirect)
    pub is_direct:bool,
	pub linker_table: Option<String>, /// when it is not an indirect table, a linker table must be specified
    pub description:Option<String>,
    /// more information of this tab
    pub info:Option<String>,
    ///which table does this tab corresponds to
    pub table:String,
    pub schema:Option<String>,
    pub fields:Vec<Field>,
    ///optional logo/emblem for the user to uniquely identify this tab.
    ///its color pallete can be used to be as a mini theme of the window itself
    /// in order for the user to have distinct sense on each of the windows, which has
    /// more or less similar set of fields and styles.
    /// a scaled version of the logo can be added to the icon
    pub logo:Option<String>,
    /// a small image to be embedded on the toolbar or tooltip when used in a referred lookup
    pub icon:Option<String>,
    ///
    /// estimated row count for hinting the size of the table to be displayed
    pub estimated_row_count:Option<usize>,
    ///default ordering of (columns, ASC | DESC)
    pub default_order:HashMap<String, bool>,
}

impl Tab{
    
    /// derive a detailed tab from a table definition
    pub fn detailed_from_table(table:&Table, all_tables:&Vec<Table>)->Tab{
        let fields:Vec<Field> = Self::derive_fields(table, all_tables);

        Tab{
            name:table.displayname(),
            is_owned: table.is_owned_or_semi_owned(all_tables),
            is_extension:false,
            is_has_one:false,
            is_has_many:false,
            is_direct:false,
			linker_table: None,
            description:table.comment.clone(),
            info:None,
            table:table.name.clone(),
            schema:table.schema.clone(),
            fields:fields,
            logo:None,
            icon:None,
            estimated_row_count: table.estimated_row_count,
            default_order:HashMap::new(),
        }
    }
    
    /// derive a basic tab from table, use for table identifier values extraction
    pub fn basic_from_table(table:&Table, all_tables:&Vec<Table>)->Tab{
        let fields:Vec<Field> = Self::derive_fields(table, all_tables);

        Tab{
            name:table.displayname(),
            is_owned: table.is_owned_or_semi_owned(all_tables),
            is_extension:false,
            is_has_one:false,
            is_has_many:false,
            is_direct:false,
			linker_table: None,
            description:table.comment.clone(),
            info:None,
            table:table.name.clone(),
            schema:table.schema.clone(),
            fields:fields,
            logo:None,
            icon:None,
            estimated_row_count: table.estimated_row_count,
            default_order:HashMap::new(),
        }
    }
    
    pub fn from_has_one_table(column:&Column, table:&Table, all_tables:&Vec<Table>)->Tab{
        let fields:Vec<Field> = Self::derive_fields(table, all_tables);
        Tab{
            name:column.condense_name(),
            is_owned: table.is_owned_or_semi_owned(all_tables),
            is_extension:false,
            is_has_one:true,
            is_has_many:false,
            is_direct:true,
			linker_table: None,
            description:table.comment.clone(),
            info:None,
            table:table.name.clone(),
            schema:table.schema.clone(),
            fields:fields,
            logo:None,
            icon:None,
            estimated_row_count: table.estimated_row_count,
            default_order:HashMap::new(),
        }
    }
    /// derive an extension tab from_table, do not do recursion of course
    pub fn from_ext_table(ext:&Table, from_table:&Table, all_tables:&Vec<Table>)->Tab{
        let fields:Vec<Field> = Self::derive_fields(ext, all_tables);
        
        Tab{
            name:ext.condensed_displayname(from_table),
            is_owned: ext.is_owned_or_semi_owned(all_tables),
            is_extension:true,
            is_has_one:false,
            is_has_many:false,
            is_direct:false,
			linker_table: None,
            description:ext.comment.clone(),
            info:None,
            table:ext.name.clone(),
            schema:ext.schema.clone(),
            fields:fields,
            logo:None,
            icon:None,
            estimated_row_count: ext.estimated_row_count,
            default_order:HashMap::new(),
        }
    }
    pub fn from_has_many_table(has_many:&Table, table:&Table, all_tables:&Vec<Table>)->Tab{
        let fields:Vec<Field> = Self::derive_fields(has_many, all_tables);
        Tab{
            name:has_many.condensed_displayname(table),
            is_owned: has_many.is_owned_or_semi_owned(all_tables),
            is_extension:false,
            is_has_one:false,
            is_has_many:true,
            is_direct:true,
			linker_table: None,
            description:has_many.comment.clone(),
            info:None,
            table:has_many.name.clone(),
            schema:has_many.schema.clone(),
            fields:fields,
            logo:None,
            icon:None,
            estimated_row_count: has_many.estimated_row_count,
            default_order:HashMap::new(),
        }
    }    
    
    pub fn from_has_many_indirect_table(has_many:&Table, linker_table:&Table, all_tables:&Vec<Table>)->Tab{
        let fields:Vec<Field> = Self::derive_fields(has_many, all_tables);
        Tab{
            name:has_many.condensed_displayname(linker_table),
            is_owned: has_many.is_owned_or_semi_owned(all_tables),
            is_extension:false,
            is_has_one:false,
            is_has_many:true,
            is_direct:false,
			linker_table: Some(linker_table.complete_name()),
            description:has_many.comment.clone(),
            info:None,
            table:has_many.name.clone(),
            schema:has_many.schema.clone(),
            fields:fields,
            logo:None,
            icon:None,
            estimated_row_count: has_many.estimated_row_count,
            default_order:HashMap::new(),
        }
    }
    
	fn derive_fields(table:&Table, all_tables:&Vec<Table>)->Vec<Field>{
        let has_one_fields = Self::derive_has_ones_fields(table, all_tables);
       
        let mut fields = Vec::new();
        for c in &table.columns{
            let has_one_field = Self::in_has_one_fields(c, &has_one_fields);
            if has_one_field.is_some(){
                fields.push(has_one_field.unwrap());
            }
            else{
                fields.push(Field::from_column(c, table, all_tables));
            }
        }
        fields.sort_by(|a,b| a.seq_no.cmp(&b.seq_no));
        fields
    }
    
    fn in_has_one_fields(column:&Column, has_one_fields:&Vec<Field>)->Option<Field>{
         for field in has_one_fields{
            if field.column == column.name{
                return Some(field.clone());
            }
        }
         None
    }
    
    
    fn derive_has_ones_fields(table:&Table, all_tables:&Vec<Table>)->Vec<Field>{
        let mut fields = Vec::new();
        let has_ones = table.referred_tables(all_tables);
        for (column, has1) in has_ones{
            // the corresponding column of the field that derive this table relationship
            // will display the details here, as a drop down box selecting the identifier value
            // of the table that is being refered to.
            let field = Field::from_has_one_column(column, table, has1, all_tables);//should not derive futher tabs
            fields.push(field);
        }
        // all fields of extension table will be listed on the window
        fields
    }
    
    /// extension tables
    
    fn derive_ext_tabs(table:&Table, all_tables:&Vec<Table>)->Vec<Tab>{
        let mut tabs = Vec::new();
        // all fields of extension table will be listed on the window
        for ext in table.extension_tables(all_tables){
            tabs.push(Tab::from_ext_table(ext, table, all_tables));
        }
        tabs
    }
    /// directly referring table
    /// What to do with the case when there is too many table referring this table
    /// Many just add: "used in", but not display the whole details of the tab using it
    /// Solution: Determine popular tables or global tables where they are referenced by most(90%) of the tables in the database
    /// These table includes "users(user_id)", "organization(org_id)"
    /// In turn these popular tables will have a lot of has_many tabs.
    /// To avoid these issue, limit the number of has_tabs in that refers to the popular table
    /// Related table such as "product" which has buyers and sellers but refers to a popular "users(user_id)" table.
    /// Usually the lookup column name (i.e "owner_id", "customer_id"), will be different from the usual referrer column name "user_id".
    fn derive_has_many_tabs(table:&Table, ext_tables:&Vec<&Table>, all_tables:&Vec<Table>)->Vec<Tab>{
        let mut tabs = Vec::new();
        // all fields of has_many tables will be listed on the window
        for (refing_table, column) in table.referring_tables(all_tables){
            if !refing_table.is_linker_table() && !ext_tables.contains(&refing_table){
                tabs.push(Tab::from_has_many_table(refing_table, table, all_tables));
            }
        }
        tabs.sort_by(|a,b| 
            b.estimated_row_count.cmp(&a.estimated_row_count)
            );
        tabs
    }
    
    /// indirectly referring table via linker
    fn derive_has_many_indirect_tabs(table:&Table, all_tables:&Vec<Table>)->Vec<Tab>{
        let mut tabs = Vec::new();
        // all fields of has_many tables will be listed on the window
        for (has_many, linker_table) in table.indirect_referring_tables(all_tables){
            tabs.push(Tab::from_has_many_indirect_table(has_many, linker_table, all_tables));
        }
        tabs
    }
    
    
}

/// directly correspond to a table, no need for tabs
/// TODO: should this include the identifier repo when serialized to the client side
#[derive(RustcDecodable, RustcEncodable)]
#[derive(Debug)]
#[derive(Clone)]
pub struct Window{
    ///name of the window
    /// derive from  "{tab[0].displayname()}, tab[1].condense_name(tab[0].table) & tab[n].displayname()}"
    pub name:String,
    pub description:Option<String>,
    /// the table name used as identifier
    pub table: String,
    pub schema: Option<String>,
    ///main tab, must have at least 1
    /// more helpful information about this window
    pub info:Option<String>,
    pub main_tab:Option<Tab>,
    /// extension tabs
    pub ext_tabs: Vec<Tab>,
    /// has_many tabs
    pub has_many_tabs:Vec<Tab>,
    pub has_many_indirect_tabs:Vec<Tab>,
}


impl Window{
    
    ///Create a window base from a table
    pub fn from_table(table:&Table, all_tables:&Vec<Table>)->Window{
        let ext_tables = table.extension_tables(all_tables);
        let ext_tabs = Tab::derive_ext_tabs(table, all_tables);
        let has_many_tabs = Tab::derive_has_many_tabs(table, &ext_tables, all_tables);
        let has_many_indirect_tabs = Tab::derive_has_many_indirect_tabs(table, all_tables);
        Window{
            name: table.displayname(),
            description: table.comment.clone(),
            table: table.name.to_string(),
            schema: table.schema.to_owned(),
            info: None,
            main_tab:Some(Tab::detailed_from_table(table, all_tables)),
            ext_tabs: ext_tabs,
            has_many_tabs: has_many_tabs,
            has_many_indirect_tabs: has_many_indirect_tabs,
        }
    }
    
    /// just the summary ommitting the need to derive tabs
    pub fn summary_from_table(table:&Table, all_tables:&Vec<Table>)->Window{
        Window{
            name: table.displayname(),
            table: table.name.to_string(),
            schema: table.schema.to_owned(),
            description: table.comment.clone(),
            info: None,
            main_tab:None,
            ext_tabs: vec![],
            has_many_tabs: vec![],
            has_many_indirect_tabs: vec![],
        }
    }
    
}

