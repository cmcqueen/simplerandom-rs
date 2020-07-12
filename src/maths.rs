
/*
 * Modular exponentiation.
 *
 * Calculation of 'base' to the power of an unsigned integer 'n',
 * modulo a value 'm'.
 *
 * TODO: Complete this. This is untested work-in-progress.
 */
fn pow_mod<T, N>(base: T, n: N, m: T) -> T {
    let mut result: T = 1;
    let mut temp_exp = base;

    loop {
        if (n & 1) {
            result = ((result * temp_exp) % m) as T;
        }
        n = n >> 1;
        if n == 0 {
            break;
        }
        temp_exp = ((temp_exp * temp_exp) % m) as T;
    }
    return result;
}

