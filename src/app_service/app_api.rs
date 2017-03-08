use inquerest;
use global;
use global::Context;
use window_service::window_api;
use validator::DbElementValidator;
use from_query::FromQuery;
use rustorm::query::Select;
use inquerest as iq;
use rustorm::dao::Dao;
use rustorm::dao::DaoResult;
use rustorm::table::Table;
use rustorm::query::Equality;
use rustorm::query::{Filter, Join, JoinType, Modifier, Condition, Operand, Connector};
use rustorm::query::{TableName, ToTableName};
use rustorm::query::source::ToSourceField;
use rustorm::query::column_name::{ToColumnName, ColumnName};
use rustc_serialize::json;
use std::collections::BTreeMap;
use rustorm::dao::Value;
use data_service::data_api;
use uuid::Uuid;
use rustc_serialize::json::Json;
use rustorm::database::DbError;
use error::ParseError;
use error::ServiceError;
use error::ParamError;
use window_service::window::Window;
use window_service::window::Tab;
use rustc_serialize::json::DecoderError;
use config;
use url::percent_encoding;
use rustorm::query::IsQuery;
use rustorm::query::Update;
use rustorm::query::Delete;
use rustorm::query::Insert;


pub fn focused_record(context: &mut Context,
                      main_table: &str,
                      url_query: &Option<String>)
                      -> Result<Vec<TableDao>, ServiceError> {
    let validator = DbElementValidator::from_context(context);
    println!("url query: {:?}", url_query);
    let (main_table_filter, rest_table_filter) = parse_complex_url_query(main_table, url_query);
    println!("filter: {:#?}", rest_table_filter);
    let main_validated = main_table_filter.transform(context, &validator);
    match main_validated {
        Ok(main_validated) => {
            let mut rest_validated = vec![];
            for rest in rest_table_filter {
                match rest.transform(context, &validator) {
                    Ok(rtransformed) => {
                        rest_validated.push(rtransformed);
                    }
                    Err(e) => (),
                }
            }
            println!("main validated: {:#?}", main_validated);
            let rest_data: Result<Vec<TableDao>, ServiceError> =
                retrieve_data_from_focused_dao(context, &main_validated, &rest_validated);
            rest_data
        }
        Err(e) => Err(ServiceError::from(e)), 
    }
}

pub fn complex_query(context: &mut Context,
                     main_table: &str,
                     url_query: &Option<String>)
                     -> Result<Vec<TableDao>, ServiceError> {
    let validator = DbElementValidator::from_context(context);
    println!("url_query: {:?}", url_query);
    let (main_table_filter, rest_table_filter) = parse_complex_url_query(main_table, url_query);
    println!("filter: {:#?}", rest_table_filter);
    let main_validated = main_table_filter.transform(context, &validator);
    //println!("main validated: {:#?}", main_validated);
    match main_validated {
        Ok(main_validated) => {
            let mut rest_validated = vec![];
            for rest in rest_table_filter {
                match rest.transform(context, &validator) {
                    Ok(rtransformed) => {
                        rest_validated.push(rtransformed);
                    }
                    Err(e) => (),
                }
            }
            println!("main validated query: {:#?}", &main_validated.query);
            let rest_data: Result<Vec<TableDao>, ServiceError> =
                retrieve_main_data(context, &main_validated, &rest_validated);
            rest_data
        }
        Err(e) => Err(ServiceError::from(e)), 
    }
}

/// retrieve the window data for all tabs involved in this window
fn retrieve_main_data(context: &mut Context,
                      main_query: &ValidatedQuery,
                      rest_vquery: &Vec<ValidatedQuery>)
                      -> Result<Vec<TableDao>, ServiceError> {
    let main_table: Table = main_query.table.clone();
    let main_window = match window_api::retrieve_window(context, &main_table.name) {
        Ok(main_window) => main_window,
        Err(e) => {
            return Err(ServiceError::from(e));
        }
    };
    let mut table_dao = vec![];
    let mut mquery: Select = match main_query.query {
        Some(ref query) => query.clone(),
        None => Select::new(),
    };
    let main_table_name = main_table.to_table_name();
    mquery.enumerate_from_table(&main_table_name);
    mquery.from(&main_table.clone());
    if mquery.get_range().limit.is_none() {
        mquery.set_limit(config::default_page_size);
    }
    let db = &*context.db()?;
    let main_debug = mquery.debug_build(db);
    let main_dao_result = {
        let debug_sql = mquery.debug_build(db);
        match mquery.retrieve(db) {
            Ok(main_dao_result) => main_dao_result,
            Err(e) => {
                return Err(ServiceError::from(e));
            }
        }
    };
    let main_table_dao = TableDao::from_dao_result(&main_dao_result, &main_table.complete_name());
    table_dao.push(main_table_dao);
    Ok(table_dao)
}

