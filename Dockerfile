FROM ubuntu:14.04

## Install postgresql

RUN apt-get update

RUN apt-get install -y --force-yes postgresql-9.3 postgresql-client-9.3 postgresql-contrib-9.3

ADD build /home/build


USER postgres

RUN echo "host all  all    0.0.0.0/0  md5" >> /etc/postgresql/9.3/main/pg_hba.conf

RUN echo "listen_addresses='*'" >> /etc/postgresql/9.3/main/postgresql.conf



RUN service postgresql start &&\

    psql -f /home/build/setup_postgres_user.sql
 

USER root


ADD build/.pgpass /root/

RUN chmod 0600 /root/.pgpass

ADD target/release/iron_curtain /home/curtain/


## Import Data

WORKDIR /home/build/mockdata

RUN service postgresql start && sh reimport.sh

# For curtain-elm UI
# Install Nginx
RUN \
  apt-get update && \
  apt-get install -y nginx && \
  rm -rf /var/lib/apt/lists/* && \
  echo "\ndaemon off;" >> /etc/nginx/nginx.conf && \
  chown -R www-data:www-data /var/lib/nginx

RUN rm /etc/nginx/sites-enabled/default

RUN cp /home/build/curtain-elm/build/curtain-elm.conf /etc/nginx/sites-available/

RUN ln -s /etc/nginx/sites-available/curtain-elm.conf /etc/nginx/sites-enabled/curtain-elm.conf

RUN chmod 755 -R /home/build/curtain-elm/build/

ADD run.sh /home/

RUN chmod u+x /home/run.sh

EXPOSE 80 5432 3224

CMD /home/run.sh
