use snafu::prelude::*;

const INPUT: &str = include_str!("../input");

#[snafu::report]
fn main() -> Result<()> {
    Ok(())
}

#[derive(Debug, Snafu)]
enum Error {}

type Result<T, E = Error> = std::result::Result<T, E>;

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = include_str!("../input.test");

    #[test]
    #[snafu::report]
    fn example() -> Result<()> {
        Ok(())
    }
}
