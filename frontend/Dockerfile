
FROM node:18-alpine AS builder
WORKDIR /usr/src/app

COPY package*.json ./
RUN npm ci
COPY . .
RUN npm run build

FROM nginx:stable-alpine AS runtime
WORKDIR /usr/share/nginx/html

COPY --from=builder /usr/src/app/dist .

EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
