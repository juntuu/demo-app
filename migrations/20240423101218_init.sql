/*
Note:
Some of the fields could use more appropriate data type, but text is fine for
this demo. Especially some ids could be switched to numeric ones, making some
id updates, and thus `on update cascade` clauses, redundant.
*/

/* Primary data */

create table if not exists user (
	username text not null primary key,
	email text not null unique,
	password text not null,
	bio text null,
	image text null
);

create table if not exists article (
	slug text not null primary key,
	title text not null,
	description text not null,
	body text not null,
	created_at text NOT NULL default (date('now')),
	updated_at text null,

	author text not null references user(username) on delete cascade on update cascade
);

create table if not exists comment (
	id integer primary key,
	body text NOT NULL,
	created_at text NOT NULL default (date('now')),

	article text not null references article(slug) on delete cascade on update cascade,
	user text not null references user(username) on delete cascade on update cascade
);

/* Relations */

create table if not exists tag (
	tag text not null,
	article text not null references article(slug) on delete cascade on update cascade,
	primary key (tag, article)
);


create table if not exists follow (
	follower text not null references user(username) on delete cascade on update cascade,
	followed text not null references user(username) on delete cascade on update cascade,
	primary key (follower, followed)
);

create table if not exists favorite (
	user text not null references user(username) on delete cascade on update cascade,
	article text not null references article(slug) on delete cascade on update cascade,
	primary key (user, article)
);
