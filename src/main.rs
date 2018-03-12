extern crate threshold_signature;
extern crate bn;
extern crate rand;

use bn::{G1, G2, Group};
use threshold_signature::user;
use threshold_signature::public;

// fn main() {
//     let rng0 = &mut rand::thread_rng();
//     let rng1 = &mut rand::thread_rng();
//     let i: bn::Fr = bn::Fr::random(rng0);
//     let j: bn::Fr = bn::Fr::random(rng1);
// }

fn main() {
    let n = 10;
    let t = 5;

    // create the vector of n users
    let mut clients: Vec<user::Client> = Vec::new();
    for i in 0..n {
        clients.push(user::Client::new(i, t));
    }

    let mut message_pool = public::MessagePool::new(&mut clients);
    
}
