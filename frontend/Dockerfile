FROM node:16-alpine as builder
# Set the working directory to /app inside the container
WORKDIR /app
# Copy app files
COPY . .
# Install dependencies (npm ci makes sure the exact versions in the lockfile gets installed)
RUN npm ci 
# Build the app
RUN npm run build

# Bundle static assets with nginx
FROM nginx:1.23.3-alpine-slim as production
ENV NODE_ENV production
# Copy built assets from `builder` image
COPY --from=builder /app/dist /usr/share/nginx/html
# Add your nginx.conf
COPY nginx.conf /etc/nginx/nginx.conf
# Expose port
EXPOSE 80


# Start nginx
CMD ["nginx", "-g", "daemon off;"]