# STAGE 1
FROM node:18-alpine as builder
RUN mkdir -p /home/node/app/node_modules && chown -R node:node /home/node/app
WORKDIR /home/node/app
COPY package*.json ./
RUN npm install -g typescript
RUN npm install -g ts-node
RUN npm install
COPY --chown=node:node . .
RUN npm run build

# STAGE 2
FROM node:18-alpine
RUN mkdir -p /home/node/app/node_modules && chown -R node:node /home/node/app
WORKDIR /home/node/app
COPY package*.json ./
# RUN npm install --save-dev sequelize-cli
RUN npm install --production
COPY --from=builder /home/node/app/dist ./dist

EXPOSE 8080
CMD [ "node", "dist/index.js" ]
