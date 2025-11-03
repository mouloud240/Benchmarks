# Dockerfile

# Stage 1: Build the application
FROM node:20-alpine AS builder
WORKDIR /usr/src/app

# Copy package files and install dependencies
COPY package.json pnpm-lock.yaml ./
RUN npm install -g pnpm
RUN pnpm install

# Copy the rest of the application source code
COPY . .

# Build the application
RUN pnpm build

# Stage 2: Create the production image
FROM node:20-alpine
WORKDIR /usr/src/app

# Copy package files and install production dependencies
COPY package.json pnpm-lock.yaml ./
RUN npm install -g pnpm
RUN pnpm install --prod

# Copy the built application from the builder stage
COPY --from=builder /usr/src/app/dist ./dist

# Expose the port the app runs on
EXPOSE 3003

# Command to run the application
CMD ["pnpm", "start:prod"]
