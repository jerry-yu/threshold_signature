use ::public::Polynomial;
use ::bn::{Fr, G1, G2, Group};

pub struct Client {
    id: i32,
    polynomial: Polynomial,
}

impl Client {
    pub fn new(_id: i32, _order: i32) -> Client {
        Client { id: _id, polynomial: Polynomial::new(_order)}
    }

    pub fn broadcast(&mut self) -> Vec<G2> {
        let mut ret: Vec<G2> = Vec::new();
        for value in &self.polynomial.coef {
            ret.push(G2::one() * *value);
        }
        ret
    }
}
