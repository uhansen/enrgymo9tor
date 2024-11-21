use rusqlite::{params, Connection, Result};

// src/crates/dbaccess.rs


#[derive(Debug)]
pub struct Customer {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub address: String,
    pub apikey: String,
}

pub fn add_new(conn: &Connection, customer: &Customer) -> Result<()> {
    conn.execute(
        "INSERT INTO customer (firstname, lastname, email, address, apikey) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![customer.firstname, customer.lastname, customer.email, customer.address, customer.apikey],
    )?;
    Ok(())
}

pub fn update(conn: &Connection, customer: &Customer) -> Result<()> {
    conn.execute(
        "UPDATE customer SET firstname = ?1, lastname = ?2, email = ?3, address = ?4, apikey = ?5 WHERE id = ?6",
        params![customer.firstname, customer.lastname, customer.email, customer.address, customer.apikey, customer.id],
    )?;
    Ok(())
}

pub fn delete(conn: &Connection, id: i32) -> Result<()> {
    conn.execute("DELETE FROM customer WHERE id = ?1", params![id])?;
    Ok(())
}

pub fn get_by_id(conn: &Connection, id: i32) -> Result<Customer> {
    conn.query_row(
        "SELECT id, firstname, lastname, email, address, apikey FROM customer WHERE id = ?1",
        params![id],
        |row| {
            Ok(Customer {
                id: row.get(0)?,
                firstname: row.get(1)?,
                lastname: row.get(2)?,
                email: row.get(3)?,
                address: row.get(4)?,
                apikey: row.get(5)?,
            })
        },
    )
}

pub fn get_all(conn: &Connection) -> Result<Vec<Customer>> {
    let mut stmt = conn.prepare("SELECT id, firstname, lastname, email, address, apikey FROM customer")?;
    let customer_iter = stmt.query_map([], |row| {
        Ok(Customer {
            id: row.get(0)?,
            firstname: row.get(1)?,
            lastname: row.get(2)?,
            email: row.get(3)?,
            address: row.get(4)?,
            apikey: row.get(5)?,
        })
    })?;

    let mut customers = Vec::new();
    for customer in customer_iter {
        customers.push(customer?);
    }
    Ok(customers)
}