-- Your SQL goes here
CREATE TABLE vehicles (
    id VARCHAR(20) PRIMARY KEY,
    name VARCHAR(50) NOT NULL,
    owner VARCHAR(50) NOT NULL,
    motor_type VARCHAR(30) NOT NULL,
    year INT NOT NULL,
    color VARCHAR(20) NOT NULL
);

-- Insert additional data into the Owner table
-- Insert into vehicles table
INSERT INTO vehicles (id, name, year, color, motor_type, owner)
VALUES
    ('ABC123', 'Car1', 2020, 'Red', 'Electric', 'John Doe'),
    ('XYZ456', 'Car2', 2018, 'Blue', 'Gas', 'Jane Doe'),
    ('DEF789', 'Car3', 2019, 'Green', 'Hybrid', 'Bob Smith'),
    ('GHI012', 'Car4', 2021, 'Yellow', 'Diesel', 'Alice Johnson'),
    ('JKL345', 'Car5', 2017, 'Black', 'Electric', 'Charlie Brown'),
    ('MNO678', 'Car6', 2022, 'White', 'Gas', 'Eva White'),
    ('PQR901', 'Car7', 2016, 'Silver', 'Hybrid', 'David Lee'),
    ('STU234', 'Car8', 2023, 'Purple', 'Electric', 'Grace Taylor'),
    ('VWX567', 'Car9', 2015, 'Orange', 'Gas', 'Frank Miller'),
    ('YZA890', 'Car10', 2014, 'Pink', 'Hybrid', 'Helen Davis');
