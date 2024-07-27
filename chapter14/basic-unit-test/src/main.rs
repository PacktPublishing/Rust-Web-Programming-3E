use std::convert::TryInto;

fn add_two_numbers<T, U>(a: T, b: U) 
-> Result<T, <U as TryInto<T>>::Error>
where
    T: std::ops::Add<Output = T>,
    U: TryInto<T>,
{
    let b_converted: T = b.try_into()?;
    Ok(a + b_converted)
}

fn main() {
    println!("hello world");
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_add_two_numbers_i32() {
        let result = add_two_numbers(5i32, 3i32);
        assert_eq!(result, Ok(8));
    }
    #[test]
    fn test_add_two_numbers_i32_u64() {
        let result = add_two_numbers(5i32, 3u64);
        assert_eq!(result, Ok(8));
    }
    #[test]
    fn test_add_two_numbers_minus() {
        let result = add_two_numbers(5i32, -3i32);
        assert_eq!(result, Ok(2));
    }
    #[test]
    fn test_add_two_numbers_float_and_int() {
        let result = add_two_numbers(5.0, 3);
        assert_eq!(result, Ok(8.0));
    }
    #[test]
    fn test_add_two_numbers_outsize() {
        let result = add_two_numbers(5u8, 5000i32);
        assert_eq!(result.is_err(), true);
    }

}