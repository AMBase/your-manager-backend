alter table users add column password varchar(255);
update users set password = '';
alter table users alter column password set not null;