fn retrieve_data_from_focused_dao(context: &mut Context,
                                  main_query: &ValidatedQuery,
                                  rest_vquery: &Vec<ValidatedQuery>)
                                  -> Result<Vec<TableDao>, ServiceError> {
    // all the other table dao will not be retrieved when there is no focused record on the main tab
    // if there is a focused record, then the list of records in the main table will not be included
    let mut table_dao = vec![];
    let main_table: Table = main_query.table.clone();
    let window = match window_api::retrieve_window(context, &main_table.name) {
        Ok(window) => window,
        Err(e) => {
            return Err(ServiceError::from(e));
        }
    };
    if let Some(main_tab) = window.main_tab {
        match &main_query.focus_param {
            &Some(ref focus_param) => {
                let focused_filter = focused_param_as_filter(&main_table, &focus_param);
                let ext_tab_dao = retrieve_data_from_direct_tabs(context,
                                                                 &main_table,
                                                                 &focused_filter,
                                                                 rest_vquery,
                                                                 &window.ext_tabs);
                match ext_tab_dao {
                    Ok(ext_tab_dao) => table_dao.extend_from_slice(&ext_tab_dao),
                    Err(e) => return Err(e),
                }
                let has_many_dao = retrieve_data_from_direct_tabs(context,
                                                                  &main_table,
                                                                  &focused_filter,
                                                                  rest_vquery,
                                                                  &window.has_many_tabs);
                has_many_dao.map(|dao| table_dao.extend_from_slice(&dao));
                let indirect_dao = retrieve_data_from_indirect_tabs(context,
                                                                    &main_table,
                                                                    &focused_filter,
                                                                    rest_vquery,
                                                                    &window.has_many_indirect_tabs);
                indirect_dao.map(|dao| table_dao.extend_from_slice(&dao));
                Ok(table_dao)
            }
            &None => Err(ServiceError::new(&format!("no focused record specified"))),
        }
    } else {
        Err(ServiceError::new(&format!("no main tab for table {}", main_table.complete_name())))
    }
}


pub fn update_data(context: &mut Context,
                   main_table: &str,
                   updatable_data: &str)
                   -> Result<Vec<UpdateResponse>, ServiceError> {
    if updatable_data.trim().is_empty() {
        return Err(ServiceError::from(ParamError::new("empty updatable data")));
    } else {
        let changeset: Result<Vec<Changeset>, DecoderError> = json::decode(updatable_data);
        match changeset {
            Ok(changeset) => {
                let window = window_api::retrieve_window(context, main_table);
                match window {
                    Ok(window) => {
                        let result = apply_data_changeset(context, &window, &changeset);
                        match result{
                            Ok(result) => Ok(result),
                            Err(e) => Err(ServiceError::from(e))
                        }
                    }
                    Err(e) => {
                        return Err(ServiceError::from(e));
                    }
                }
            }
            Err(e) => {
                return Err(ServiceError::from(ParseError::new(&format!("{}", e))));
            }
        }
    }
}

#[derive(Debug)]
#[derive(Clone)]
struct WindowTabTables {
    main_table: (Tab, Table),
    ext_tables: Vec<(Tab, Table)>,
    has_many_tables: Vec<(Tab, Table)>,
    has_many_indirect: Vec<(Tab, Table, Table)>, // the last table arg is the linker table
}

fn extract_window_tables(context: &mut Context,
                         window: &Window)
                         -> Result<WindowTabTables, DbError> {
    if let Some(ref main_tab) = window.main_tab {
        let main_table = match window_api::get_matching_table(context, &main_tab.table) {
            Some(main_table) => main_table,
            None => {
                return Err(DbError::new("no main table found"));
            }
        };
        let mut ext_tables = vec![];
        let mut has_many_tables = vec![];
        let mut indirect_tables = vec![];
        for ext_tab in &window.ext_tabs {
            let ext_table = window_api::get_matching_table(context, &ext_tab.table);
            ext_table.map(|table| ext_tables.push((ext_tab.clone(), table)));
        }
        for has_many_tab in &window.has_many_tabs {
            let has_many_table = window_api::get_matching_table(context, &has_many_tab.table);
            has_many_table.map(|table| has_many_tables.push((has_many_tab.clone(), table)));
        }
        for indirect in &window.has_many_indirect_tabs {
            let indirect_table = window_api::get_matching_table(context, &indirect.table);
            assert!(indirect.linker_table.is_some(),
                    "There should be a linker here");
            let linker_table_name = indirect.linker_table.as_ref().unwrap();
            let linker = match window_api::get_matching_table(context, &linker_table_name) {
                Some(linker) => linker,
                None => return Err(DbError::new("linker table not found")),
            };
            indirect_table.map(|table| indirect_tables.push((indirect.clone(), table, linker)));
        }
        let window_tables = WindowTabTables {
            main_table: (main_tab.clone(), main_table),
            ext_tables: ext_tables,
            has_many_tables: has_many_tables,
            has_many_indirect: indirect_tables,
        };
        Ok(window_tables)
    } else {
        Err(DbError::new("No main tab"))
    }
}

/// for each table on this window, get their corresponding changeset and apply each changeset accordingly
fn apply_data_changeset(context: &mut Context,
                        window: &Window,
                        changesets: &Vec<Changeset>)
                        -> Result<Vec<UpdateResponse>, DbError> {
    let window_tables = match extract_window_tables(context, window) {
        Ok(window_tables) => {
            let main_updates:Vec<UpdateResponse> =
                apply_changeset_to_main_table(context, &window_tables.main_table, changesets)?;
            let direct_updates =
                apply_changeset_to_direct_tables(context,
                                                 &window_tables.main_table,
                                                 &window_tables.ext_tables,
                                                 changesets)?;
            let indirect_updates = 
                apply_changeset_to_indirect_tables(context,
                                                   &window_tables.main_table,
                                                   &window_tables.has_many_indirect,
                                                   changesets)?;
            let mut update_response = vec![];
                update_response.extend(main_updates);
                update_response.extend(direct_updates);
                update_response.extend(indirect_updates);
                return Ok(update_response);
        }
        Err(e) => {
            return Err(DbError::new("no window tables"));
        }
    };
    Ok(vec![])
}

/// only owned direct table will be updated on this place since
/// not owned direct table may have also been referenced somewhere at different context
fn apply_changeset_to_direct_tables(context: &mut Context,
                                    main_table: &(Tab, Table),
                                    direct_tables: &Vec<(Tab, Table)>,
                                    changesets: &Vec<Changeset>)
                                    -> Result<Vec<UpdateResponse>, DbError> {
    println!("applying changeset to direct tables...");
    for &(ref direct_tab, ref direct) in direct_tables {
        let changeset = changesets.find(&direct.complete_name());
        if let Some(changeset) = changeset {
            println!("direct changeset: {:#?}", changeset);
        } else {
            println!("------------>>>> No direct changeset");
        }
    }
    Ok(vec![])
}


