# Stage 1: Build the React app
FROM rust:latest AS build

# Install Node.js and npm
RUN curl -fsSL https://deb.nodesource.com/setup_18.x | bash -
RUN apt-get install -y nodejs

# Install wasm-pack
RUN cargo install wasm-pack

# Set the working directory
WORKDIR /app

# Copy the package.json and install dependencies
COPY package.json ./
RUN npm install

# Copy the rest of the application files
COPY . .

# Build the WASM package
RUN npm run wasm

# Build the React app
RUN npm run build

# Stage 2: Set up NGINX to serve the React app
FROM nginx:alpine

# Copy the built files from the previous stage
COPY --from=build /app/public /usr/share/nginx/html

# Copy the custom NGINX configuration file
COPY nginx.conf /etc/nginx/nginx.conf

# Expose the port the app runs on
EXPOSE 80

# Start NGINX
CMD ["nginx", "-g", "daemon off;"]
