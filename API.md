

http://localhost:8181/window/ - list all windows

/window - list all windows
/app/<window> - all associated data with these window

/app/product.0-10[0]/category.0-10[0]/ - starting with product window, list down 0-10 focused on record[0], with category 0-10, focused on record[0].

a more detailed query

/app/product?age=lt.13&(student=eq.true|gender=eq.M)&order_by=age.desc,height.asc&page=20&page_size=100&focused=0/category?name=starts_with(lee)&focused=0


struct app_data{
	table: table,
	filter:
}

/app/[{table:product,filter:age=lt.13&(student=eq.true|gender=eq.M),page=20&page_size=100,focused=0},{table:category,filter:name}]


/app/product?/category/
