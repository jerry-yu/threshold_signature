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
    pub A: Vec<Vec<G2>>,
    pub anti_vote: Vec<Vec<i32>>,
    S: Vec<Vec<Fr>>,
}

impl MessagePool {
    pub fn new(clients: &mut Vec<::user::Client>, _n: i32) -> MessagePool{
        let mut _a: Vec<Vec<G2>> = Vec::new();
        let mut _s: Vec<Vec<Fr>> = Vec::new();
        let mut _anti_vote: Vec<Vec<i32>> = Vec::new();

        for client in clients {
            _a.push(client.broadcast_a());
            _s.push(client.broadcast_s(_n));
            _anti_vote.push(Vec::new());
        }
        MessagePool{ A: _a, S: _s, anti_vote: _anti_vote}
    }
    pub fn get_message(&self, client: &::user::Client) -> Vec<Fr>{
        let mut ret = Vec::new();
        for message_list in &self.S {
            ret.push(message_list[client.id as usize]);
        }
        ret
    }
}
