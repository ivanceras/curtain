use rustorm::table::Column;
use rustorm::table::Table;

pub struct Mode{
    edit:bool,
    readonly:bool,
    list:bool,
    row:bool,
}
    
    /// try to decode references based on the column, table it is used as context, the reference table when applicable
    /// Html for description,
    /// checkboxes for bools
    /// Dropdown for tables
    /// Calculator for Numeric
    /// Timepicker for time
    /// Datepicker for dates
    /// Phone number with formatting and phone icon
    /// Toggle or checkboxes for booleans
    /// Radio buttons for 1 select among minimal set of options
    /// has_many can be condensed to multiple checkboxes if there arent many options to choose from
    /// such as 10 or less.

    /// interpret the best ui to use for the column, in a table
    fn get_references(column:&Column, table:&Table, ref_table:Table)->Option<String>{
        let column_name = lowercase(&column.name);
        // checkbox, when to use checkbox or toggle?
        let reference = if  &column.data_type == "bool"{
            "Checkbox"
        }
        // a character or a byte
        else if &column.db_data_type == "char"{
            "Character"
        }
        // user
        else if &column_name == "user_id"{
            "User"
        }
        else if &column_name == "organization_id"{
            "Organization"
        }
        else if &column_name == "client_id" || &column_name == "tenant_id"{
            "Client"
        }
        // json
        else if &column.db_data_type == "json"{
            "json"
        }
        //calendar date picker
        else if  &column.data_type == "DateTime<UTC>"{
            "Calendar"
        }
        else if  &column.data_type == "NaiveTime"{
            "TimePicker"
        }
        // larger text box
        else if &column.data_type == "String" 
            && &column.db_data_type == "text"{
            "Markdown"
        }
        //just a simple text box
        else if &column.data_type == "String" 
            && column.db_data_type == "character varying"{
            "TextBox"
        }
        //numeric display use calc
        else if &column.data_type == "f32" 
             || &column.data_type == "f64"{
            "Calculator"
        }
        
        //stepper integer display
        else if &column.data_type == "i16" || 
                &column.data_type == "i32" || 
                &column.data_type == "i64" || 
                &column.data_type == "u32" || 
                &column.data_type == "u64" {
            "Stepper"
        }
        //web url links
        else if column_name.contains("url") || column_name.contains("link"){
            "Url_Link"
        }
        // base64 image photo
        // image box for read, file upload for edit mode
        else if table.name.contains("photo") &&  &column_name == "data"{
            if column.comment.is_some(){
                if column.comment.clone().unwrap().contains("base64"){//@base64
                    "Base64_Image"
                }
                else{
                    "Textbox"
                }
            }
            else{
                "Textbox"
            }
        }
        //flags drop down box with countr name
        else if ref_table.name.contains("country"){
            "Flag_Country"
        }
        // a lot closer to tag selection in select2 angular
        else if ref_table.name.contains("category"){
            "Category"
        }
        else if column_name == "tags"{
            "Tags"
        }
        else if table.name.contains("review") && column_name == "rating"{
            "Review_Star_Rating"
        }
        else{
            "textbox"
        };
        Some(reference.to_string())
    }
    
    /// return the ui field to use when based on the reference returned
    fn get_uifield(mode:&Mode, reference:&String){
        let ui = match reference as &str{
            "TextBox" => {
                if mode.readonly{
                    "div"
                }else{
                    "TextEdit"
                }
            },
             "Base64_Image" => {
                if mode.readonly{
                    "img" //img data=base64;
                }else{
                    "image_upload"
                }
            },
            "Category" => {
                if mode.readonly{
                    "multi-select"
                }else{
                    "tags"
                }
            },
            "Tags" => {
                if mode.readonly{
                    "multi-select"
                }else{
                    "tags"
                }
            },
            _ => "TextBox"
        };
        
    }
    
    
    /// some fields will be displayed in the list, while other are in detail
    fn is_displayed_in_list(column:&Column)->bool{
        let column_name = &column.name as &str;
        match column_name{
         "created" => false,
         "createdby" | "created_by" => false,
         "updated" => false,
         "updatedby" | "updated_by" => false,
         "organization_id" => false,
         "client_id" => false,
         _ => true
        }
    }
    
    /// 
    fn is_displayed_in_detail(column:&Column)->bool{
        let column_name = &column.name as &str;
        match column_name{
         "created" => false,
         "createdby" | "created_by" => false,
         "updated" => false,
         "updatedby" | "updated_by" => false,
         "organization_id" => true,
         "client_id" => true,
         _ => true
        }
    }
    

fn lowercase(str:&str)->String{
     str.chars()
         .flat_map(char::to_lowercase)
         .collect()
}

/// the system should be able to determine and contextualize database tables
/// based on their names and columns
/// it should be able to detect which are users table, which one is the username column and the password column,
/// detection of password hashing is a bit difficult.
fn tag_tables(){

}

#[test]
fn test_lower_case(){
    assert_eq!(lowercase("Description"), "description".to_string());
}