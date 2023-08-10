FROM node:18-alpine

WORKDIR /app

COPY package*.json yarn.lock ./

COPY . .

RUN rm -rf node_modules build
RUN npm install -g npm@9.6.7
RUN npm install

RUN npm run build

RUN npm install -g serve

CMD ["serve", "-l", "3001", "-n", "-s", "build"]
