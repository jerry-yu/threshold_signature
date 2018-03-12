extern crate bn;
extern crate rand;
extern crate threshold_signature;

use bn::{G1, G2, Group};
use threshold_signature::user::Client;
use threshold_signature::public;

fn main() {
    let n = 10;
    let t = 5;

    // create the vector of n users
    let mut clients: Vec<Client> = Vec::new();
    for i in 0..n {
        clients.push(Client::new(i, t));
    }

    let mut message_pool = public::MessagePool::new(&mut clients, n, t);
    for client in clients.iter() {
        client.verify(&mut message_pool);
    }
    message_pool.get_qual_usr(&mut clients);
}

#[cfg(test)]
mod tests {
    use rand;
    use bn::{pairing, Fr, G1, G2, Group};

    #[test]
    fn test_for_pairing() {
        let rng0 = &mut rand::thread_rng();
        let rng1 = &mut rand::thread_rng();
        let g1 = G1::one();
        let g2 = G2::one();
        let a = Fr::random(rng0);
        let b = Fr::random(rng1);
        let part0 = pairing(g1 * a, g2 * b);
        let part1 = pairing(g1 * b, g2 * a);
        let part2 = pairing(g1, g2).pow(a * b);
        assert!(part0 == part1);
        assert!(part0 == part2);
    }

    #[test]
    fn test_for_abel() {
        let rng0 = &mut rand::thread_rng();
        let rng1 = &mut rand::thread_rng();
        let i = G1::random(rng0);
        let j = G1::random(rng1);
        let k0 = i + j;
        let k1 = j + i;
        assert!(k0 == k1);
    }

    #[test]
    fn test_qual_usr() {
        let n = 10;
        let t = 5;

        // create the vector of n users
        let mut clients: Vec<::Client> = Vec::new();
        for i in 0..n {
            clients.push(::Client::new(i, t));
        }

        let mut message_pool = ::public::MessagePool::new(&mut clients, n);
        for client in clients.iter() {
            client.verify(&mut message_pool);
        }
        message_pool.get_qual_usr(&mut clients);

        assert!(message_pool.qual_usr.len() == n as usize)
    }
}