/// delete only upto the linker
fn apply_changeset_to_indirect_tables(context: &mut Context,
                                      main_tab_table: &(Tab, Table),
                                      indirect_tables: &Vec<(Tab, Table, Table)>,
                                      changesets: &Vec<Changeset>)
                                      -> Result<Vec<UpdateResponse>, DbError> {
    let &(ref main_tab, ref main_table) = main_tab_table;
    println!("---->>>>>>>> INDIRECT TABLES <<<<<<<--------");
    println!("applying changeset to indirect tables..");
    for &(ref indirect_tab, ref indirect, ref linker) in indirect_tables {
        let changeset = changesets.find(&indirect.complete_name());
        if let Some(changeset) = changeset {
            println!("indirect changeset: {:#?}", changeset);
            for ref insert in &changeset.inserted {
                // whatever is the primary key of this insert dao, use it in the record in the linker table
                // this needs access to the parent record
                insert_dao_to_linker(context, &main_table, indirect, linker, insert);
            }
            for ref delete in &changeset.deleted {

            }
            for ref update in &changeset.updated {

            }
        } else {
            println!("----->>>No indirect changeset");
        }
    }
    Ok(vec![])
}

fn insert_dao_to_linker(context: &mut Context,
                        main_table: &Table,
                        table: &Table,
                        linker: &Table,
                        insert: &DaoInsert) {

}


fn apply_changeset_to_main_table(context: &mut Context,
                                 main_tab_table: &(Tab, Table),
                                 changesets: &Vec<Changeset>)
                                 -> Result<Vec<UpdateResponse>, DbError> {
    println!("applying changeset to main table: {:?}", changesets);
    let &(ref main_tab, ref main_table) = main_tab_table;
    let changeset = changesets.find(&main_table.name);//TODO: should employ smarter matching
    println!("Finding change set for {} there is----->>> {:?}",
             &main_table.complete_name(),
             changeset);
    let mut inserted_dao = vec![];// this is a list of updated insert with their primary keys set in the db
    let mut insert_error = vec![];
    if let Some(changeset) = changeset {
        println!("main changeset: {:?}", changeset);
        for ref insert in &changeset.inserted {
            println!("inserts: {:#?}", insert);
            let result = insert_dao(context, &main_tab_table, &insert.dao);
            match result{
                Ok(result) => {
                    let updated_insert = DaoInsert::update_from_inserted_dao(&insert, &result);
                    inserted_dao.push(updated_insert.dao);
                }
                Err(e) => {
                    insert_error.push((insert.dao.clone(), format!("{}", e)));
                }
            }
        }
        println!("-->>> There are {} DAO's to be deleted",
                 changeset.deleted.len());

        let mut deleted = vec![];
        let mut delete_error = vec![];
        for delete in &changeset.deleted {
            // determine if the table properties which decides
            // whether to delete its referring record
            // when this dao is referred, and the referring table is an extension table
            // delete the record in the extension table first, then this.
            // delete the extension first

            // TODO: the main da, primary keys may not be the same as the name in the refering tables
            // will have to recreate the filter for each referring tables
            delete_records_in_extension_tables(context, main_table, delete);
            delete_records_in_direct_tables(context, main_table, delete);
            delete_records_in_linker_tables(context, main_table, delete);

            let result = delete_dao(context, main_tab_table, delete);
            match result {
                Ok(result) => {
                    deleted.push(delete.clone());
                }
                Err(e) => {
                    delete_error.push((delete.clone(), format!("{}",e)))
                }
            }
        }
        let mut all_updated = vec![];
        let mut update_error:Vec<(Dao,String)> = vec![];
        for ref update in &changeset.updated {
            println!("update: {:#?}", update);
            let dao = update_dao(context, &main_tab_table, update);
            println!("after update dao");
            match dao{
                Ok(dao) => {
                    match dao{
                        Some(dao) => {
                            all_updated.push(dao);
                        },
                        None => {
                            update_error.push((update.updated.clone(), "Nothing is updated".to_string()))
                        }
                    }
                },
                Err(e) => {
                    update_error.push((update.updated.clone(), format!("{}", e)));
                }
            }
            println!("after match");
        }
        let total_records = try!(get_total_records(context, main_table));
        let update_response = 
            UpdateResponse{
                 deleted: deleted,
                 delete_error: delete_error,
                 updated: all_updated,
                 update_error: update_error,
                 inserted: inserted_dao,
                 insert_error: insert_error,
                 table: main_table.complete_name(), // always use complete names in responses
                 total_records: total_records
             };
        return Ok(vec![update_response])
    }
    Ok(vec![])
}


fn get_total_records(context: &mut Context, table: &Table)-> Result<usize, DbError> {
    let mut query = Select::new();
    query.column("COUNT(*) as COUNT");
    query.from(table);
    let db = &*context.db()?;
    let result = query.retrieve_one(db)?;
    match result{
        Some(result) => {
            let count = result.get("count");
            match count {
                Some(&Value::U64(v)) => Ok(v as usize),
                Some(&Value::I64(v)) => Ok(v as usize),
                Some(&Value::U32(v)) => Ok(v as usize),
                Some(&Value::I32(v)) => Ok(v as usize),
                _ => Err(DbError::new("error converting"))
            }
        },
        None => Err(DbError::new("Can't get count"))
    }
}

