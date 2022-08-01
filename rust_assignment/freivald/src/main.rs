// TODO: Import necessary libraries. Check cargo.toml and the documentation of the libraries.
use rand::Rng;
use ndarray::prelude::*;
use ark_bls12_381::fq::Fq;
use ark_ff::fields::Field;
use ark_ff::fields::models::Fp384;

struct Freivald {
    x: Array1<Fq>  //Vec<Fq>// Array/Vec of Fq,
}

impl Freivald {
    // TODO: Create constructor for object
    fn new(array_size: usize) -> Self {
        // todo!();

        // Generate random number
        let r: Fq = rand::thread_rng().gen(); // generates an unisigned int 32

        // Populate vector with values r^i for i=0..matrix_size
        let mut vector: Vec<Fq> = Vec::new();
        for i in 0..array_size {
            let n = i as u64;
            vector.push(r.pow([n].as_ref()));
        }

        // Return freivald value with this vector as its x value
        Self {
            x: Array1::from(vector)
        }

    }

    // TODO: Add proper types to input matrices. Remember matrices should hold Fq values
    fn verify(&self, matrix_a: &Array2<Fq>, matrix_b: &Array2<Fq>, supposed_ab: &Array2<Fq>) -> bool {
        if !check_matrix_dimensions(matrix_a, matrix_b, supposed_ab) {
            return false;
        }
        // todo!()
        // TODO: check if a * b * x == c * x. Check algorithm to make sure order of operations are
        // correct

        let bx = matrix_b.dot(&self.x);
        let abx = matrix_a.dot(&bx);

        let cx = supposed_ab.dot(&self.x);

        return abx == cx;

    }

    // utility function to not have to instantiate Freivalds if you just want to make one
    // verification.
    // TODO: Add types for arguments
    fn verify_once(matrix_a: &Array2<Fq>, matrix_b: &Array2<Fq>, supposed_ab: &Array2<Fq>) -> bool {
        let freivald = Freivald::new(supposed_ab.nrows());
        freivald.verify(matrix_a, matrix_b, supposed_ab)
    }
}
// TODO: [Bonus] Modify code to increase your certainty that A * B == C by iterating over the protocol.
// Note that you need to generate new vectors for new iterations or you'll be recomputing same
// value over and over. No problem in changing data structures used by the algorithm (currently its a struct
// but that can change if you want to)


// You can either do a test on main or just remove main function and rename this file to lib.rs to remove the
// warning of not having a main implementation
fn main() {
    // todo!()
    // tests::freivald_verify_success_test();
}

// TODO: Add proper types to input matrices. Remember matrices should hold Fq values
pub fn check_matrix_dimensions(matrix_a: &Array2<Fq>, matrix_b: &Array2<Fq>, supposed_ab: &Array2<Fq>) -> bool {
    // TODO: Check if dimensions of making matrix_a * matrix_b matches values in supposed_ab.
    // If it doesn't you know its not the correct result independently of matrix contents

    // We only compare rows because we are only considering square matrices
    // todo!()
    return (matrix_a.nrows() == supposed_ab.nrows()) & (matrix_b.nrows() == supposed_ab.nrows());
}

#[cfg(test)]
mod tests {
    // #[macro_use]
    use lazy_static::lazy_static;
    use rstest::rstest;

    use super::*;

