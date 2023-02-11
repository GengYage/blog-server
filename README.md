# blog-server
### 创建表语句
```sql
create sequence
    blog_articles_seq
    increment 1
    minvalue 1
    maxvalue 9223372036854775807
    start with 1
    cache 1;

create table if not exists articles
(
    id          bigint primary key default nextval('blog_articles_seq'::regclass),
    title       varchar(255),
    content     text,
    create_time timestamp          default now(),
    update_time timestamp          default now()
);

create or replace function blog_article_update_timestamp() returns trigger as
$$
begin
    new.update_time = now();
    return new;
end
$$
    language plpgsql;

create trigger blog_article_update_timestamp
    before update
    on articles
    for each row
execute procedure blog_article_update_timestamp();

insert into articles (title, content) values ('Hello World', '测试数据');
```
### api

```
GET     /api/rest/articles/v1       查询所有文章
POST    /api/rest/article/add/v1    添加文章
DELETE  /api/rest/article/delete/v1 删除文章
POST    /api/rest/article/update/v1 更新文章
GET     /api/rest/article/search/v1 搜索文章(搜索标题和内容)
GET     /api/rest/article/get/v1    通过id查询单个文章
```
