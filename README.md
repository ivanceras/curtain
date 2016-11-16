# Curtain

[![Build Status](https://api.travis-ci.org/ivanceras/curtain.svg)](https://travis-ci.org/ivanceras/curtain)

## Warning: Early Alpha Status, use in production at your own risk

Curtain is a data management system for PostgreSQL using the best of current technology.

![](https://raw.githubusercontent.com/ivanceras/curtain/master/screenshots/users.png)

##[Alpha builds](https://github.com/ivanceras/curtain-releases)
### Supported platform
- [x] Linux
- [x] Mac OSX
- [x] PostgreSQL

## Screencast
[link1](https://gfycat.com/GlossyDownrightBorderterrier) [link2](http://s1.webmshare.com/BXZxE.webm)


## Goal
- A user friendly data manipulation tool even to non-technical users.

## Features
- Intuitive and modern user interface
    - Load on demand, infinite scrolling
    - Fast and interactive filtering

#### Load on demand, infinite scrolling
Viewing a table will eagerly load only the first page, scrolling to the bottom of the page
loads the next records, up until the last page

#### Fast and interactive filtering
Filter records as you type in text into their corresponding columns and displays only the matching records.



## Online Demo

[Herokuapp Demo](http://curtain-elm.herokuapp.com)

[DigitalOcean Demo](http://45.55.7.231:8080/)

Data and schema of the database used in the demo can be found here [mockdata](https://github.com/ivanceras/mockdata) which was generated using mockaroo.com


## Platforms
   - rust
   - postgresql
   - elm
  
## Quick start local dockerize demo

## Requirement:
 - linux/osx
 - cargo and rust installed
 - libssl-dev library installed (openssl issue stuff)
 - docker

```sh
git clone https://github.com/ivanceras/curtain

cd curtain

sh init_build_docker_demo.sh

```
open browser to http://localhost:8080



## Inspiration
Design is inspired by Openbravo, iDempiere/Adempiere, Compiere etc.


If you would like to support my project
- [Bountysource](https://www.bountysource.com/teams/ivanceras)
- [Gratipay](https://gratipay.com/~ivanceras/)
- [twitter: @ivanceras](https://twitter.com/ivanceras)

Contact me: ivanceras[at]gmail.com

