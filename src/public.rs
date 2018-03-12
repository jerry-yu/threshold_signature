pub mod utility {
    use ::bn::Fr;

    pub fn coef_gen(order: i32) -> Vec<Fr>{
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

use ::bn::{G1, G2, Group, Fr};

pub struct Polynomial {
    pub order: i32,
    pub coef: Vec<Fr>
}

impl Polynomial {
    pub fn new(_order: i32) -> Polynomial {
        Polynomial{ order: _order, coef: utility::coef_gen(_order)}
    }
}

pub struct MessagePool {
    // A means the broadcast value A_{ik} = g_2^{a_{ik}}
    A: Vec<Vec<G2>>,
    S: Vec<Vec<Fr>>,
}

impl MessagePool {
    pub fn new(clients: &mut Vec<::user::Client>, _n: i32) -> MessagePool{
        let mut _A: Vec<Vec<G2>> = Vec::new();
        let mut _S: Vec<Vec<Fr>> = Vec::new();

        for client in clients {
            _A.push(client.broadcastA());
            _S.push(client.broadcastS(_n));
        }
        MessagePool{ A: _A, S: _S}
    }

    pub fn get_message(&self, client: &::user::Client) -> Vec<G2>{
        let mut ret = Vec::new();
        for message_list in &self.A {
            ret.push(message_list[client.id as usize]);
        }
        ret
    }
}
