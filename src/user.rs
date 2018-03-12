use public::Polynomial;
use bn::{Fr, G1, G2, Group};

pub struct Client {
    pub id: i32,
    pub pk: G2,
    sk: Fr,
    polynomial: Polynomial,

}

impl Client {
    pub fn new(_id: i32, _order: i32) -> Client {
        Client {
            id: _id,
            polynomial: Polynomial::new(_order),
            pk: G2::zero(),
            sk: Fr::zero(),
        }
    }

    pub fn broadcast_a(&mut self) -> Vec<G2> {
        let mut ret: Vec<G2> = Vec::new();
        for value in &self.polynomial.coef {
            ret.push(G2::one() * *value);
        }
        ret
    }

    pub fn calc_secret(&mut self, to_usr: i32) -> Fr {
        // from_usr = i, to_usr = j
        // S_{ij} = P_{i}(j) = \sum_{k=0}{t-1}(a_{ik}j^{k})
        let s: String = to_usr.to_string();
        let j_fr: Fr = Fr::from_str(&s).unwrap();
        let mut jk = Fr::one();
        let mut res = Fr::zero();
        for value in &self.polynomial.coef {
            res = res + *value * jk;
            jk = jk * j_fr;
        }
        res
    }

    pub fn broadcast_s(&mut self, _n: i32) -> Vec<Fr> {
        let mut ret: Vec<Fr> = Vec::new();
        for j in 0.._n {
            ret.push(self.calc_secret(j));
        }
        ret
    }

    pub fn verify(&self, message_pool: &mut ::public::MessagePool) {
        // get the information I should receive
        let received_message = message_pool.get_message(&self);
        // verify each one
        let to_usr = self.id;

        for from_usr in 0..received_message.len() {
            let res = self.verify_specific(
                received_message[from_usr],
                from_usr as i32,
                to_usr,
                message_pool,
            );

            if res == false {
                message_pool.veto[from_usr as usize].push(to_usr);
            }
        }
    }

    pub fn verify_specific(
        &self,
        sk: Fr,
        from_usr: i32,
        to_usr: i32,
        message_pool: &::public::MessagePool,
    ) -> bool {
        let lhs = G2::one() * sk;
        let mut rhs = G2::zero();

        let s: String = to_usr.to_string();
        let j_fr: Fr = Fr::from_str(&s).unwrap();
        let mut jk: Fr = Fr::one();

        for k in 0..self.polynomial.order {
            rhs = rhs + message_pool.A[from_usr as usize][k as usize] * jk;
            jk = jk * j_fr;
        }

        lhs == rhs
    }

    pub fn calc_pk_sk(&mut self, message_pool: &mut ::public::MessagePool) {
        self.sk = Fr::zero();
        let received_message = message_pool.get_message(&self);
        for i in message_pool.qual_usr.iter() {
            self.sk = self.sk + received_message[*i as usize];
        }

        self.pk = G2::zero();
        let s: String = self.id.to_string();
        let i_fr: Fr = Fr::from_str(&s).unwrap();
        let mut ik = Fr::one();
        for k in 0..message_pool.t {
            let Ak = message_pool.pk[k as usize];
            self.pk = self.pk + Ak * i_fr;
            ik = ik * i_fr;
        }
    }

    pub fn get_signature(&mut self, hashed_message: &G1) -> G1 {
        *hashed_message * self.sk
    }
}
