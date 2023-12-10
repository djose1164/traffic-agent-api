-- Your SQL goes here
CREATE TABLE drivers (
    id INT PRIMARY KEY AUTO_INCREMENT,
    name VARCHAR(64) NOT NULL,
    birth_date DATE NOT NULL,
    address VARCHAR(100) NOT NULL,
    phone_number varchar(20) NOT NULL UNIQUE,
    picture varchar(256) NOT NULL
);

INSERT INTO drivers (name, birth_date, address, phone_number, picture)
VALUES
    ('Donald Trump', '1946-06-14', 'Miami, USA', '123-1234-1234', 'http://127.0.0.1:8080/assets/donald_trump.jpg'),
    ('Elon Musk', '1971-06-28', 'Texas, USA', '012-0123-0123', 'http://127.0.0.1:8080/assets/elon_musk.jpg');