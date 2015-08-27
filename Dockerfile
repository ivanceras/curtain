FROM ubuntu:14.04

ENV HOME /root



WORKDIR /root

ADD target/release/iron_curtain /root/

EXPOSE 8080

CMD ["/root/iron_curtain"]