use bn::{Fr, G2, Group};
use std::collections::BTreeMap;

pub struct MessagePool {
    // user_id_coefs means the broadcast value A_{ik} = g_2^{a_{ik}}
    pub userid_coefs_g2: BTreeMap<i32, Vec<G2>>,

    /*comment when network comunication succ,to be continue*/
    //pub veto: BTreeMap<i32,Vec<i32>>,
    pub qual_usr: Vec<i32>,
    pub whole_coefs: Vec<G2>,
    userid_poly_secrets: BTreeMap<i32, Fr>,
    my_id: i32,
    pub n: usize,
    pub t: usize,
}

impl MessagePool {
    pub fn new(n: usize, t: usize, my_id: i32) -> MessagePool {
        MessagePool {
            userid_coefs_g2: BTreeMap::new(),
            userid_poly_secrets: BTreeMap::new(),
            qual_usr: Vec::new(),
            whole_coefs: Vec::new(),
            my_id: my_id,
            n: n,
            t: t,
        }
    }

    pub fn get_mpk(&self) -> Option<G2> {
        if self.whole_coefs.is_empty() {
            return None;
        }
        Some(self.whole_coefs[0])
    }

    pub fn set_client_id_coefs(&mut self, src_id: i32, coefs: &Vec<G2>) -> bool {
        if coefs.len() != self.t {
            println!("not right");
            return false;
        }
        let ret = self
            .userid_coefs_g2
            .insert(src_id, coefs.clone())
            .map_or_else(|| true, |_| false);
        ret
    }

    pub fn set_client_id_secrets(&mut self, src_id: i32, secret: Fr) -> bool {
        self.userid_poly_secrets
            .insert(src_id, secret)
            .map_or_else(|| true, |_| false)
    }

    /*verify $id's secret is ok or not via coefs_g2 */
    pub fn verify(&self, id: i32) -> bool {
        let ret = self.userid_poly_secrets.get(&id).and_then(|secret| {
            self.userid_coefs_g2.get(&id).and_then(|coefs| {
                let lhs = G2::one() * *secret;
                let mut rhs = G2::zero();

                let s: String = self.my_id.to_string();
                let j_fr: Fr = Fr::from_str(&s).unwrap();
                let mut jk: Fr = Fr::one();
                for k in 0..self.t {
                    rhs = rhs + coefs[k] * jk;
                    jk = jk * j_fr;
                }
                Some(lhs == rhs)
            })
        });
        ret.unwrap_or(false)
    }

    pub fn calc_sk(&self) -> Option<Fr> {
        let mut sk = Fr::zero();
        for id in self.qual_usr.iter() {
            if let Some(one_sk) = self.userid_poly_secrets.get(id) {
                sk = sk + *one_sk;
            } else {
                return None;
            }
        }
        Some(sk)
    }

    pub fn calc_pk(&self, my_id: i32) -> Option<G2> {
        let mut pk = G2::zero();
        let s: String = my_id.to_string();
        let i_fr: Fr = Fr::from_str(&s).unwrap();
        let mut ik = Fr::one();
        for k in 0..self.t {
            let ak = self.whole_coefs[k];
            pk = pk + ak * ik;
            ik = ik * i_fr;
        }
        Some(pk)
    }

    pub fn cal_mpk(&self) -> Option<G2> {
        if self.whole_coefs.is_empty() {
            return None;
        }
        Some(self.whole_coefs[0])
    }

    fn clean_unqual_id(&mut self, id: i32) {
        self.userid_coefs_g2.remove(&id);
        self.userid_poly_secrets.remove(&id);
    }

    pub fn get_qual_usr(&mut self) -> Vec<i32> {
        let ids: Vec<i32> = self.userid_coefs_g2.keys().cloned().collect();
        for i in ids.iter() {
            if self.verify(*i) {
                self.qual_usr.push(*i);
            } else {
                self.clean_unqual_id(*i);
            }
        }
        return self.qual_usr.clone();
    }

    pub fn calc_whole_coefs(&mut self) -> bool {
        // calc the public value
        for k in 0..self.t {
            let mut ret = G2::zero();
            for i in self.qual_usr.iter() {
                if let Some(coefs) = self.userid_coefs_g2.get(&i) {
                    ret = ret + coefs[k];
                } else {
                    println!("error for not find id coefs in id {}", i);
                    return false;
                }
            }
            self.whole_coefs.push(ret);
        }
        true
    }
}
