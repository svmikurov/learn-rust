#[derive(Debug)]
enum UsState {
    Alabama,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}


fn value_in_cents(coin: Coin) -> u8 {
    // match - ключевое слово
    // значение coin - выражение
    // в фигурных скобках - ветки {
    //  Coin::Nickel ...  - шаблон (шаблоны должны покрывать все возможные варианты)
    //  ... => ...        - оператор
    //  ... 5,            - выражение (код ассоциированный для данного ответвления)
    // }
    // Сравнение с if:
    //   if выражение должно возвращать boll
    //   match выражением может быть любой тип, например Coin
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}");
            25
        },
    }
}


fn main() {

    dbg!(value_in_cents(Coin::Penny));
    dbg!(value_in_cents(Coin::Nickel));
    dbg!(value_in_cents(Coin::Dime));
    dbg!(value_in_cents(Coin::Quarter(UsState::Alabama)));
}
