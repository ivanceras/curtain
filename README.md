# Curtain

[![Build Status](https://api.travis-ci.org/ivanceras/curtain.svg)](https://travis-ci.org/ivanceras/curtain)

A smart data editing tool for non-programmers.

![](https://raw.githubusercontent.com/ivanceras/curtain/master/screenshots/curtain-elm.png)

Long:

The UI is created using the meta data information of tables and their foreign key relationships. If a certain record in a table refer to a foreign record in some other table, instead of displaying the foreign key value, a derived recognizable representation of the referenced record will be displayed.

The project even goes farther by displaying the pertaining details of record in focused.
If you click on a certain product, the list of its category, orders, reviews, photos will also be implicitly displayed as its detail.
 

As in our demo database, setting a price of a [product](https://github.com/ivanceras/mockdata/blob/master/schema/product.sql) with multiple currency will be something like

```sql
CREATE TABLE product
(
  product_id uuid ..,
  name character varying
  description character varying,
  price double precision,
  currency_id uuid
  ....
  CONSTRAINT product_pkey PRIMARY KEY (product_id)
);

```
It will be rendered like this.

![](https://raw.githubusercontent.com/ivanceras/curtain/master/screenshots/product.png)


## Online Demo

[herokuapp demo](http://curtain-elm.herokuapp.com)

[digital ocean demo](http://45.55.7.231:8080/)

Data and schema of the database used in the demo can be found here [mockdata](https://github.com/ivanceras/mockdata) which was generated using mockaroo.com


## Architecture
  - Server side:
   - rust
   - iron
   - rustorm
   - postgresql
  
## Client implementation
 1. [Curtain-elm](https://github.com/ivanceras/curtain-elm) - This is the main client implemented in [elm](elm-lang.org)

 2. [curtain_ui](https://github.com/ivanceras/curtain_ui) - plain javascript implementation using mithril and material design
   ![](https://raw.githubusercontent.com/ivanceras/curtain_ui/master/screenshots/curtain_ui.png)

 3. [curtain_gtk](https://github.com/ivanceras/curtain_gtk) - gtk client (WIP) Doing [gtk-rs](https://github.com/gtk-rs/gtk) UI is not very ergonomic, and the project is not yet matured.
    ![](https://raw.githubusercontent.com/ivanceras/curtain_gtk/master/screenshot/client_side.png)

## Quick start local dockerize demo

## Requirement:
 - linux/osx
 - cargo and rust installed
 - libssl-dev library installed
 - docker installed

```sh
git clone https://github.com/ivanceras/curtain

cd curtain

sh init_build_docker_demo.sh

```
open browser to http://localhost:8080


## What is working?
Most of read operations of the database.

## What is NOT working?
 - Updating
 - Deleting

* The project is still very young. 




## Inspiration
Design is inspired by Openbravo, iDempiere/Adempiere, Compiere etc.


If you would like to support my project
[Bountysource](https://www.bountysource.com/teams/ivanceras)
[Gratipay](https://gratipay.com/~ivanceras/)
[twitter: @ivanceras](https://twitter.com/ivanceras)

Contact me: ivanceras[at]gmail.com ( I need a co-founder )

