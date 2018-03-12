pub mod utility {
    use bn::Fr;

    pub fn coef_gen(order: i32) -> Vec<Fr> {
        let mut ret = Vec::new();
        let rng = &mut ::rand::thread_rng();
        for _ in 0..order {
            ret.push(Fr::random(rng));
        }
        ret
    }

    #[test]
    fn test_coef_gen() {
        let coef = coef_gen(10);
        assert_eq!(coef.len(), 10 as usize);
    }
}

use bn::{Fr, G1, G2, Group, pairing};

pub struct Polynomial {
    pub order: i32,
    pub coef: Vec<Fr>,
}

impl Polynomial {
    pub fn new(_order: i32) -> Polynomial {
        Polynomial {
            order: _order,
            coef: utility::coef_gen(_order),
        }
    }
}

pub struct MessagePool {
    // A means the broadcast value A_{ik} = g_2^{a_{ik}}
    pub A: Vec<Vec<G2>>,
    pub veto: Vec<Vec<i32>>,
    pub qual_usr: Vec<i32>,
    pub pk: Vec<G2>,
    S: Vec<Vec<Fr>>,
    pub n: i32,
    pub t: i32,
}

impl MessagePool {
    pub fn new(clients: &mut Vec<::user::Client>, _n: i32, _t: i32) -> MessagePool {
        let mut _a: Vec<Vec<G2>> = Vec::new();
        let mut _s: Vec<Vec<Fr>> = Vec::new();
        let mut _veto: Vec<Vec<i32>> = Vec::new();

        for client in clients {
            _a.push(client.broadcast_a());
            _s.push(client.broadcast_s(_n));
            _veto.push(Vec::new());
        }

        MessagePool {
            A: _a,
            S: _s,
            veto: _veto,
            qual_usr: Vec::new(),
            pk: Vec::new(),
            n: _n, t: _t,
        }
    }

    pub fn get_message(&self, client: &::user::Client) -> Vec<Fr> {
        let mut ret = Vec::new();
        for message_list in &self.S {
            ret.push(message_list[client.id as usize]);
        }
        ret
    }

    pub fn get_qual_usr(&mut self, clients: &mut Vec<::user::Client>) {
        for to_usr in 0..self.veto.len() {
            if self.veto[to_usr].len() == 0 as usize {
                self.qual_usr.push(to_usr as i32);
            } else {
                for from_usr in self.veto[to_usr].iter() {
                    let sk = clients[*from_usr as usize].calc_secret(to_usr as i32);
                    let mut res = true;
                    for _client in clients.iter() {
                        if _client.verify_specific(sk, *from_usr as i32, to_usr as i32, self)
                            == false
                        {
                            res = false;
                            break;
                        }
                    }

                    if res == true {
                        self.qual_usr.push(to_usr as i32);
                    }
                }
            }
        }

        // calc the public value
        for k in 0..self.t {
            let mut ret = G2::zero();
            for i in 0..self.qual_usr.len() {
                ret = ret + self.A[i as usize][k as usize];
            }
            self.pk.push(ret);
        }

        // calc public key and secret key for clients
    }

    pub fn get_signature(&mut self, hashed_message: &G1, clients: &mut Vec<::user::Client>) {
        let mut signatures: Vec<G1> = Vec::new();
        for i in self.qual_usr.iter() {
            let sig = clients[*i as usize].get_signature(hashed_message);
            // verification
            let lhs = pairing(sig, G2::one());
            let rhs = pairing(*hashed_message, clients[*i as usize].pk);
            assert!(lhs == rhs);

            signatures.push(sig);
        }
        // let mut lhs =
        // for i in 0..t {

        // }
    }
}
