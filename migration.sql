CREATE TABLE IF NOT EXISTS Customers (
    id INT PRIMARY KEY,
    firstname VARCHAR(50),
    lastname VARCHAR(50),
    email VARCHAR(100),
    address VARCHAR(255),
    apikey VARCHAR(100)
);