    lazy_static! {
        // todo!("add matrices types and values")
        static ref MATRIX_A: Array2<Fq> = array![[Fp384::from(1), Fp384::from(2)],[Fp384::from(3), Fp384::from(4)]];
        static ref MATRIX_A_DOT_A: Array2<Fq> = array![[Fp384::from(7), Fp384::from(10)],[Fp384::from(15), Fp384::from(22)]];    // Correct result of A * A
        static ref MATRIX_B: Array2<Fq> = array![[Fp384::from(5), Fp384::from(6)],[Fp384::from(7), Fp384::from(8)]];
        static ref MATRIX_B_DOT_B: Array2<Fq> = array![[Fp384::from(67), Fp384::from(78)],[Fp384::from(91), Fp384::from(106)]];       // Correct result of B * B
        static ref MATRIX_C: Array2<Fq> = array![[Fp384::from(1), Fp384::from(1), Fp384::from(1), Fp384::from(1), Fp384::from(1), Fp384::from(1), Fp384::from(1), Fp384::from(1)], [Fp384::from(1), Fp384::from(1), Fp384::from(1), Fp384::from(1), Fp384::from(1), Fp384::from(1), Fp384::from(1), Fp384::from(1)], [Fp384::from(1), Fp384::from(1), Fp384::from(1), Fp384::from(1), Fp384::from(1), Fp384::from(1), Fp384::from(1), Fp384::from(1)], [Fp384::from(1), Fp384::from(1), Fp384::from(1), Fp384::from(1), Fp384::from(1), Fp384::from(1), Fp384::from(1), Fp384::from(1)], [Fp384::from(1), Fp384::from(1), Fp384::from(1), Fp384::from(1), Fp384::from(1), Fp384::from(1), Fp384::from(1), Fp384::from(1)], [Fp384::from(1), Fp384::from(1), Fp384::from(1), Fp384::from(1), Fp384::from(1), Fp384::from(1), Fp384::from(1), Fp384::from(1)], [Fp384::from(1), Fp384::from(1), Fp384::from(1), Fp384::from(1), Fp384::from(1), Fp384::from(1), Fp384::from(1), Fp384::from(1)], [Fp384::from(1), Fp384::from(1), Fp384::from(1), Fp384::from(1), Fp384::from(1), Fp384::from(1), Fp384::from(1), Fp384::from(1)]];
        static ref MATRIX_C_DOT_C: Array2<Fq> = array![[Fp384::from(8), Fp384::from(8), Fp384::from(8), Fp384::from(8), Fp384::from(8), Fp384::from(8), Fp384::from(8), Fp384::from(8)], [Fp384::from(8), Fp384::from(8), Fp384::from(8), Fp384::from(8), Fp384::from(8), Fp384::from(8), Fp384::from(8), Fp384::from(8)], [Fp384::from(8), Fp384::from(8), Fp384::from(8), Fp384::from(8), Fp384::from(8), Fp384::from(8), Fp384::from(8), Fp384::from(8)], [Fp384::from(8), Fp384::from(8), Fp384::from(8), Fp384::from(8), Fp384::from(8), Fp384::from(8), Fp384::from(8), Fp384::from(8)], [Fp384::from(8), Fp384::from(8), Fp384::from(8), Fp384::from(8), Fp384::from(8), Fp384::from(8), Fp384::from(8), Fp384::from(8)], [Fp384::from(8), Fp384::from(8), Fp384::from(8), Fp384::from(8), Fp384::from(8), Fp384::from(8), Fp384::from(8), Fp384::from(8)], [Fp384::from(8), Fp384::from(8), Fp384::from(8), Fp384::from(8), Fp384::from(8), Fp384::from(8), Fp384::from(8), Fp384::from(8)], [Fp384::from(8), Fp384::from(8), Fp384::from(8), Fp384::from(8), Fp384::from(8), Fp384::from(8), Fp384::from(8), Fp384::from(8)]];           // Correct result of C * C
    }

    #[rstest]
    #[case(&MATRIX_A, &MATRIX_A, &MATRIX_A_DOT_A)]
    #[case(&MATRIX_B, &MATRIX_B, &MATRIX_B_DOT_B)]
    #[case(&MATRIX_C, &MATRIX_C, &MATRIX_C_DOT_C)]
    fn freivald_verify_success_test(
        #[case] matrix_a: &Array2<Fq>,
        #[case] matrix_b: &Array2<Fq>,
        #[case] supposed_ab: &Array2<Fq>,
    ) {
        let freivald = Freivald::new(supposed_ab.nrows());
        assert!(freivald.verify(matrix_a, matrix_b, supposed_ab));
    }

    #[rstest]
    #[case(&MATRIX_A, &MATRIX_B, &MATRIX_A_DOT_A)]
    #[case(&MATRIX_B, &MATRIX_A, &MATRIX_B_DOT_B)]
    #[case(&MATRIX_C, &MATRIX_B, &MATRIX_C_DOT_C)]
    fn freivald_verify_fail_test(
        #[case] a: &Array2<Fq>,
        #[case] b: &Array2<Fq>,
        #[case] c: &Array2<Fq>,
    ) {
        let freivald = Freivald::new(c.nrows());
        assert!(!freivald.verify(a, b, c));
    }
}
