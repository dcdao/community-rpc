FROM nginx:1.24-alpine

COPY .maintain/docker/nginx/default.conf /etc/nginx/conf.d
COPY analysis/dashboard/dist  /usr/share/nginx/html

