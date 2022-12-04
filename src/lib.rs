mod adv_2022;

#[cfg(test)]
mod tests {
    use crate::adv_2022;

    #[test]
    fn test_2022_01_a() {
        adv_2022::adv_2022_01::calories_on_top_elves(1, "resources/2022/01.txt");
    }

    #[test]
    fn test_2022_01_b() {
        adv_2022::adv_2022_01::calories_on_top_elves(3, "resources/2022/01.txt");
    }
}