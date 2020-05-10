-- Your SQL goes here
CREATE TABLE todo
(
    id int PRIMARY KEY,
    title varchar NOT NULL ,
    description varchar NOT NULL ,
    done boolean NOT NULL
);