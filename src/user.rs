use ::public::Polynomial;
use ::bn::{Fr, G1, G2, Group};

pub struct Client {
    pub id: i32,
    polynomial: Polynomial,
}

impl Client {
    pub fn new(_id: i32, _order: i32) -> Client {
        Client { id: _id, polynomial: Polynomial::new(_order)}
    }

    pub fn broadcastA(&mut self) -> Vec<G2> {
        let mut ret: Vec<G2> = Vec::new();
        for value in &self.polynomial.coef {
            ret.push(G2::one() * *value);
        }
        ret
    }

    pub fn broadcastS(&mut self, _n: i32) -> Vec<Fr> {
        let mut ret: Vec<Fr> = Vec::new();
        for j in 0.._n {
            let s: String = j.to_string();
            let j_fr: Fr = Fr::from_str(&s).unwrap();
            let mut rhs = Fr::one();
            let mut res = Fr::zero();
            for value in &self.polynomial.coef {
                res = res + *value * rhs;
                rhs = rhs * j_fr;
            }
            ret.push(res);
        }
        ret
    }

    // pub fn verify(&self, message_pool: &::public::MessagePool) {
    //     // get the information I should receive
    //     let received_message = message_pool.get_message(&self);
    //     // verify each one
    //     for message in &received_message {
    //     }
    // }
}
