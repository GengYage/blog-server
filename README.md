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
    create_time date               default current_date,
    update_time date               default current_date
);


create or replace function blog_article_update_timestamp() returns trigger as
$$
begin
    new.update_time = current_date;
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
GET /api/rest/articles/v1
POST /api/rest/article/add/v1
```