fn delete_records_in_extension_tables(context: &mut Context, main_table: &Table, dao: &Dao) {
    let all_tables = window_api::get_tables(context);
    let extension_tables = main_table.extension_tables(&all_tables);
    for ext_table in extension_tables {
        let filter = build_translated_filter(main_table, dao, &ext_table);
        println!("ext table: {}", ext_table.complete_name());
        delete_dao_with_filter(context, ext_table, &filter);
    }
}

/// create a filter for referring table `table` using the record `main_dao` which is
/// a record in context from `main_table`
/// Note: `main_dao` column names is in context with `main_table`
/// so the equivalent column name of `table` which foreign refers to `main_table` must be used
///
fn build_translated_filter(main_table: &Table, main_dao: &Dao, table: &Table) -> Vec<Filter> {
    let mut filters = vec![];
    let foreign_column = table.get_foreign_columns_to_table(main_table);
    for fc in foreign_column {
        println!("fc: {} foreign to {:?}", fc.complete_name(), fc.foreign);
        if let Some(ref foreign) = fc.foreign {
            let main_pk = &foreign.column;
            let main_value = main_dao.get(main_pk);// take the value using the pk name
            if let Some(main_value) = main_value {
                let filter = Filter::new(&fc.name, Equality::EQ, main_value);//but save the value using the local column name
                filters.push(filter);
            }
        }
    }
    filters
}


/// delete only when the direct table is owned
fn delete_records_in_direct_tables(context: &mut Context, main_table: &Table, dao: &Dao) {
    let all_tables = window_api::get_tables(context);
    let direct_tables = main_table.direct_tables(&all_tables);
    for direct in direct_tables {
        let filter = build_translated_filter(main_table, dao, &direct);
        println!("direct table: {}", direct.complete_name());
        delete_dao_with_filter(context, direct, &filter);
    }
}

/// delete only up to the linker table, since the actual indirect table may also be used in other contexts
fn delete_records_in_linker_tables(context: &mut Context, main_table: &Table, dao: &Dao) {
    let all_tables = window_api::get_tables(context);
    let indirect_tables = main_table.indirect_tables(&all_tables);
    for (indirect, linker, via_column) in indirect_tables {
        let filter = build_translated_filter(main_table, dao, &linker);
        println!("indirect: {} linker: {}",
                 indirect.complete_name(),
                 linker.complete_name());
        delete_dao_with_filter(context, linker, &filter);
    }
}

fn update_dao(context: &mut Context,
              tab_table: &(Tab, Table),
              dao: &DaoUpdate)
              -> Result<Option<Dao>, DbError> {
    let &(ref tab, ref table) = tab_table;
    let filters = create_filter_from_dao(&table, &dao.original);
    let min = dao.minimize_update();
    let mut query = Update::table(&table.to_table_name());
    for column in &table.columns {
        let value = min.get(&column.name);
        if let Some(value) = value {
            query.set(&column.name, value);
        }
    }
    query.add_filters(&filters);
    query.return_all();
    let result:Result<Dao,DbError> = context.db()?.update(&query);
    match result {
        Ok(result) => Ok(Some(result)),
        Err(e) => Ok(None),
    }
}

fn delete_dao_with_filter(context: &mut Context,
                          table: &Table,
                          filters: &Vec<Filter>)
                          -> Result<usize, DbError> {
    let mut query = Delete::from(&table.to_table_name());
    query.add_filters(filters.clone());
    let db = &*context.db()?;
    let debug = query.debug_build(db);
    query.execute(db)
}

fn delete_dao(context: &mut Context, tab_table: &(Tab, Table), dao: &Dao) -> Result<usize, DbError> {
    let &(ref tab, ref table) = tab_table;
    let filters = create_filter_from_dao(&table, dao);
    let mut query = Delete::from(&table.to_table_name());
    query.add_filters(filters);
    let db = &*context.db()?;
    let result = query.execute(db);
    println!("result: {:?}", result);
    result
}

fn insert_dao(context: &mut Context, tab_table: &(Tab, Table), dao: &Dao) -> Result<Dao, DbError> {
    println!("About to insert dao: {:?}", dao);
    let &(ref tab, ref table) = tab_table;
    let mut query = Insert::into(&table.to_table_name());
    for column in &table.columns {
        if let Some(value) = dao.get(&column.name) {
            query.set(&column.name, value);
        }
    }
    query.return_all();
    let db = &*context.db()?;
    let debug = query.debug_build(db);
    println!("DEBUG SQL: {}", debug);
    let dao: Result<Dao, DbError> = db.insert(&query);
    println!("inserted dao: {:?}", dao);
    dao
}





fn parse_complex_url_query(main_table: &str,
                           url_query: &Option<String>)
                           -> (TableFilter, Vec<TableFilter>) {
    let mut main_table_filter = TableFilter {
        table: main_table.to_owned(),
        filter: None,
    };
    let mut rest_table_filter = vec![];

    if let &Some(ref query) = url_query {
        let query_decode = percent_encoding::percent_decode(query.as_bytes()).decode_utf8();
        println!("decoded query: {:#?}", query_decode);
        let clean_query: String = 
            match query_decode{
                Ok(ref query_decode) => query_decode.to_string(),
                Err(e) => query.clone(),
            };
        let table_queries: Vec<&str> = clean_query.split("/").collect();
        if table_queries.len() > 0 {
            main_table_filter.filter = Some(table_queries[0].to_owned());
        };
        let rest_query: Vec<&&str> = table_queries.iter().skip(1).collect();
        for q in rest_query {
            let table_filter: Vec<&str> = q.split("?").collect();
            let table = if table_filter.len() > 0 {
                Some(table_filter[0])
            } else {
                None
            };

            let filter = if table_filter.len() > 1 {
                Some(table_filter[1].to_owned())
            } else {
                None
            };

            if let Some(tbl) = table {
                let table_filter = TableFilter {
                    table: tbl.to_owned(),
                    filter: filter,
                };
                rest_table_filter.push(table_filter);
            }

        }
    }
    (main_table_filter, rest_table_filter)
}

