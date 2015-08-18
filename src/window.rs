use rustorm::table::Table;
use rustorm::table::Column;

use std::collections::HashMap;
use rustc_serialize::json;
use rustorm::query::Query;
use identifier::Identifier;
use codegenta::generator;

use rustorm::database::DatabaseDev;


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
    pub is_identifier:bool,
    /// whether or not this field, will have to be included in the searching of records
    /// same as is_selection_column
    pub include_in_search:bool,
    /// is the field mandatory for the user to fill up., derived from  not_null is colum
    /// intel tag:@Required/@Mandatory
    pub is_mandatory:bool,
    ///ordering of the fields, when displayed on the UI
    pub seq_no:u32,
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
    pub display_length:Option<String>, 
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
        Field{
            name:column.displayname(),
            column:column.name.clone(),
            is_keyfield:column.is_primary,
            data_type:column.data_type.clone(),
            reference:column.db_data_type.clone(),
            reference_value:None,
            description:column.comment.clone(),
            info:None,
            is_identifier: column.is_unique,
            include_in_search:column.is_unique,
            is_mandatory:column.is_primary || column.is_unique,
            seq_no:0,
            is_same_line:false,
            is_displayed:true,
            is_readonly:false,
            is_autocomplete:false,
            display_logic:None,
            display_length:None, 
            default_value:column.default.clone(),
        }
    }
    
    pub fn from_has_one_column(column:&Column, table:&Table, has_one:&Table, tables:&Vec<Table>)->Field{
        Field{
            name:column.condense_name(),
            column:column.name.clone(),
            is_keyfield:column.is_primary,
            data_type:column.data_type.clone(),
            reference:"Table".to_string(),
            reference_value:Some(has_one.name.to_string()),
            description:column.comment.clone(),
            info:None,
            is_identifier:column.is_unique,
            include_in_search:column.is_unique,
            is_mandatory:column.is_primary || column.is_unique,
            seq_no:0,
            is_same_line:false,
            is_displayed:true,
            is_readonly:false,
            is_autocomplete:false,
            display_logic:None,
            display_length:None, 
            default_value:column.default.clone(),
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
#[derive(RustcDecodable, RustcEncodable)]
#[derive(Debug)]

pub struct Tab{
    /// derive from table.displayname()
    pub name:String,
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
    pub description:Option<String>,
    /// more information of this tab
    pub info:Option<String>,
    ///which table does this tab corresponds to
    pub table:String,
    pub schema:String,
    pub fields:Vec<Field>,
    /// extension tabs
    pub ext_tabs:Option<Vec<Tab>>,
    /// has_many tabs
    pub has_many_tabs:Option<Vec<Tab>>,
    pub has_many_indirect_tabs:Option<Vec<Tab>>,
    ///optional logo/emblem for the user to uniquely identify this tab.
    ///its color pallete can be used to be as a mini theme of the window itself
    /// in order for the user to have distinct sense on each of the windows, which has
    /// more or less similar set of fields and styles.
    /// a scaled version of the logo can be added to the icon
    pub logo:Option<String>,
    /// a small image to be embedded on the toolbar or tooltip when used in a referred lookup
    pub icon:Option<String>,
    /// an optional page size of when paging records on this tab
    /// items_per_page
    pub page_size:Option<u32>,
    ///default ordering of (columns, ASC | DESC)
    pub default_order:HashMap<String, bool>,
}

impl Tab{
    
    /// derive a detailed tab from a table definition
    pub fn detailed_from_table(table:&Table, all_tables:&Vec<Table>)->Tab{
        let fields:Vec<Field> = Self::derive_fields(table, all_tables);
        let ext_tables = table.extension_tables(all_tables);
        let ext_tabs = Some(Self::derive_ext_tabs(table, all_tables));
        let has_many_tabs = Some(Self::derive_has_many_tabs(table, &ext_tables, all_tables));
        let has_many_indirect_tabs = Some(Self::derive_has_many_indirect_tabs(table, all_tables));


        Tab{
            name:table.displayname(),
            is_extension:false,
            is_has_one:false,
            is_has_many:false,
            is_direct:false,
            description:table.comment.clone(),
            info:None,
            table:table.name.clone(),
            schema:table.schema.clone(),
            fields:fields,
            ext_tabs:ext_tabs,
            has_many_tabs:has_many_tabs,
            has_many_indirect_tabs:has_many_indirect_tabs,
            logo:None,
            icon:None,
            page_size:None,
            default_order:HashMap::new(),
        }
    }
    
    /// derive a basic tab from table, use for table identifier values extraction
    pub fn basic_from_table(table:&Table, all_tables:&Vec<Table>)->Tab{
        let fields:Vec<Field> = Self::derive_fields(table, all_tables);

        Tab{
            name:table.displayname(),
            is_extension:false,
            is_has_one:false,
            is_has_many:false,
            is_direct:false,
            description:table.comment.clone(),
            info:None,
            table:table.name.clone(),
            schema:table.schema.clone(),
            fields:fields,
            ext_tabs:None,
            has_many_tabs:None,
            has_many_indirect_tabs:None,
            logo:None,
            icon:None,
            page_size:None,
            default_order:HashMap::new(),
        }
    }
    
    pub fn from_has_one_table(column:&Column, table:&Table, all_tables:&Vec<Table>)->Tab{
        let fields:Vec<Field> = Self::derive_fields(table, all_tables);
        Tab{
            name:column.condense_name(),
            is_extension:false,
            is_has_one:true,
            is_has_many:false,
            is_direct:true,
            description:table.comment.clone(),
            info:None,
            table:table.name.clone(),
            schema:table.schema.clone(),
            fields:fields,
            ext_tabs:None,
            has_many_tabs:None,
            has_many_indirect_tabs:None,
            logo:None,
            icon:None,
            page_size:None,
            default_order:HashMap::new(),
        }
    }
    /// derive an extension tab from_table, do not do recursion of course
    pub fn from_ext_table(ext:&Table, from_table:&Table, all_tables:&Vec<Table>)->Tab{
        let fields:Vec<Field> = Self::derive_fields(ext, all_tables);
        
        Tab{
            name:ext.condensed_displayname(from_table),
            is_extension:true,
            is_has_one:false,
            is_has_many:false,
            is_direct:false,
            description:ext.comment.clone(),
            info:None,
            table:ext.name.clone(),
            schema:ext.schema.clone(),
            fields:fields,
            ext_tabs:None,
            has_many_tabs:None,
            has_many_indirect_tabs:None,
            logo:None,
            icon:None,
            page_size:None,
            default_order:HashMap::new(),
        }
    }
    pub fn from_has_many_table(has_many:&Table, table:&Table, all_tables:&Vec<Table>)->Tab{
        let fields:Vec<Field> = Self::derive_fields(has_many, all_tables);
        Tab{
            name:has_many.condensed_displayname(table),
            is_extension:false,
            is_has_one:false,
            is_has_many:true,
            is_direct:true,
            description:has_many.comment.clone(),
            info:None,
            table:has_many.name.clone(),
            schema:has_many.schema.clone(),
            fields:fields,
            ext_tabs:None,
            has_many_tabs:None,
            has_many_indirect_tabs:None,
            logo:None,
            icon:None,
            page_size:None,
            default_order:HashMap::new(),
        }
    }    
    
    pub fn from_has_many_indirect_table(has_many:&Table, linker_table:&Table, all_tables:&Vec<Table>)->Tab{
        let fields:Vec<Field> = Self::derive_fields(has_many, all_tables);
        Tab{
            name:has_many.condensed_displayname(linker_table),
            is_extension:false,
            is_has_one:false,
            is_has_many:true,
            is_direct:false,
            description:has_many.comment.clone(),
            info:None,
            table:has_many.name.clone(),
            schema:has_many.schema.clone(),
            fields:fields,
            ext_tabs:None,
            has_many_tabs:None,
            has_many_indirect_tabs:None,
            logo:None,
            icon:None,
            page_size:None,
            default_order:HashMap::new(),
        }
    }
    /// [FIXME] determine which fields are used in has_ones table
    fn derive_fields(table:&Table, all_tables:&Vec<Table>)->Vec<Field>{
        let has_one_fields = Self::derive_has_ones_fields(table, all_tables);
       
        let mut fields = Vec::new();
        for c in &table.columns{
            let has_on_field = Self::in_has_one_fields(c, &has_one_fields);
            if has_on_field.is_some(){
                fields.push(has_on_field.unwrap());
            }
            else{
                fields.push(Field::from_column(c, table, all_tables));
            }
        }
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
    /// [FIXME] don't include extension tables and linker tables
    /// What to do with the case when there is too many table referring this table
    /// Many just add: "used in", but not display the whole details of the tab using it
    fn derive_has_many_tabs(table:&Table, ext_tables:&Vec<&Table>, all_tables:&Vec<Table>)->Vec<Tab>{
        let mut tabs = Vec::new();
        // all fields of has_many tables will be listed on the window
        for (refing_table, column) in table.referring_tables(all_tables){
            if !refing_table.is_linker_table() && !ext_tables.contains(&refing_table){
                tabs.push(Tab::from_has_many_table(refing_table, table, all_tables));
            }
        }
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
    
    
    /// build a query that extracts the identifier list of this table
    fn build_query_for_identifier_list(&self)->Vec<Identifier>{
        let mut q = Query::select();
        q.from_table(&self.table);
        
        for field in &self.fields{
            if field.is_identifier || field.is_keyfield {
                q.column(&format!("{}.{}",self.table, field.column));
            }
        }
        panic!("soon");
    }
}

/// directly correspond to a table, no need for tabs
/// TODO: should this include the identifier repo when serialized to the client side
#[derive(RustcDecodable, RustcEncodable)]
#[derive(Debug)]
pub struct Window{
    ///name of the window
    /// derive from  "{tab[0].displayname()}, tab[1].condense_name(tab[0].table) & tab[n].displayname()}"
    pub name:String,
    pub description:Option<String>,
    /// the table name used as identifier
    pub table: String,
    ///main tab, must have at least 1
    /// more helpful information about this window
    pub info:Option<String>,
    pub tab:Option<Tab>,
}


impl Window{
    
    ///Create a window base from a table
    pub fn from_table(table:&Table, all_tables:&Vec<Table>)->Window{
        Window{
            name: table.displayname(),
            description: table.comment.clone(),
            table: table.name.to_string(),
            info: None,
            tab:Some(Tab::detailed_from_table(table, all_tables)),
        }
    }
    
    /// just the summary ommitting the need to derive tabs
    pub fn summary_from_table(table:&Table, all_tables:&Vec<Table>)->Window{
        Window{
            name: table.displayname(),
            table: table.name.to_string(),
            description: table.comment.clone(),
            info: None,
            tab:None,
        }
    }
    
    /// build the SQL query based on the table involved in the tabs
    ///
    /// * columns are enumerated based on the is_displayed
    ///
    /// * values of has_ones tables, primary columns and columns marked as identifier are the only values extracted
    ///    they can involved all the unique values of the table
    ///    they identifier value of the matching primary value will be displayed on the field
    ///    has_one table values are extracted with another query, the correct value is marked with is_chosen = true
    /// 
    /// * ext_tabs
    ///    extension tabs table values are extracted as a normal tab itself.
    ///    extension tables are extracted using left join
    /// 
    ///  * has_many direct  
    ///      has_many direct tables are extracted using another query
    ///      the enumerated columns is based on the is_displayed field
    ///      the records of the fields that are drop down will still be all retrieved since the user can 
    ///      alter these values, the selected value will is based on the id of the table
    ///
    //// * has_many indirect
    ///     has_many indirect tables are extracted using dual left joins
    ///     the main table is left join to the direct table left joining the indirect table.
    ///
    /// This function should be marked as extract_data, based on the window structure
    /// has_many should be extracted independently, since they are not immediate data and will cause a longer load times
    /// while the user may not to see that data.
    
    fn build_query(&self){
        let q = Query::select();
        let tab = &self.tab.as_ref().unwrap();
        let table = &tab.table;
        let page_size = &tab.page_size;
        let fields = &tab.fields;
        // retrieve the has_many only of the first record
        
    }
}

/// list a sumamary of window tables
pub fn list_windows(tables:&Vec<Table>)->Vec<Window>{
    let window_tables = window_tables(tables);
    let mut window_list = vec![];
    for t in window_tables{
        let window = Window::summary_from_table(t, tables);
        window_list.push(window);
    }
    window_list
}

///
/// return the list of tables that has a window
///
fn window_tables(tables:&Vec<Table>)->Vec<&Table>{
    let mut window_tables = Vec::new();
    let all_extension_tables = get_all_extension_tables(tables);
    for t in tables{
        if t.is_linker_table(){
            println!("NOT a Window: {} <<-linker table", t.name);
        }
        else{   
            if t.is_owned(tables){
                println!("OWNED table: {}", t.name);
            }
            else{
                if all_extension_tables.contains(&t){
                    println!("EXTENSION table: {}", t);
                }
                else{
                    println!("{}", t.name);
                    window_tables.push(t);
                    for (col, has1) in t.referred_tables(tables){
                        println!("\t has one: {} -> {}", col.condense_name(), has1);
                    }
                    for ext in t.extension_tables(tables){
                        println!("\t ext tab: {} [{}]", ext.name, ext.condensed_displayname(t));
                    }
                    for (has_many, column) in t.referring_tables(tables){
                        if !has_many.is_linker_table(){
                            println!("\t has many direct: {} [{}] via column: {}", has_many.name, has_many.condensed_displayname(t), column.name);
                        }else{
                            //println!("\t has many direct: {} <---- but is a linker table, so no!", has_many.name);
                        }
                    }
                    for (has_many,linker) in t.indirect_referring_tables(tables){
                        println!("\t has many INDIRECT: {}[{}], via {}",has_many.name, has_many.condensed_displayname(t), linker.name);
                    }
                }
            }
        }
    }
    window_tables
}

//// a summary of windows
/// build windows from a set of tables
/// 
pub fn extract_windows(tables:&Vec<Table>)->Vec<Window>{
    
    let window_tables = window_tables(tables);
    let mut all_windows = vec![];
    for wt in window_tables{
        println!("{}", wt);
        let window = Window::from_table(&wt, tables);
        all_windows.push(window);
    }
    all_windows
}



pub fn get_window(db_dev:&DatabaseDev, table_name: &str)->Result<Window, String>{
    println!("getting window: {}", table_name);
    let tables = generator::get_all_tables(db_dev);
    let windows = extract_windows(&tables);
    for win in windows{
        if win.table == table_name{
            return Ok(win);
        }
    }
    Err(format!("No window for {}",table_name))
}

fn get_all_extension_tables(tables:&Vec<Table>)->Vec<&Table>{
    let mut all_extension_tables = Vec::new();
    for t in tables{
        for ext in t.extension_tables(tables){
            if !all_extension_tables.contains(&ext){
                println!("extension table: {}", ext);
                all_extension_tables.push(ext);
            }
        }
    }
    all_extension_tables

}

