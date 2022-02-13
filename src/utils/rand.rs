/// secret を生成するための乱数生成器
pub fn gen_secret(len: u8) -> String {
    use rand::Rng;

    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";

    let mut rng = rand::thread_rng();
    let secret: String = (0..len)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    // println!("{:?}", secret);

    secret
}

/// test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen_secret() {
        gen_secret(255);
    }
}