#[test]
fn test_parse_simple_query(){
    let url_query = "price=gt.100.012e-10&order_by=product.seq_no&group_by=product.seq_no&limit=10".to_string();
    let (main_filter, rest_filter) = parse_complex_url_query("product", &Some(url_query));
    println!("main filter: {:#?}", main_filter);
    println!("rest filter: {:#?}", rest_filter);
    assert_eq!(rest_filter.len(), 0);
}

#[test]
fn test_parse_complex_query(){
    let url_query = "price=gt.100.012e-10&order_by=product.seq_no&limit=10&focused=3/category?category.name=eq.accessories&order_by=name.asc.nullsfirst&focused=0".to_string();
    let (main_filter, rest_filter) = parse_complex_url_query("product", &Some(url_query));
    println!("main filter: {:#?}", main_filter);
    println!("rest filter: {:#?}", rest_filter);
    assert_eq!(rest_filter.len(), 1);
}

#[test]
fn test_parse_complex_query_with_percent20(){
    let url_query = "msg=like.hello%20world".to_string();
    let (main_filter, rest_filter) = parse_complex_url_query("message", &Some(url_query));
    println!("main filter: {:#?}", main_filter);
    println!("rest filter: {:#?}", rest_filter);
    assert_eq!(TableFilter{
            table: "message".to_string(),
            filter: Some("msg=like.hello world".to_string())
            }, main_filter);
    assert_eq!(rest_filter.len(), 0);
}



/// can be used by the direct 1:1 and direct 1:M tab
fn retrieve_data_from_direct_tabs(context: &mut Context,
                                  main_table: &Table,
                                  main_with_focused_filter: &Vec<Filter>,
                                  rest_vquery: &Vec<ValidatedQuery>,
                                  tabs: &Vec<Tab>)
                                  -> Result<Vec<TableDao>, ServiceError> {
    let mut tabs_table_dao = vec![];
    for tab in tabs {
        let table = match window_api::get_matching_table(context, &tab.table) {
            Some(table) => table,
            None => {
                return Err(ServiceError::new("Unable to get table for extension"));
            }
        };
        let mut query = build_query(&main_table, &table, &main_with_focused_filter, rest_vquery);
        let db = &*context.db()?;
        let debug = query.debug_build(db);
        println!("-->> DIRECT TAB SQL {}", debug);
        match query.retrieve(db) {
            Ok(dao_result) => {
                let table_dao = TableDao::from_dao_result(&dao_result, &table.complete_name());
                tabs_table_dao.push(table_dao);
            }
            Err(e) => {
                return Err(ServiceError::from(e));
            }
        }
    }
    Ok(tabs_table_dao)
}

fn retrieve_data_from_indirect_tabs(context: &mut Context,
                                    main_table: &Table,
                                    main_with_focused_filter: &Vec<Filter>,
                                    rest_vquery: &Vec<ValidatedQuery>,
                                    indirect_tabs: &Vec<Tab>)
                                    -> Result<Vec<TableDao>, ServiceError> {

    let mut indirect_table_dao = vec![];

    for indirect in indirect_tabs {
        // will use inner join to linker table, and inner join to the indirect
        let indirect_table = match window_api::get_matching_table(context, &indirect.table) {
            Some(indirect_table) => indirect_table,
            None => {
                return Err(ServiceError::new("Unable to get table for extension"));
            }
        };
        assert!(indirect.linker_table.is_some(),
                format!("indirect tab {} must have a linker table specified",
                        indirect.table));
        let linker_table_name = &indirect.linker_table.as_ref().unwrap();
        let linker_table = match window_api::get_matching_table(context, &linker_table_name) {
            Some(linker_table) => linker_table,
            None => {
                return Err(ServiceError::new("linker table can not be found"));
            }
        };
        let mut ind_query = build_query_with_linker(&main_table,
                                                    &indirect_table,
                                                    &main_with_focused_filter,
                                                    &linker_table,
                                                    rest_vquery);

        let db = &*context.db()?;
        let debug = ind_query.debug_build(db);
        println!("-->> INDIRECT TAB SQL {}", debug);
        match ind_query.retrieve(db) {
            Ok(dao_result) => {
                let table_dao = TableDao::from_dao_result(&dao_result,
                                                          &indirect_table.complete_name());
                indirect_table_dao.push(table_dao);
            }
            Err(e) => {
                return Err(ServiceError::from(e));
            }
        }
    }
    Ok(indirect_table_dao)
}


fn build_query(main_table: &Table,
               ext_table: &Table,
               main_filter: &Vec<Filter>,
               rest_vquery: &Vec<ValidatedQuery>)
               -> Select {
    let mut ext_query = Select::new();
    let ext_table_name = ext_table.to_table_name();
    ext_query.enumerate_from_table(&ext_table_name);
    ext_query.from(main_table);
    let on_filter = create_on_filter(&main_table, &ext_table);
    let ext_join = Join {
        modifier: None,
        join_type: Some(JoinType::INNER),
        table_name: ext_table_name,
        on: on_filter,
    };
    ext_query.joins.push(ext_join);
    for filter in main_filter {
        ext_query.add_filter(filter);
    }
    let ext_filters = rest_vquery.find_query(ext_table);
    if let Some(ext_filters) = ext_filters {
        for ext_filter in ext_filters.filters {
            ext_query.add_filter(&ext_filter);
        }
    }
    ext_query
}

