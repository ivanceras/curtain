
## Main table


* Records insert in the main table is inserted as it
* Records inserted in the ext_table will be inserted and refers to a record in the main table
* Records inserted in has_many direct(ex: `order_line` in `orders` table) table has 2 cases
   - If the direct table is owned by the main table, the record is inserted to the direct table
   - If it is not owned and may have a window of its own, the record must already be an existing record of the direct table

* Records inserted in indirect table
  - The record must already been an existing record in the indirect table, and a new record will be inserted into the linker table referring the main record and the indirect record.
 
 
## Examples

* Main table: product
 - ext_table: product_availability
 - has_many: 
 - indirect: category via `product_category`, photo via `product_photo`

* Main table: orders
 - ext_table:
 - has_many: order_line
 - indirect: 
 

## Insert, Update, Delete
 * Main table
   - Insert
   - Update
   - Delete
 
 * Extension table
   - Insert
   - Update
   - Delete
   
 * Has Many (Not owned)
   - Insert
   - Delete
 
 * Has Many ( Owned )
   - Insert
   - Update
   - Delete
 
 * Indirect
   - Insert
   - Delete
 	
