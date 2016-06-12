#Curtain

[![Build Status](https://api.travis-ci.org/ivanceras/curtain.svg)](https://travis-ci.org/ivanceras/curtain)

A data-only administration tool for postgresql, which understand relationship between tables through their foreign keys and display related data to the data in context.

Inspired by: Application Dictionary of Compiere, Adempiere, Openbravo.

## Vision
 - A data application that is easy to use even for non-programmers. This will be achieve by not using terms such as `primary keys`, `foreign keys` which are not easily understood by non programmers.
 - This will be achieve by inferring relationships between data


## Clients
There are Work in progress clients 

1. [curtain_ui](https://github.com/ivanceras/curtain_ui) - javascript implementation
   ![](https://raw.githubusercontent.com/ivanceras/curtain_ui/master/screenshots/curtain_ui.png)

   [Demo](http://curtain-ui.herokuapp.com/?/new) - Please don't flood the servers.


2. [curtain_gtk](https://github.com/ivanceras/curtain_gtk) - gtk client (WIP) Doing [gtk-rs](https://github.com/gtk-rs/gtk) UI is not very ergonomic, and the projects is not yet matured.
    ![](https://raw.githubusercontent.com/ivanceras/curtain_gtk/master/screenshot/client_side.png)


3. [curtain_elm](https://github.com/ivanceras/curtain-elm) - Early progress using elm implementation. Despite using simplistic framework, in javascript implementation, it gets realy hard to quickly grasp the original intent of the code that were previously written. As a programmer who jumps from multiple languages and frameworks, I had no problem jumping in between multiple rust projects. This is a different case for javascript projects. I'm giving elm a shot on this.
4. 
   ![](https://raw.githubusercontent.com/ivanceras/curtain-elm/master/screenshot.png)

## Quick start installation

Checkout the code and set a compatible nightly build. nightly-2015-12-26 is known to work.


```sh

git clone https://github.com/ivanceras/curtain
cd curtain
multirust override nightly-2015-12-26
cargo run --release

```

Curtain opens port [8181](https://github.com/ivanceras/curtain/blob/master/src/main.rs#L83) and the client is configured to use this port.

```sh

git clone https://github.com/ivanceras/curtain_ui
cd curtain_ui
python -m SimpleHttpServer

```
Python HttpServer opens on port 8000

Navigate your browser to http://localhost:8000

Click on `Connect to server` and supply the db_url of your database.
db_url format is `postgres://user:password@host:port/db`

You should be able to view your data on your database.
If you want a sample data you can use the data provided by the sample [bazaar](https://github.com/ivanceras/bazaar).

```sh

git clone https://github.com/ivanceras/bazaar
cd bazaar/scripts
./setup.sh

```

### Support

Curtain is part of my pet projects that I have been working on my spare time.
[Rustorm](https://github.com/ivanceras/rustorm) is an ORM and serves as the core library on most of my projects.


[Bountysource](https://www.bountysource.com/teams/ivanceras)

[Gratipay](https://gratipay.com/~ivanceras/)

[twitter: @ivanceras](https://twitter.com/ivanceras)
