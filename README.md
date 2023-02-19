# blog-server
### 创建表语句
```sql
-- 创建自增主键序列
create sequence
    blog_articles_seq
    increment 1
    minvalue 1
    maxvalue 9223372036854775807
    start with 1
    cache 1;

-- 创建文章表
create table if not exists articles
(
    id          bigint primary key default nextval('blog_articles_seq'::regclass),
    title       varchar(255),
    content     text,
    create_time timestamp with time zone default now(),
    update_time timestamp with time zone default now()
);

-- 创建更新update_time的函数
create or replace function blog_article_update_timestamp() returns trigger as
$$
begin
    new.update_time = now();
    return new;
end
$$
    language plpgsql;

-- 添加触发器
create trigger blog_article_update_timestamp
    before update
    on articles
    for each row
execute procedure blog_article_update_timestamp();

-- 插入测试数据
insert into articles (title, content) values ('Hello World', '测试数据');

-- 创建用户表
create table if not exists users
(
    id          bigint primary key,
    name        varchar(255),
    avatar_url  varchar(255),
    url         varchar(255),
    html_url    varchar(255),
    create_time timestamp with time zone default now(),
    update_time timestamp with time zone default now()
);

-- 设置更新时间的触发器
create trigger blog_user_update_timestamp
    before update
    on users
    for each row
execute procedure blog_article_update_timestamp();

-- 创建评论自增序列
create sequence
    blog_comments_seq
    increment 1
    minvalue 1
    maxvalue 9223372036854775807
    start with 1
    cache 1;

-- 创建评论表
create table if not exists comments
(
    id          bigint primary key       default nextval('blog_comments_seq'::regclass),
    article_id  bigint not null,
    user_id     bigint not null,
    p_id        bigint null,
    content     text,
    create_time timestamp with time zone default now()
);

```
### api

| 方法   | API                           | 权限 | 描述              |
| ------ | ----------------------------- | ---- | ----------------- |
| GET    | /api/rest/articles/v1         | User | 查询所有文章      |
| POST   | /api/rest/article/add/v1      | User | 增加文章          |
| DELETE | /api/rest/article/delete/v1   | User | 删除文章          |
| POST   | /api/rest/article/update/v1   | User | 更新文章          |
| GET    | /api/rest/article/search/v1   | None | 查询文章          |
| GET    | /api/rest/article/get/v1      | None | 根据id查询文章    |
| POST   | /api/rest/auth/login/v1       | None | 登陆oauth(github) |
| POST   | /api/rest/comment/add/v1      | User | 增加评论          |
| GET    | /api/rest/article/comments/v1 | None | 查询文章所有评论  |
| DELETE | /api/rest/comment/delete/v1   | User | 删除评论          |

### 登陆方式
```
<a href="https://github.com/login/oauth/authorize?client_id={your client id}">登陆</a>
```
### 环境变量
```
DATABASE_URL=postgres://localhost:5432/yage
CLIENT_ID=
CLIENT_SECRET=
ADMIN_ID=
```