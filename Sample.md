 http://localhost:8181/data/bazaar.product?product_id=eq.f7521093-734d-488a-9f60-fc9f11f7e750
 
 
 {
  "active": true,
  "barcode": "bar11101",
  "client_id": "ae14a664-aacd-11e5-a4ed-73e0d8bcf46d",
  "created": "2015-11-19T17:10:42.246382+00:00",
  "created_by": "baaef73a-aacd-11e5-a104-d356ffe6408c",
  "currency_id": "c25d844c-aacd-11e5-a4ce-c32886d39239",
  "description": "Second hand Iphone4s",
  "help": "This is a sample with no nulls",
  "info": ["All nulls removed"],
  "is_service": false,
  "name": "iphone4s",
  "organization_id": "984dfcd6-aacd-11e5-b045-d3384d750e86",
  "owner_id": "3e51d5f9-5bff-4664-9946-47bf37973636",
  "parent_product_id": "a1ee3e2c-aacd-11e5-9d69-9b99017cbd77",
  "price": 7000,
  "priority": 7,
  "product_id": "f7521093-734d-488a-9f60-fc9f11f7e750",
  "seq_no": 10,
  "tags": ["good"],
  "unit": "Php",
  "updated": "2015-11-19T17:10:42.246382+00:00",
  "updated_by": "e1d39b0e-aacd-11e5-b58a-a3b380742ecc",
  "upfront_fee": 0,
  "use_parent_price": false
}

 
 
 UPDATE bazaar.product
      SET organization_id = '984dfcd6-aacd-11e5-b045-d3384d750e86' , client_id = 'ae14a664-aacd-11e5-a4ed-73e0d8bcf46d' , created = '2015-11-19T17:10:42.246382Z' , 
      created_by = 'baaef73a-aacd-11e5-a104-d356ffe6408c' , updated = '2015-11-19T17:10:42.246382Z' , updated_by = 'e1d39b0e-aacd-11e5-b58a-a3b380742ecc' , priority = 7 , name = 'iphone4s' , 
      description = 'Second hand Iphone4s' , help = 'This is a sample with no nulls' , active = true , product_id = 'f7521093-734d-488a-9f60-fc9f11f7e750' , parent_product_id = 'a1ee3e2c-aacd-11e5-9d69-9b99017cbd77' 
      , is_service = false , price = 7000 , 
      use_parent_price = false , unit = 'Php' , tags = '["good"]' , 
      info = '["All nulls removed"]'  , seq_no = 10 , upfront_fee = 0 , barcode = 'bar11101' , owner_id = '3e51d5f9-5bff-4664-9946-47bf37973636', currency_id = 'c25d844c-aacd-11e5-a4ce-c32886d39239'
    WHERE product_id = 'f7521093-734d-488a-9f60-fc9f11f7e750'
