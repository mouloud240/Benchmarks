# Use PHP CLI with Composer to build and run Laravel
FROM php:8.3-cli-alpine

# Install dependencies and Composer
RUN apk add --no-cache bash git unzip libzip-dev oniguruma-dev icu-dev && \
    docker-php-ext-install intl mbstring zip

# Install Composer
COPY --from=composer:2 /usr/bin/composer /usr/bin/composer

# Create Laravel project
WORKDIR /app
RUN composer create-project --no-dev --prefer-dist laravel/laravel:^11.0 . && \
    php -r "file_put_contents('.env', str_replace('APP_ENV=local', 'APP_ENV=production', file_get_contents('.env')));" && \
    php -r "file_put_contents('.env', str_replace('APP_DEBUG=true', 'APP_DEBUG=false', file_get_contents('.env')));"

# Copy our routes and controller overriding defaults
COPY routes/api.php /app/routes/api.php
COPY app/Http/Controllers/GreetingsController.php /app/app/Http/Controllers/GreetingsController.php

# Optimize autoload and config cache for better performance
RUN php artisan route:cache && \
    php artisan config:cache && \
    php artisan view:cache

# Expose the port and run PHP built-in server
EXPOSE 8080
CMD ["php", "-S", "0.0.0.0:8080", "-t", "public"]
