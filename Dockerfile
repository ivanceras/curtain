FROM ubuntu:14.04

ENV HOME /home/curtain/


ENV DATABASE_URL postgres://postgres:p0stgr3s@45.55.7.231:5432/bazaar_v7

ENV PORT 3224

WORKDIR /home/curtain/

ADD target/release/iron_curtain /home/curtain/

EXPOSE 3224

CMD ["/home/curtain/iron_curtain"]


