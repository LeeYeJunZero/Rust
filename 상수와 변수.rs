fn main(){
    let x = 5;
    println!("x is {}", x);
    x = 6;
    println!(x is  {}. x);
}

//이 코드에서 x는 불변성 변수이기에 실행 할 수 없다. x의 값을 변경하려면 mut를 사용해야한다.

fn main(){
    let mut x = 5;
    println!("x is {}", x);
    x = 6;
    println!("x is {}", x);
}

//이렇게 작성해야 함
