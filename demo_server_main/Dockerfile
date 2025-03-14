FROM alpine:3.18
ARG target
ARG name
# RUN sed -i 's/dl-cdn.alpinelinux.org/mirrors.ustc.edu.cn/g' /etc/apk/repositories
# RUN apk update && apk add tzdata
# RUN ln /usr/share/zoneinfo/Asia/Shanghai /etc/localtime
# RUN echo "Asia/Shanghai" >/etc/timezone
WORKDIR /app
COPY ./target/${target}/release/${name} application
COPY .env .
COPY static static
CMD ["/app/application"]
