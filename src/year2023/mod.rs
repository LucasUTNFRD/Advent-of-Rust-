macro_rules! declare_days {
    ($($day:ident),* $(,)?) => {
        $(
            pub mod $day;
        )*
    };
}

declare_days!(day1, day2, day3, day4, day5, day6, day7, day8, day9, day10);
