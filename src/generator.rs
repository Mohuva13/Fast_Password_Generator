pub mod password_generator{

    // crate rand and dependencies
    use rand::{Rng, thread_rng};
    use rand::distributions::Uniform;

    // Generate password by length
    pub fn gen_by_len(count: i32) -> String{

        // generate password include symbols, numbers, and alphabets
        let password: String = thread_rng()
            .sample_iter(&Uniform::new(33u8, 126u8))
            .take(count as usize)
            .map(char::from)
            .collect();
        password
    }
}