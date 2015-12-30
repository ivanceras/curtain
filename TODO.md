
Table Information meta data

```js

@Info{
    'icon': 'http://icon.me/table_icon'
    'display_name': 'Products',
    'identifier': ['name', 'description'],
    'order': ['name', 'description', 'active'],
    'hide': ['createdby', 'created', 'updatedby','updated']
}
```

at product.description

```js

Column information meta data

@Info{
    'display_name': 'Description'
    'display_length': 50,
    'min_length': 1,
    'max_length': 100,
}
```

Nice visualization of Data using D3
http://bl.ocks.org/robschmuecker/7880033

## VOWL standards and specs, useful for table model visualization
http://vowl.visualdataweb.org/webvowl/
https://github.com/VisualDataWeb/WebVOWL


## Need to refactor CORS and token reception
https://github.com/dudadornelles/todo-backend-rust-iron/blob/master/src/main.rs



## data API.

* /window - list down all windows
* /window/<window> - specific window
* /data/<table> - retrieve data from a table
  * paging included, and user specific setting

* /view/<window> - retrieve the specific window
      * include the window info, including the model meta data
      * include the first page of data of the table
      * include all 1:1 data for the extension tables
      * fetch the 1:M data only when the record is focused?
      * eager vs lazy
      * determine how the relative count of detail record, when it reaches more than 10 then probably use lazy loading.
 
 
      
### eager loading: 
- when the record is not put into focused, then the loading was unnecessary
+ smooth introspection of data.
+ paging should be lazy
+ good if there paging size is small.

### lazy loading:
- a bit more complex, algorithmn, including caching
+ minimal server load
 
 
### persistent user preferences:
Each window, a user can set preference.
* which tabs where last opened, which records where focused
* [x] Remember opened tabs and records
* connection urls
* which column to order
* arrangement of columns in the view
* which column are not included/shown
* global settings?, apply these settings to all windows (if applicable) I'll open (may complicate the system)

* settings can be removed everytime.
* history of settings can also be tracked
* filters are expressed in strings
* select=person.name,age&hide=gender&order_by=name.desc&page_size=15
* The user preferences will be stored in a sqlite file in his local app directory
setting

| window  |  value |
|:--------|-------:|
| product | select=person.name,age&hide=gender&order_by=name.desc&page_size=15 |


## create Update / Delete API

* create  - insert each row 1 by 1 or bulk?
    * POST - /product <json_data>
        - new record
        [{name:"swiss knife", desc: "more data"}]
    * csv - use bult insert when csv
       /product?format=csv&column=name,description,price
       "swiss knife","simple cutter","10.00"
       "butterfly knife","evasion 10%","36.00"
       
* update - update data per record/row - use primary keys/unique keys if possible, else use all filters as possible.
    * PUT - /product <json_data>
        [{
            old:{name:"swiss knife", desc: "more data"},
            new:{name:"swift knife", desc: "more date"}
         }
       ]
* delete 
    * DELETE /product <json_data>
        [{name:"swiss knife", desc: "more data"}]


Algorithmn:
Whenever the [Save] button is clicked, the data models is iterated if it is modified or not.
Checking modification is just by comparing the loaded data and the data in the user interface.
* Each data in the user interface may have to be reformatted to the original formatting before it is displayed.

There is a daemon that keeps checking for changes, when a changes is detected
the Save/Update button will be highlighted.


## Optimization

* determine the size of a row,
* rows which contains blobs such as images and base64 encoded data or blobs will have higher counts
* Count the number of bytes the data when retrieve using octet_length, pg_column_size when stored

http://stackoverflow.com/questions/13304572/how-can-pg-column-size-be-smaller-than-octet-lengthngth, 


## Plugin system

Server side code in rust, and can be loaded dynamically using nodejs and invoke appropriate functions
Can have optional client side plugin to enchance the UI display
