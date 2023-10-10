// errors4.rs
//
// Execute `rustlings hint errors4` or use the `hint` watch subcommand for a
// hint.


#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        // Hmm...? Why is this only returning an Ok value?
        let a: Result<PositiveNonzeroInteger, CreationError> = match <i64 as TryInto<u64>>::try_into(value) {
            Ok(x) => {
                if x == 0{
                    Err(CreationError::Zero)
                }else {
                    Ok(PositiveNonzeroInteger(value as u64))
                }
            }
            Err(t) => Err(CreationError::Negative)
            
        };
        a
    }
}

#[test]
fn test_creation() {
    assert!(PositiveNonzeroInteger::new(10).is_ok());
    assert_eq!(
        Err(CreationError::Negative),
        PositiveNonzeroInteger::new(-10)
    );
    assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
}
