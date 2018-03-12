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

    pub fn broadcast_a(&mut self) -> Vec<G2> {
        let mut ret: Vec<G2> = Vec::new();
        for value in &self.polynomial.coef {
            ret.push(G2::one() * *value);
        }
        ret
    }

    pub fn broadcast_s(&mut self, _n: i32) -> Vec<Fr> {
        let mut ret: Vec<Fr> = Vec::new();
        for j in 0.._n {
            // user = i, reciver = j
            // S_{ij} = P_{i}(j) = \sum_{k=0}{t-1}(a_{ik}j^{k})
            let s: String = j.to_string();
            let j_fr: Fr = Fr::from_str(&s).unwrap();
            let mut jk = Fr::one();
            let mut res = Fr::zero();
            for value in &self.polynomial.coef {
                res = res + *value * jk;
                jk = jk * j_fr;
            }
            ret.push(res);
        }
        ret
    }

    pub fn verify(&self, message_pool: &mut ::public::MessagePool) {
        // get the information I should receive
        let received_message = message_pool.get_message(&self);
        // verify each one
        for j in 0..received_message.len() {
            let lhs = G2::one() * received_message[j];
            let mut rhs = G2::zero();

            let s: String = j.to_string();
            let j_fr: Fr = Fr::from_str(&s).unwrap();
            let mut jk: Fr = Fr::one();

            for k in 0..self.polynomial.order {
                rhs = rhs + message_pool.A[self.id as usize][k as usize] * jk;
                jk = jk * j_fr;
            }

            if lhs != rhs {
                message_pool.anti_vote[j as usize].push(self.id);
            }
        }
    }
}
