-- create table users (
--     id serial primary key,
--     username varchar not null unique,
--     hash varchar(122) not null,
--     first_name varchar(50) not null,
--     last_name varchar(50) not null,
--     role varchar(50) not null
-- );
-- insert into users(username, hash, first_name, last_name, role) values ('vinson3', '$argon2id$v=19$m=4096,t=192,p=4$NEWWvndJryPwSgvsuF23RCDcLclz1uZLN3bSqy1SF0$kFfrW7jHZfGU0rxSscBIjAbHND45bViJYdXlrCzu7T4', 'vinson3', 'wong3', 'ADMIN');

create table "User" (
    "Id" SERIAL,
    "Username" VARCHAR(50) NOT NULL UNIQUE,
    "password" VARCHAR(255) NOT NULL,
    "Firstname" VARCHAR(50) NOT NULL,
    "Lastname" VARCHAR(50) NOT NULL,
    "IsActive" BOOLEAN DEFAULT 't' NOT NULL,
    "IsSuper" BOOLEAN DEFAULT 'f' NOT NULL,
    "CreatedAt" timestamp DEFAULT NOW() NOT NULL,
    "UpdatedAt" timestamp DEFAULT NOW() NOT NULL,
    PRIMARY KEY ("Id")
);

create table "Role" (
  "Id" SERIAL,
  "Code" VARCHAR(50) NOT NULL UNIQUE,
  "Name" VARCHAR(100) NOT NULL UNIQUE,
  "CreatedAt" timestamp DEFAULT NOW() NOT NULL,
  "UpdatedAt" timestamp DEFAULT NOW() NOT NULL,
  PRIMARY KEY ("Id")
);

create table "UserRole"(
  "Id" SERIAL,
  "UserId" SERIAL,
  "RoleId" SERIAL,
  PRIMARY KEY ("Id"),
  FOREIGN KEY ("UserId") REFERENCES "User"("Id"),
  FOREIGN KEY ("RoleId") REFERENCES "Role"("Id")
)