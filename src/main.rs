fn main() {
    if let Some(css) = read_pipe::read_pipe() {
        let result = emotionless::next(0, css);
        println!("{result}")
    }
}
