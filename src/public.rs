extern crate bn;


pub mod utility {
    fn coef_gen(order: i32) -> Vec<bn::Fr>{
        let mut ret = Vec::new();
        let rng = &mut rand::thread_rng();
        for i in 0..order {
            ret.push(bn::Fr::random(rng));
        }
        ret
    }

    #[test]
    fn test_coef_gen() {
        let coef = coef_gen(10);
        assert_eq!(coef.len, 10 as usize);
    }
}

struct Polynomial {
    order: i32,
    coef: Vec<bn::Fr>
}

impl Polynomial {
    fn new(_order: i32) -> Polynomial {
        Polynomial{ order: _order, coef: utility::coef_gen(order)}
    }
}
