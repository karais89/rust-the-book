mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    // 절대 경로
    crate::front_of_house::hosting::add_to_waitlist();

    // 상대 경로
    front_of_house::hosting::add_to_waitlist();

    // 여름에 아침 식사로 호밀빵을 주문한다.
    let mut meal = back_of_house::Breakfast::summer("호밀빵");
    // 고객이 마음이 빠꿔서 빵 종류를 바꾼다.
    meal.toast = String::from("밀빵");
    println!("{} 토스트로 주세요", meal.toast);

    // 다음 줄의 주석을 해제하면 컴파일되지 않는다.
    // 고객은 식사와 함께 제공되는 계절 과일을 선택할 수 없다.
    // meal.seasonal_fruit = String::from("블루베리");
}

// super 키워드 사용 예
fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order()
    }

    fn cook_order() {}

    // pub 예
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("복숭아"),
            }
        }
    }
}