fn build_query_with_linker(main_table: &Table,
                           indirect_table: &Table,
                           main_filter: &Vec<Filter>,
                           linker_table: &Table,
                           rest_vquery: &Vec<ValidatedQuery>)
                           -> Select {
    let mut query = Select::new();
    query.enumerate_from_table(&indirect_table.to_table_name());
    query.from(main_table);
    let join1 = Join {
        modifier: None,
        join_type: Some(JoinType::INNER),
        table_name: linker_table.to_table_name(),
        on: create_on_filter(&main_table, &linker_table),
    };
    query.joins.push(join1);
    let join2 = Join {
        modifier: None,
        join_type: Some(JoinType::INNER),
        table_name: indirect_table.to_table_name(),
        on: create_on_filter(&indirect_table, &linker_table),
    };
    query.joins.push(join2);
    for filter in main_filter {
        query.add_filter(filter);
    }
    let ind_query = rest_vquery.find_query(indirect_table);
    if let Some(ind_query) = ind_query {
        for indirect_filter in ind_query.filters {
            query.add_filter(&indirect_filter);
        }
    }
    query
}

fn extract_focused_dao(table: &Table,
                       dao_list: &Vec<Dao>,
                       focus_param: &Option<FocusParam>)
                       -> Option<Dao> {
    if let &Some(ref focus_param) = focus_param {
        match focus_param {
            &FocusParam::Index(index) => {
                if dao_list.len() > index {
                    Some(dao_list[index].clone())
                } else {
                    None
                }
            }
            &FocusParam::PrimaryBlob(ref blob) => {
                let pkeys = table.primary_columns();
                let blob: Vec<&str> = blob.split(",").collect();
                assert!(pkeys.len() == blob.len(),
                        "focused_record should only have the same length as the number of \
                         primary keys");
                let mut needle: BTreeMap<String, Value> = BTreeMap::new();
                for i in 0..pkeys.len() {
                    let pk = pkeys[i];
                    let bvalue = blob[i];
                    let value = Value::String(bvalue.to_owned());
                    let corrected_value = data_api::correct_value_type(&value, &pk.data_type);
                    needle.insert(pk.name.to_owned(), corrected_value);
                }
                println!("focused record map: {:#?}", needle);
                let focused_dao = find_from_dao_list(dao_list, &needle);
                println!("focused dao: {:#?}", focused_dao);
                focused_dao
            }
        }
    } else {
        None
    }
}

fn focused_param_as_filter(table: &Table, focused_param: &FocusParam) -> Vec<Filter> {
    match focused_param {
        &FocusParam::Index(index) => {
            panic!("this is unsupported");
        }
        &FocusParam::PrimaryBlob(ref blob) => {
            let pkeys = table.primary_columns();
            let blob: Vec<&str> = blob.split(",").collect();
            assert!(pkeys.len() == blob.len(),
                    "focused_record should only have the same length as the number of primary \
                     keys");
            let mut filters = vec![];
            for i in 0..pkeys.len() {
                let pk = pkeys[i];
                let bvalue = blob[i];
                let value = Value::String(bvalue.to_owned());
                let corrected_value = data_api::correct_value_type(&value, &pk.data_type);
                let complete_name = format!("{}.{}", table.name, pk.name);
                let filter = Filter::new(&complete_name, Equality::EQ, &corrected_value);
                filters.push(filter);
            }
            filters
        }
    }
}

fn match_needle(dao: &Dao, needle: &BTreeMap<String, Value>) -> bool {
    let mut matches = 0;
    for key in needle.keys() {
        let dao_value = dao.get(key);
        let needle_value = needle.get(key);
        if let Some(needle_value) = needle_value {
            if let Some(dao_value) = dao_value {
                if needle_value == dao_value {
                    matches += 1;
                } else {
                    return false;
                }
            }
        }
    }
    needle.keys().len() == matches
}

fn find_from_dao_list(dao_list: &Vec<Dao>, needle: &BTreeMap<String, Value>) -> Option<Dao> {
    for dao in dao_list {
        if match_needle(dao, needle) {
            return Some(dao.clone());
        }
    }
    None
}

/// extract the filters of this query from main table, but adds details to the column names to avoid unambigous feild
/// if there is no table specified in the column name, the main table is added, else leave it as it is
fn extract_comprehensive_filter(main_table: &Table, main_query: &Select) -> Vec<Filter> {
    let mut filters = vec![];
    for filter in &main_query.filters {
        let mut condition = filter.condition.clone();
        let left = match &condition.left {
            &Operand::ColumnName(ref column) => {
                let mut column = column.clone();
                if column.table.is_some() {
                    // leave as is
                } else {
                    column.table = Some(main_table.name.to_owned())
                }
                Operand::ColumnName(column)
            }
            _ => condition.left.clone(),
        };
        let right = match &condition.right {
            &Operand::ColumnName(ref column) => {
                let mut column = column.clone();
                if column.table.is_some() {
                    // leave as is
                } else {
                    column.table = Some(main_table.name.to_owned())
                }
                Operand::ColumnName(column)
            }
            _ => condition.right.clone(),
        };
        condition.left = left;
        condition.right = right;
        let mut new_filter = filter.clone();
        new_filter.condition = condition;
        filters.push(new_filter);
    }
    filters
}

fn create_on_filter(main_table: &Table, table: &Table) -> Filter {
    let foreign_columns = table.get_foreign_columns_to_table(&main_table);
    let mut filters = vec![];
    for fc in foreign_columns {
        let fk = match fc.foreign {
            Some(ref fk) => format!("{}.{}", fk.table, fk.column),
            None => panic!("no foreign key found"),
        };
        let condition = Condition {
            left: Operand::ColumnName(fc.to_column_name()),
            equality: Equality::EQ,
            right: Operand::ColumnName(ColumnName::from_str(&fk)),
        };
        let filter = Filter {
            connector: Connector::Or, // TODO: need more work, should have for each column
            condition: condition,
            sub_filters: vec![],
        };
        filters.push(filter);
    }

    // flatten filter into 1, by anding the rest to the first filter;
    assert!(filters.len() > 0, "There must be at least 1 filter created");
    let mut first_filter = filters[0].clone();
    if filters.len() > 1 {
        let rest_filter = &filters[1..filters.len()];
        for rfilter in rest_filter {
            first_filter.sub_filters.push(rfilter.clone());
        }
    }
    first_filter
}

