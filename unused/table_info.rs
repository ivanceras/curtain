extern crate rustc_serialize;
use rustc_serialize::json;



/// table information processor
/// information are embedded in table and column comments in json format
#[derive(Debug)]
#[derive(RustcDecodable, RustcEncodable)]
struct TableInfo{
    display_name: Option<String>,
    identifier: Option<Vec<String>>,
    order: Option<Vec<String>>,
    hide: Option<Vec<String>>,
    associate: Option<TableAssociate>,
}

#[derive(Debug)]
#[derive(RustcDecodable, RustcEncodable)]
enum TableAssociate{
    /// indicates that this is a user table
    /// the application may use this a user validation lookup/ and or login
    User,
    Country,
}

#[derive(Debug)]
#[derive(RustcDecodable, RustcEncodable)]
struct ColumnInfo{
    /// a more concise readable for ui display
    display_name: Option<String>,
    /// a hint for the UI
    display_length: Option<u16>,
    /// optional validation in the application level
    min_length: Option<u16>,
    max_length: Option<u16>,
    /// the column may not be constrainted to be NOT NULL but enforced in the application level
    required: Option<bool>, //mandatory
    /// optional associate for known data, for UI enchancement and decorations
    associate: Option<ColumnAssociate>
}
/// Predefined list of associable components.
#[derive(Debug)]
#[derive(RustcDecodable, RustcEncodable)]
enum ColumnAssociate{
    /// Treats the column as description and provides the default attributes for this column
    Description,
    /// This column is used when ordering the records
    Sequence,
    /// This column is a user id and can be used to link to the User table and also lookup
    Userid,
    /// The unique nominal value of the user
    Username,
    /// This column is treated as password and will be displayed in the UI accordingly
    Password(PasswordAlgorithm),
    /// May enchance the ui to display the flag as well in the UI
    CountryCode,
    CountryName,
    /// Money formatting of the ui
    Currency,
    /// This column is an image, could be url, binary data, base64 encoded, the UI may also provide thumbnail
    Image,
    /// A user avatar or icon
    Avatar,
    /// This column is a url link
    Url,
    /// an icon for the entity, will be used as decorations in the UI
    Icon,
    /// This column contains html text, and the ui may provide a renderer
    Html,
    /// Column is a markdown text, the UI may render it as such
    Markdown,
    /// The column is a column and will render the UI with a color picker associated
    Color,
    /// associate the column as a firstname of a person
    /// automatically associated column names are: ['first_name', 'firstname']
    FirstName,
    /// associate the column as a lastname of a person
    /// automatically associated column names are: ['last_name', 'lastname']
    LastName,
    Salutation,
    /// associate the column as email field
    /// automatically associated column names are: ['email']
    Email,
    /// associate the column to active behavior
    /// automatically associated column names: ['is_active':bool, 'active':bool]
    Active,
    Select,
}

/// list of password algorithmn that is used in the user password encryption
#[derive(Debug)]
#[derive(RustcDecodable, RustcEncodable)]
enum PasswordAlgorithm{
    Plaintext,
    Md5,
    Sha1,
    Sha224, 
    Sha256, 
    Sha384, 
    Sha512,
    Scrypt,
    Bcrypt,
    Pbkdf2,
}


#[test]
fn test_encode(){
    let column_info = ColumnInfo {
            display_name: Some("Description".to_string()),
            display_length: Some(50),
            min_length: Some(1),
            max_length: Some(100),
            associate: Some(ColumnAssociate::Password(PasswordAlgorithm::Pbkdf2))
        };
    let content = format!("{}", json::as_pretty_json(&column_info));
    println!("column_info: {:#?}", content);
    assert!(1==2);
}


#[test]
fn test_column_info(){
            
    let info = r#"
    {
        'display_name': 'Description',
        'display_length': 50,
        'min_length': 1,
        'max_length': 100,
        'associate': 'Description'
    }
    "#;

    println!("info: {}",info);
    let info = info.replace("'","\"");
    println!("quoted info: {}",info);
    let column_info: ColumnInfo = json::decode(&info).unwrap();
    
    println!("{:#?}", column_info);
    assert!(1==2);

}
