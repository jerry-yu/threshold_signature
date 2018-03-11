use ::public::Polynomial;

struct Client {
    polynomial: Polynomial,
}

impl Client {
    fn new(order: i32) -> Client {
        Client { polynomial: Polynomial::new(order)}
    }
}