trait QuerySearch {
    fn find_query(&self, table: &Table) -> Option<Select>;
}

impl QuerySearch for Vec<ValidatedQuery> {
    fn find_query(&self, table: &Table) -> Option<Select> {
        for vquery in self {
            // find if the vquery will match this table
            if &vquery.table == table {
                return vquery.query.clone();
            }

        }
        None
    }
}

/// build a filter based on primary key and its value
fn create_filter_from_dao(table: &Table, focused_dao: &Dao) -> Vec<Filter> {
    let mut filters = vec![];
    for pri in table.primary_columns() {
        let pvalue = focused_dao.get(&pri.name);
        if let Some(pvalue) = pvalue {
            let column = pri.complete_name();
            let filter = Filter::new(&column, Equality::EQ, &pvalue.to_owned());
            filters.push(filter)
        }
    }
    filters
}


#[derive(Debug)]
#[derive(RustcEncodable)]
#[derive(Clone)]
pub struct TableDao {
    table: String,
    dao_list: Vec<Dao>,
    /// the page of the current fetch
    page: Option<usize>,
    /// page size of this fetch
    page_size: Option<usize>,
    /// the total number of items
    /// of this query, disregarding the page_size limit
    total: Option<usize>,
}


impl TableDao {
    fn from_dao_result(dao_result: &DaoResult, table_name: &str) -> Self {
        TableDao {
            table: table_name.to_owned(),
            dao_list: dao_result.dao.to_owned(),
            page: dao_result.page,
            page_size: dao_result.page_size,
            total: dao_result.total,
        }
    }
}



#[derive(Debug)]
struct ValidatedQuery {
    table: Table,
    query: Option<Select>,
    focus_param: Option<FocusParam>,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct TableFilter {
    pub table: String,
    pub filter: Option<String>,
}

impl TableFilter {
    fn transform(&self,
                 context: &mut Context,
                 validator: &DbElementValidator)
                 -> Result<ValidatedQuery, ParseError> {

        if validator.is_valid_table(&self.table) {
            let table = match window_api::get_matching_table(context, &self.table) {
                Some(table) => table,
                None => {
                    return Err(ParseError::new("Unable to get matching table"));
                }
            };
            match self.filter {
                Some(ref filter) => {
                    let parsed = inquerest::parse(&filter);
                    println!("parsed: {:?}", parsed);
                    match parsed {
                        Ok(parsed) => {
                            let focus_param = extract_focus_param(&parsed);
                            println!("got focused_param: {:#?}", focus_param);
                            let transformed = parsed.transform(&validator);
                            let vquery = ValidatedQuery {
                                table: table,
                                query: Some(transformed),
                                focus_param: focus_param,
                            };
                            Ok(vquery)
                        }
                        Err(e) => Err(ParseError::new(&format!("unable to parse query {:?}", e))),
                    }
                }
                None => {
                    let vquery = ValidatedQuery {
                        table: table,
                        query: None,
                        focus_param: None,
                    };
                    Ok(vquery)
                }
            }
        } else {
            Err(ParseError::new("table is invalid"))
        }
    }
}


/// prioritized focused_record than focused
/// TODO: deal with composite primary keys 
fn extract_focus_param(iquery: &iq::Select) -> Option<FocusParam> {
    let focused_record = find_in_equation(&iquery.equations, "focused_record");
    println!("focused_record: {:?}", focused_record);
    if let Some(focused_record) = focused_record {
        match focused_record {
            &iq::Operand::Number(number) => {
                let primary_blob = format!("{}", number);
                return Some(FocusParam::PrimaryBlob(primary_blob));
            }
            &iq::Operand::Column(ref column) => {
                // example: focused_record=efc62342-1230af,100001
                let primary_blob = format!("{}", column);
                return Some(FocusParam::PrimaryBlob(primary_blob));
            }
            &iq::Operand::Boolean(value) => {
                let primary_blob = format!("{}", value);
                return Some(FocusParam::PrimaryBlob(primary_blob));
            }
            &iq::Operand::Value(ref value) => {
                let primary_blob = format!("{}", value);
                return Some(FocusParam::PrimaryBlob(primary_blob));
            }
            &iq::Operand::Function(ref value) => {
                panic!("functions are unsupported");
            }
        }
    }
    None
}

/// return the right operand of the equation when this colum match
fn find_in_equation<'a>(equations: &'a Vec<iq::Equation>,
                        left_column: &str)
                        -> Option<&'a iq::Operand> {
    for ref eq in equations {
        match &eq.left {
            &iq::Operand::Column(ref column) => {
                if column == left_column {
                    return Some(&eq.right);
                }
            }
            _ => (),
        }
    }
    None
}

/// index will be the relative position of the record
/// record will be transformed into a filter of the primary keys
/// comma separated values of the primary key value with respect to the
/// position order of the primary keys in the table
#[derive(Debug)]
enum FocusParam {
    Index(usize),
    PrimaryBlob(String),
}



/// when a dao is updated
#[derive(Debug)]
#[derive(RustcEncodable)]
#[derive(RustcDecodable)]
pub struct DaoUpdate {
    pub original: Dao,
    pub updated: Dao,
}

