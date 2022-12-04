extern crate core;

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

    #[test]
    fn test_2022_02_a() {
        adv_2022::adv_2022_02::strategy_score_half_info("resources/2022/02.txt");
    }

    #[test]
    fn test_2022_02_b() {
        adv_2022::adv_2022_02::strategy_score_full_info("resources/2022/02.txt");
    }
}