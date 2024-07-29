// 쉐도잉이란 mut을 사용하지 않고 let으로 선언한 변수명을 한 번 더 선언하여 새 변수는 이전 변수를 참조하는 것을 쉐도잉이라고 한다.


fn main() {
    let x = 5; // 첫 번째 선언
    println!("The value of x is: {}", x); // 출력: 5

    let x = x + 1; // 두 번째 선언, 첫 번째 x를 쉐도잉
    println!("The value of x is: {}", x); // 출력: 6

    {
        let x = x * 2; // 블록 내에서 세 번째 선언, 두 번째 x를 쉐도잉
        println!("The value of x in the inner scope is: {}", x); // 출력: 12
    }

    println!("The value of x is: {}", x); // 출력: 6, 블록 내 쉐도잉된 변수는 범위를 벗어났기 때문에 영향 없음

    let x = "Hello"; // 네 번째 선언, 타입을 문자열로 변경하여 쉐도잉
    println!("The value of x is: {}", x); // 출력: Hello
}

////////////////////////////////////////////////////
fn main(){
  let x = 5;
  x = x + 1;
  x = x -2;
  println!("x is {}", x);
}
//이 코드를 사용하면 불변성의 법칙에 의해 에러가 발생한다
/////////////////////////////////////////////////////////////////
fn main(){
  let x = 5;
  let x = x + 1;
  let x = x - 2;
  println!("x is {}", x)
  
}
//위 코드처럼 let으로  걔속 선언하면서 값을 참조한다면 같은 이름의 변수 값을 바꿀 수 있다.


//쉐도잉의 중요한 점은 "데이터 타입"이 변경 되어도 상관 없다는 것이다.

fn main() {
    let str_len = "abcd1234";  // 문자열 리터럴을 변수에 할당(str 타입 변수)
    let str_len = str_len.len();  // 쉐도잉을 사용하여 문자열 길이로 새로 할당(usize 타팁 변수)
    println!("length {}", str_len);  // 문자열 길이를 출력
}

//주의사항 가변성 변수 mut을 사용해서 선언한 변수에 다른 데이터 타입을 대입하는 것은 불가능하다.
fn main(){
  let mut str_len = "abcd1234";
  str_len = str_len.len();
  println!("str_len {}", str_len);
}