impl DaoUpdate {
    /// get only the minimal update by including only the
    /// changed values
    pub fn minimize_update(&self) -> Dao {
        let mut changeset = Dao::new();
        let keys = self.original.keys();
        for key in keys {
            let updated_value = self.updated.get(key);
            let orig_value = self.original.get(key);
            if updated_value == orig_value {
                println!("no change for {}", key);
            } else {
                if let Some(updated_value) = updated_value {
                    changeset.insert(key.to_owned(), updated_value.to_owned());
                }
            }
        }
        changeset
    }
}

#[test]
fn test_dao_minimize_update() {
    let mut dao1 = Dao::new();
    dao1.insert("key1".to_owned(), Value::String("value1".to_owned()));
    dao1.insert("key2".to_owned(), Value::String("value2".to_owned()));

    let mut dao2 = dao1.clone();
    dao2.insert("key2".to_owned(),
                Value::String("I changed this".to_owned()));

    let update = DaoUpdate {
        original: dao1.clone(),
        updated: dao2.clone(),
    };
    let min = update.minimize_update();
    println!("min: {:#?}", min);
    let mut expected = Dao::new();
    expected.insert("key2".to_owned(),
                    Value::String("I changed this".to_owned()));

    assert_eq!(expected, min);
}

/// Dao for inserting, but with record ID to
/// identify which record it is before actually inserting into the database
/// which may change the primary key value, due to database custom function
/// that may have been used to generate it.
/// The record_id is useful when a referring record from some other
/// table is also created which also refers to this main record.
/// This is akin to simultaneously inserting relative records into different table
/// the primary key of the main record will have to return and will be used in the referring table
/// for the succedding insert operation
#[derive(Debug)]
#[derive(RustcEncodable)]
#[derive(RustcDecodable)]
#[derive(Clone)]
pub struct DaoInsert {
    pub record_id: Uuid, // if record that is about to be inserted belongs to the main table
    dao: Dao, // the dao to be inserted
    referred_record: Option<ReferredRecord>,
}

#[derive(Debug)]
#[derive(RustcEncodable)]
#[derive(RustcDecodable)]
#[derive(Clone)]
pub enum ReferredRecord {
    TemporaryId(Uuid), /* when it is just inserted as well, and has not been saved into the database to yet obtain it's primary key values */
    Dao(Dao), // when it is an existing record
}

impl DaoInsert {
    /// call this when the record has been inserted into the database with
    /// the primary values already being set
    fn update_from_inserted_dao(orig: &DaoInsert, inserted_dao: &Dao) -> Self {
        let mut dao_insert = orig.clone();
        dao_insert.dao = inserted_dao.clone();
        dao_insert
    }

    /// use this when inserting to the main table
    pub fn for_main(dao: &Dao) -> Self {
        DaoInsert {
            record_id: Uuid::new_v4(),
            dao: dao.clone(),
            referred_record: None,
        }
    }

    pub fn from_dao_with_existing_parent(dao: &Dao, main_dao: &Dao) -> Self {
        DaoInsert {
            record_id: Uuid::new_v4(),
            dao: dao.clone(),
            referred_record: Some(ReferredRecord::Dao(main_dao.clone())),
        }
    }

    pub fn from_dao_with_newly_inserted_parent(dao: &Dao, uuid: Uuid) -> Self {
        DaoInsert {
            record_id: Uuid::new_v4(),
            dao: dao.clone(),
            referred_record: Some(ReferredRecord::TemporaryId(uuid)),
        }
    }
}

trait SearchDaoInsert {
    fn find(&self, record_id: &Uuid) -> Option<&DaoInsert>;
}

impl SearchDaoInsert for Vec<DaoInsert> {
    fn find(&self, record_id: &Uuid) -> Option<&DaoInsert> {
        for insert in self {
            if &insert.record_id == record_id {
                return Some(insert);
            }
        }
        None
    }
}

/// the changeset of dao in a table
/// all the inserted dao will be inserted first
/// all the updated ones will be updated 2nd
/// all the deleted ones will be deleted last
/// deletion may cause referential integrity errors
/// insert changeset must bear the parent table dao to know which referred primary value to refer to
/// deleted changeset in extension tables deletes it
/// deleted changeset in direct has_many table (order->order_line) deletes it.
/// deleted changeset in indirect has_many table only deletes the record in the linker table that refers
/// the record and the main table dao,
/// an optional delete the record in the indirect table when it is not refered by anywhere else for sanitation purposes
/// and cleanup maintenance.
#[derive(Debug)]
#[derive(RustcEncodable)]
#[derive(RustcDecodable)]
pub struct Changeset {
    pub table: String,
    pub inserted: Vec<DaoInsert>,
    pub deleted: Vec<Dao>, /* if in has_many, delete the record, if has_many indirect, delete the linker record (and? as well together with the indirect record)?? if still referred by some other table */
    pub updated: Vec<DaoUpdate>, /* it's primary key value stay in change so no worry about reference errors, updates on has_many and has_many indirect may not be allowed */
}


#[derive(Debug)]
#[derive(RustcEncodable)]
#[derive(RustcDecodable)]
pub struct UpdateResponse {
    pub table: String,
    pub inserted: Vec<Dao>,
    pub insert_error: Vec<(Dao, String)>,
    pub deleted: Vec<Dao>,
    pub delete_error: Vec<(Dao, String)>,
    pub updated: Vec<Dao>,
    pub update_error: Vec<(Dao, String)>,
    pub total_records: usize
}


trait Search {
    type Output;
    fn find(&self, needle: &str) -> Option<&Self::Output>;
}

impl Search for Vec<Changeset> {
    type Output = Changeset;

    fn find(&self, table: &str) -> Option<&Changeset> {
        for cs in self {
            if table == cs.table {
                return Some(cs);
            }
        }
        None
    }
}
