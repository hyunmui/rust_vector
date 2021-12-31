use std::{collections::HashMap, fmt::format, vec};

fn main() {
    // COLLECTION
    let v: Vec<u32> = Vec::new();

    let v2 = vec![1, 2, 3];

    let mut vm = Vec::new();
    vm.push(5);
    vm.push(6);
    vm.push(7);
    vm.push(8);

    // unset vector
    {
        let tmpVector = vec![1, 2, 3, 4];

        // drop tmpVector
    }

    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("세 번째 원소: {}", third);

    match v.get(2) {
        Some(third) => println!("세 번째 원소: {}", third),
        None => println!("세 번째 원소가 없습니다."),
    }

    // OutOfIndex => panicked at 'index out of bounds
    // let does_not_exist = &v[100];
    let does_not_exist = v.get(100);

    if does_not_exist.is_none() {
        println!("해당 값은 None 입니다");
    }

    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];

    println!("first value: {}", first);

    v.push(6); // no error?

    // loop
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![1, 2, 3, 4, 5];
    for i in &mut v {
        *i += 50; // 역참조 연산자
    }

    for i in &v {
        println!("{}", i);
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("블루")),
        SpreadsheetCell::Float(10.12),
    ];

    // String
    let mut s = String::new();

    let data = "문자열 초기값";
    let s = data.to_string();

    // push_str & push
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("s: {}", s);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2: {}", s2);

    let mut s = String::from("lo");
    s.push('l');
    println!("s: {}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let s3 = s1 + &s2;

    println!("{}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // let s = s1 + "-" + &s2 + "-" + &s3;
    let s = format!("{}-{}-{}", s1, s2, s3);

    println!("{}", s);

    let len = String::from("안녕하세요").len();
    println!("{}", len);

    for c in "안녕하세요".chars() {
        println!("{}", c);
    }

    // HashMap
    // let mut scores = HashMap::new();

    // scores.insert(String::from("블루"), 10);
    // scores.insert(String::from("멜로"), 50);

    let teams = vec![String::from("블루"), String::from("옐로")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    println!("");

    // 해시맵 소유권 이동

    let field_name = String::from("Favorite color");
    let field_value = String::from("블루");

    let mut map = HashMap::new();
    map.insert(field_name, field_value); // 소유권이 map 으로 이동되고 field_name, field_value 는 이후 이용할 수 없음

    // println!("{} / {}", field_name, field_value);

    // 해시맵 값 접근
    let mut scores = HashMap::new();

    scores.insert(String::from("블루"), 10);
    scores.insert(String::from("옐로"), 50);

    let team_name = String::from("블루");
    let score = scores.get(&team_name);

    match score {
        Some(i) => println!("{}", i,),
        None => println!("값이 없습니다"),
    }

    println!("{}", scores["블루"]);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // 덮어씌우기
    let mut scores = HashMap::new();

    scores.insert(String::from("블루"), 10);
    scores.insert(String::from("블루"), 25);

    println!("{:?}", scores);

    // 할당 안될때만 추가
    let mut scores = HashMap::new();

    scores.insert(String::from("블루"), 10);

    scores.entry(String::from("얠로")).or_insert(50);
    scores.entry(String::from("블루")).or_insert(50);

    println!("{:?}", scores);

    // 해시맵을 이용한 단어
    let text = "hello world wonderful world";

    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
    }

    print!("{:?}", map);
}
