#[warn(dead_code)]
#[warn(unused_imports)]
#[warn(unused_variables)]
use tfhe::shortint::prelude::*;

/*
In this assignment, you are to implement a homomorphic encryption based function which privately returns the number of items in stock for a given item code without leaking the code or result.

All inputs and outputs are restricted to 4-bit unsigned integers.

Your task is to complete the implementation of the following block in main.rs:
*/
fn query(key: ServerKey, mut target: Ciphertext, inventory: &[(u8, u8)]) -> Ciphertext {
    

    let mut result = key.smart_scalar_mul(&mut target, 0); // init res with 0
    for pair in inventory {
        // target == id ?
        let mut is_equal = key.smart_scalar_equal(&mut target, pair.0);
        // result += is_equal * pair.1
        let mut value = key.smart_scalar_mul(&mut is_equal, pair.1);
        result = key.smart_add(&mut value, &mut result);
    }

    return result; // glorified else
}

fn main() {
    // nothing to do here
}

#[cfg(test)]
mod tests {
    use tfhe::shortint::parameters::PARAM_MESSAGE_4_CARRY_0_KS_PBS;
    use tfhe::shortint::prelude::*;

    use crate::query;

    #[test]
    fn test_it() {
        let (client_key, server_key) = gen_keys(PARAM_MESSAGE_4_CARRY_0_KS_PBS);

        let item_code = 0u8;

        let item_code_ciphertext = client_key.encrypt(item_code as u64);

        let stock_ciphertext = query(server_key, item_code_ciphertext, &[(1, 2), (2, 1)]);

        let stock_count = client_key.decrypt(&stock_ciphertext);

        assert_eq!(stock_count, 0); // evaluation whether the left handside equals the right hand side
    }
    #[test]
    fn test_it2() {
        let (client_key, server_key) = gen_keys(PARAM_MESSAGE_4_CARRY_0_KS_PBS);

        let item_code = 1u8;

        let item_code_ciphertext = client_key.encrypt(item_code as u64);

        let stock_ciphertext = query(server_key, item_code_ciphertext, &[(1, 14), (2, 1), (1, 1)]);

        let stock_count = client_key.decrypt(&stock_ciphertext);

        assert_eq!(stock_count, 15); // evaluation whether the left handside equals the right hand side
    }



    #[test]
    fn test_it3() {
        let (client_key, server_key) = gen_keys(PARAM_MESSAGE_4_CARRY_0_KS_PBS);

        let item_code = 1u8;

        let item_code_ciphertext = client_key.encrypt(item_code as u64);

        let stock_ciphertext = query(server_key, item_code_ciphertext, &[(1, 15), (2, 1)]);

        let stock_count = client_key.decrypt(&stock_ciphertext);

        assert_eq!(stock_count, 15); // evaluation whether the left handside equals the right hand side
    }
}
