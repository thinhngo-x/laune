# Frontend Dockerfile
FROM node:24.4.1-alpine as builder

WORKDIR /app

# Install pnpm
RUN npm install -g pnpm

# Copy package files
COPY package.json pnpm-lock.yaml ./

# Clear any existing node_modules and install dependencies with caching
RUN rm -rf node_modules && pnpm install --frozen-lockfile

# Copy source code
COPY . .

# Build the application
RUN rm -rf node_modules && pnpm add typescript && pnpm build

# Runtime stage with Nginx
FROM nginx:alpine

# Install curl for health check
RUN apk add --no-cache curl

# Copy built files to nginx
COPY --from=builder /app/dist /usr/share/nginx/html

# Copy nginx configuration
COPY nginx.conf /etc/nginx/conf.d/default.conf

# Add health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost/ || exit 1

CMD ["nginx", "-g", "daemon off;"]
