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
}

impl MessagePool {
    pub fn new(clients: &mut Vec<::user::Client>) {
        let mut ret: Vec<Vec<G2>> = Vec::new();
        for client in clients {
            ret.push(client.broadcast());
        }
    }
}
