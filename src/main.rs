
// Firstly There are 5 steps approach to compute key generation
// Step1: Choose large primes p and q
// Step2: multiply both prime number together n=p*q
// multiply the differences of 1 from each of p and q to give s(n)=(p-1)(q-1)
// choose a public key e such that the gcd(e,s(n))=1
// compute privatekey  d such that d*e=1 mod s(n)
// Kpub=(n, e) Kpr=(d)

fn main() {
    fn g_mod_p( g: usize, pr: usize,  p: usize, ) -> usize {
        let mut result = 1;
        let g_mod_p = g % p;
        for _ in 0..pr {
            result = (result * g_mod_p) % p;
        }
        return result;
    }
//    we choose 29  and 19
let p = 7;
let q = 11;

// let n  = p*q;

let s = (p-1) * (q-1);
println!("{}",s);
let kpub =7;




//  now let's encrypt

let message = 3;
let encrypt = g_mod_p(message, kpub, s); 
println!("{} encrypted message", encrypt);


let kpr = 3;
let decrypt = g_mod_p(encrypt, kpr, s);
println!("{} decrypted message", decrypt);



}
