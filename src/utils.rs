use std::str::FromStr;

pub fn parse_line_separated_list<T: FromStr>(content: &str) -> Result<Vec<T>, T::Err> {
    content.lines().map(str::parse).collect()
}
