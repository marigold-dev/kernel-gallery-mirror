# Stage 1: Build the Vite app
FROM node:16-alpine AS build

WORKDIR /app

# Copy package.json and package-lock.json
COPY package*.json ./

# Install dependencies
RUN npm install

# Copy local code to the container
COPY . .

# Build the app
RUN npm run build

# Stage 2: Serve the Vite app
FROM node:16-alpine

WORKDIR /app

# Install the 'serve' package for serving our static files
RUN npm install -g serve

# Copy the build output from the first stage
COPY --from=build /app/dist /app/dist

# Specify the command to run on container start
CMD ["serve", "-s", "dist", "-l", "3001"]
