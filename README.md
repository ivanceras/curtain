# Curtain

[![Build Status](https://api.travis-ci.org/ivanceras/curtain.svg)](https://travis-ci.org/ivanceras/curtain)

A smart data editing tool for non-programmers.

Long:

The UI is created using the meta data information of tables and their foreign key relationships. If a certain record in a table refer to a foreign record in some other table, instead of displaying the foreign key, the recognizable record is displayed.

The project even goes farther by displaying the pertaining details of record in focused.
If you click on a certain product, the list of its category, orders, reviews, photos will also be implicitly displayed as its detail.
 

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
 1. [Curtain-elm](https://github.com/ivanceras/curtain-elm) - early progress in elm implementation
    ![](https://raw.githubusercontent.com/ivanceras/curtain-elm/master/screenshot.png)

 2. [curtain_ui](https://github.com/ivanceras/curtain_ui) - plain javascript implementation using mithril and material design
   ![](https://raw.githubusercontent.com/ivanceras/curtain_ui/master/screenshots/curtain_ui.png)

 3. [curtain_gtk](https://github.com/ivanceras/curtain_gtk) - gtk client (WIP) Doing [gtk-rs](https://github.com/gtk-rs/gtk) UI is not very ergonomic, and the project is not yet matured.
    ![](https://raw.githubusercontent.com/ivanceras/curtain_gtk/master/screenshot/client_side.png)

## Quick start local dockerize demo

## Requirement:
 - linux/osx
 - cargo and rust installed
 - libssl-dev installed

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

