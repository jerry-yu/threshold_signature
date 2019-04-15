extern crate bn;
extern crate rand;
extern crate threshold_signature;

use bn::{Group, G1};
use rand::Rng;
use std::time::Instant;
use threshold_signature::user::Client;

fn main() {
    let n: usize = 4;
    let t: usize = 3;
    let rng = &mut rand::thread_rng();
    // create the vector of n users
    let mut clients = Vec::new();
    for _ in 0..n {
        let id = rng.gen_range(1, std::i32::MAX);
        clients.push(Client::new(id, n, t).unwrap());
    }
    println!("client push ok ");

    let mut stime = Instant::now();
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            let id = clients[j].id;
            {
                stime = Instant::now();
            }
            let coef = clients[j].calc_coef_g2();
            {
                println!("calc coef g2 time {:?}", Instant::now() - stime);
                stime = Instant::now();
            }
            let sec = clients[j].calc_secret(clients[i].id);
            {
                println!("calc_secret time {:?}", Instant::now() - stime);
                stime = Instant::now();
            }

            if !clients[i].set_client_id_coefs(id, &coef) {
                println!("err set_client_id_coefs ");
            }
            if !clients[i].set_client_id_secrets(id, sec) {
                println!("err set_client_id_coefs ");
            }
        }
    }

    for i in 0..n {
        for j in 0..n {
            {
                stime = Instant::now();
            }
            if !clients[i].verify(clients[j].id) {
                println!("verify error src {} dst {}", clients[i].id, clients[j].id);
            }
            {
                println!("verify time {:?}", Instant::now() - stime);
                stime = Instant::now();
            }
        }
    }

    for i in 0..n {
        let qual_user = clients[i].get_qual_usr();
        println!(" id {}'s qual users {:?}", clients[i].id, qual_user);
    }

    for i in 0..n {
        {
            stime = Instant::now();
        }
        clients[i].calc_pk_sk();
        {
            println!("calc_pk_sk time {:?}", Instant::now() - stime);
            stime = Instant::now();
        }
    }

    let hashed_message = G1::random(rng);

    let mut sigs = Vec::new();
    let mut ids = Vec::new();

    let inspector = 1;
    for i in 0..n {
        if let Some(sig) = clients[i].calc_signature(&hashed_message) {
            {
                stime = Instant::now();
            }
            if clients[inspector].verify_single_signature(clients[i].id, &hashed_message, &sig) {
                sigs.push(sig);
                ids.push(clients[i].id);
                println!("succ in verify signature from id {}", clients[i].id);
            } else {
                println!("eroor in verify signature from id {}", clients[i].id);
            }
            {
                println!("verify_single_signature time {:?}", Instant::now() - stime);
                stime = Instant::now();
            }
        }
    }

    let mut left_ids = Vec::new();
    let mut right_ids = Vec::new();
    let mut left_sigs = Vec::new();
    let mut right_sigs = Vec::new();

    for i in 0..t {
        left_ids.push(ids[i]);
        right_ids.push(ids[i + 1]);
        left_sigs.push(sigs[i]);
        right_sigs.push(sigs[i + 1]);
    }

    let mut rhs = G1::zero();
    let mut lhs = G1::zero();
    for i in 0..t {
        {
            stime = Instant::now();
        }
        lhs = lhs + left_sigs[i] * Client::calc_lambda(0, t, i, &left_ids);
        {
            println!("calc_lambda time {:?}", Instant::now() - stime);
            stime = Instant::now();
        }
        rhs = rhs + right_sigs[i] * Client::calc_lambda(0, t, i, &right_ids);
        {
            println!("calc_lambda time {:?}", Instant::now() - stime);
        }
    }
    assert!(lhs == rhs);

    {
        stime = Instant::now();
    }
    if clients[inspector].verify_completed_signature(&hashed_message, &lhs) {
        println!("verify completed signature ok");
    } else {
        println!("verify completed signature not ok");
    }
    {
        println!(
            "verify_completed_signature time {:?}",
            Instant::now() - stime
        );
        stime = Instant::now();
    }
}

#[cfg(test)]
mod tests {
    use bn::{pairing, Fr, Group, G1, G2};
    use rand;

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
}
