-- create user krabby@localhost identified by 'patty';
-- create database petclinic;
-- grant all privileges on petclinic.* to krabby@localhost;


create table user (
    id INTEGER UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    username varchar(50) unique,
    password varchar(100) not null
) engine innodb;


-- username/password admin
insert into user values (1,'admin', 'd033e22ae348aeb5660fc2140aec35850c4da997');


create table vet(
    id INTEGER UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    name varchar(100)
) engine innodb;

create table pet(
    id INTEGER UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    name varchar(100),
    owner_name varchar(100),
    owner_phone varchar(20),
    age tinyint unsigned,
    pet_type tinyint not null,
    vet_id integer unsigned null,
    created_at datetime,
    created_by integer unsigned not null,
    FOREIGN key (vet_id) REFERENCES vet(id) on delete cascade,
    FOREIGN key (created_by) REFERENCES user(id)   
) engine innodb;

create table visit(
      id INTEGER UNSIGNED AUTO_INCREMENT PRIMARY KEY,
      pet_id integer unsigned not null,
      vet_id integer unsigned not null,
      visit_date datetime not null,
      notes text,
      FOREIGN key (pet_id) REFERENCES pet(id) on delete cascade,
      FOREIGN key (vet_id) REFERENCES vet(id) on delete cascade   
) engine innodb;


insert into vet values(1, "James Carter");
insert into vet values(2, "Helen Leary");
insert into vet values(3, "Linda Douglas");
insert into vet values(4, "Rafael Ortega");

INSERT INTO pet VALUES(1, 'Felix', 'John Doe', '333', 3, 1, 1, '2022-01-01 9:00:00', 1);
INSERT INTO pet VALUES(2, 'Chloe', 'Peter Falk', '333', 5, 2, 1, '2022-01-01 9:00:00', 1);
INSERT INTO pet VALUES(3, 'Iru', 'Dr.Falken', '333', 8, 2, 3, '2022-01-01 9:00:00', 1);
INSERT INTO pet VALUES(4, 'Willy', 'Harold Davis', '333', 10, 2, null, '2022-01-01 9:00:00', 1);
