/* 
 docker run --name postgres-db -e POSTGRES_PASSWORD=docker -p 5432:5432 -d postgres
*/

/* 
    Create a table -> A table represents an object
*/
CREATE TABLE IF NOT EXISTS questions (
    id serial PRIMARY KEY,
    title VARCHAR (255) NOT NULL,
    content TEXT NOT NULL,
    tags TEXT [],
    created_on TIMESTAMP NOT NULL DEFAULT NOW()
);


echo DATABASE_URL=postgres://postgres:docker@localhost/postgres > .env