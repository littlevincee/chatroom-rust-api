FROM node:14.15.1-alpine
WORKDIR ./apollo-server
RUN apk add --no-cache bash
COPY package.json gateway.js wait-for-it.sh ./
RUN npm install
RUN chmod +x wait-for-it.sh
# wait until all downstream services start
CMD ./wait-for-it.sh auth-service:8003 -s -- \
    npm run start-gateway
EXPOSE 4000



# FROM node:14.15.1-alpine
# WORKDIR ./apollo-server
# RUN apk add --no-cache bash
# COPY package.json gateway.js ./
# RUN npm install
# EXPOSE 4000

# ## THE LIFE SAVER
# ADD https://github.com/ufoscout/docker-compose-wait/releases/download/2.2.1/wait /wait
# RUN chmod +x /wait

# # wait until all downstream services start
# CMD /wait && npm run start-gateway