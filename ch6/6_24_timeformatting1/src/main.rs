use chrono::Local;

fn main() {
    // 현재 로컬 날짜와 시간을 가져옵니다.
    let now = Local::now();
    
    // 현재 날짜를 YYYY-MM-DD 형식으로 출력합니다.
    println!("{}", now.format("%Y-%m-%d"));
    
    // 현재 시간을 HH:MM:SS 형식으로 출력합니다.
    println!("{}", now.format("%H:%M:%S"));
    
    // 현재 날짜와 시간을 사용자 정의 포맷으로 출력합니다.
    // 이 경우 "오늘은 [요일], [월] [일], [년]. 현재 시간은 [시:분:초]." 형식으로 출력됩니다.
    println!("{}", now.format("오늘은 %A, %B %d, %Y. 현재 시간은 %H:%M:%S."));
}