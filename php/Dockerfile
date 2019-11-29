FROM php:7.4-zts-alpine

RUN apk add --no-cache --virtual .persistent-deps libffi-dev \
    && docker-php-ext-configure ffi --with-ffi \
    && docker-php-ext-install ffi

RUN apk add --no-cache gcompat libgcc
