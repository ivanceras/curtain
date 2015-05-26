##Converting markdown to html
https://crates.io/crates/hoedown


##Sanitizing html input from users
https://github.com/earthreader/rust-earth


#$Notes 
*Window seperator will be based on the extension table
*Many to Many relation is based on the direct table
*All many to many has separate Windows, containing each others window instances
*Linker tables has no windows

##Indirect tables to indirect tables
How far down to the rabbit hole
* Most common table relation has 1 extension table and 1 many relation
* Do we include indirect relation from indirect relation

Notes:
* extension tables will be inlined with the main table.
has many will be inlined when there is only 1 in a table.
* multiple has many table will be listed as tabs with 1 default maybe opened. can be listed ordered by name alphabetically
server request round trips.
defaults will have to do a server round trip since it wouldnt know which record is opened.
* The record of the parent table will be used as filter for the records.
extension tables will be fecthed togehter with the main record since it is sure that there is only 1 extension record.
* If a table is a has many, does it make since to display its own windows, such case is, order_line, category