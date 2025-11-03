# Use PHP CLI with Composer to build and run Laravel
FROM php:8.3-cli-alpine

# Install dependencies and Composer
RUN apk add --no-cache bash git unzip libzip-dev oniguruma-dev icu-dev && \
    docker-php-ext-install intl mbstring zip

# Install Composer
COPY --from=composer:2 /usr/bin/composer /usr/bin/composer

# Set working directory
WORKDIR /app

# Copy composer files first for better layer caching
COPY composer.json composer.lock ./

# Install PHP dependencies (production only)
RUN composer install --no-dev --optimize-autoloader --no-interaction --no-scripts

# Copy the entire Laravel application
COPY . .

# Set up environment file for production
RUN cp .env.example .env && \
    php artisan key:generate && \
    php -r "file_put_contents('.env', str_replace('APP_ENV=local', 'APP_ENV=production', file_get_contents('.env')));" && \
    php -r "file_put_contents('.env', str_replace('APP_DEBUG=true', 'APP_DEBUG=false', file_get_contents('.env')));"

# Create SQLite database file
RUN touch database/database.sqlite

# Run migrations
RUN php artisan migrate --force

# Optimize for production
RUN php artisan config:cache && \
    php artisan route:cache && \
    php artisan view:cache

# Expose the port and run PHP built-in server
EXPOSE 8080
CMD ["php", "-S", "0.0.0.0:8080", "-t", "public"]
