#[cfg(test)]
#[derive(Debug)]
enum UiObejct {
    Button,
    SelectBox,
}

mod tests {

    use super::*;
    #[test]
    fn example_01() {
        let obj = [UiObejct::Button, UiObejct::SelectBox];
        for ite in obj {
            draw(ite);
        }

        fn draw(ite: UiObejct) {
            eprintln!("{:?}", ite);
        }
    }
}
