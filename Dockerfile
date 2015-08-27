FROM ubuntu:14.04

ENV HOME /home/curtain/


ENV DATABASE_URL postgres://postgres:p0stgr3s@45.55.7.231:5432/bazaar_v6

ENV PORT 8080

WORKDIR /home/curtain/

ADD target/release/iron_curtain /home/curtain/

EXPOSE 8080

CMD ["/home/curtain/iron_curtain"]


# upload to digital ocean
# rsync -avzhP curtain root@45.55.7.231:~/