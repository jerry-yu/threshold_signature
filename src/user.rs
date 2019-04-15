use bn::{pairing, Fr, Group, G1, G2};
use public::MessagePool;
use rand::Rng;

pub struct Polynomial {
    pub order: usize,
    pub coef: Vec<Fr>,
}

impl Polynomial {
    pub fn new(order: usize) -> Polynomial {
        Polynomial {
            order: order,
            coef: Polynomial::coef_gen(order),
        }
    }

    pub fn coef_gen(order: usize) -> Vec<Fr> {
        let mut ret = Vec::new();
        let rng = &mut ::rand::thread_rng();
        for _ in 0..order {
            //ret.push(Fr::random(rng));
            ret.push(Fr::one());
        }
        ret
    }
}

pub struct Client {
    pub id: i32,
    pub pk: Option<G2>,
    pub mpk: Option<G2>,
    sk: Option<Fr>,
    polynomial: Polynomial,
    msg_pool: MessagePool,
}

impl Client {
    pub fn new(id: i32, n: usize, t: usize) -> Option<Client> {
        let mut real_id = id;
        if id == 0 {
            let rng = &mut ::rand::thread_rng();
            real_id = rng.gen_range(1, ::std::i32::MAX);
        }
        if n < t || t == 0 || n == 0 {
            return None;
        }
        let mut new_cli = Client {
            id: real_id,
            polynomial: Polynomial::new(t),
            mpk: None,
            pk: None,
            sk: None,
            msg_pool: MessagePool::new(n, t, real_id),
        };
        let coef = new_cli.calc_coef_g2();
        let sec = new_cli.calc_secret(real_id);
        new_cli.set_client_id_coefs(real_id, &coef);
        new_cli.set_client_id_secrets(real_id, sec);
        Some(new_cli)
    }

    pub fn set_client_id_coefs(&mut self, src_id: i32, coefs: &Vec<G2>) -> bool {
        self.msg_pool.set_client_id_coefs(src_id, coefs)
    }

    pub fn set_client_id_secrets(&mut self, src_id: i32, secret: Fr) -> bool {
        self.msg_pool.set_client_id_secrets(src_id, secret)
    }

    pub fn calc_coef_g2(&self) -> Vec<G2> {
        let mut ret: Vec<G2> = Vec::new();
        for value in &self.polynomial.coef {
            ret.push(G2::one() * *value);
        }
        ret
    }

    pub fn calc_secret(&self, to_usr: i32) -> Fr {
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

    pub fn verify(&self, src_id: i32) -> bool {
        self.msg_pool.verify(src_id)
    }

    pub fn get_qual_usr(&mut self) -> Vec<i32> {
        self.msg_pool.get_qual_usr()
    }

    pub fn calc_pk_sk(&mut self) {
        if self.msg_pool.calc_whole_coefs() {
            self.sk = self.msg_pool.calc_sk();
            self.pk = self.msg_pool.calc_pk(self.id);
            self.mpk = self.msg_pool.cal_mpk();
        }
    }

    pub fn calc_signature(&self, hashed_message: &G1) -> Option<G1> {
        self.sk.and_then(|sk| Some(*hashed_message * sk))
    }

    pub fn verify_single_signature(&self, signer_id: i32, hashed_message: &G1, sig: &G1) -> bool {
        if let Some(pk) = self.msg_pool.calc_pk(signer_id) {
            return pairing(*hashed_message, pk) == pairing(*sig, G2::one());
        }
        false
    }

    pub fn verify_completed_signature(&self, hmsg: &G1, sig: &G1) -> bool {
        self.mpk.map_or_else(
            || false,
            |mpk| pairing(*sig, G2::one()) == pairing(*hmsg, mpk),
        )
    }

    pub fn calc_lambda(st: usize, ed: usize, exc: usize, ids: &Vec<i32>) -> Fr {
        let mut up = Fr::one();
        let mut down = Fr::one();

        let i = ids[exc];
        for k in st..ed {
            if k == exc {
                continue;
            }
            let j = ids[k];
            // up
            let mut res = 0 - j;
            if res >= 0 {
                let s: String = res.to_string();
                let num = Fr::from_str(&s).unwrap();
                up = up * num;
            } else {
                let s: String = (-res).to_string();
                let num = -Fr::from_str(&s).unwrap();
                up = up * num;
            }

            res = i - j;
            if res >= 0 {
                let s: String = res.to_string();
                let num = Fr::from_str(&s).unwrap();
                down = down * num;
            } else {
                let s: String = (-res).to_string();
                let num = -Fr::from_str(&s).unwrap();
                down = down * num;
            }
        }
        up * down.inverse().unwrap()
    }
}
