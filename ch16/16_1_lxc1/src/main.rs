fn main() {
    // "apicontainer"라는 이름의 lxc 컨테이너를 초기화합니다.
    let c = lxc::Container::new("apicontainer", None).expect("lxc초기화 실패");

    if !c.is_defined() {
        println!("컨테이너 생성 (시간이 꽤 걸립니다)");
        // "apicontainer"를 ubuntu focal (amd64) 버전으로 생성합니다.
        c.create(
            "download",
            None,
            None,
            lxc::CreateFlags::QUIET,
            &["-d", "ubuntu", "-r", "focal", "-a", "amd64"],
        )
        .expect("컨테이너 생성 실패");
    }

    println!("컨테이너 구동");
    c.start(false, &[]).expect("컨테이너 구동 실패");

    // 컨테이너의 상태와 PID를 출력합니다.
    println!("Container state: {}", c.state());
    println!("Container PID: {}", c.init_pid());
    
    // 컨테이너의 네트워크 인터페이스 목록을 출력합니다.
    println!("Interfaces: {:?}", c.get_interfaces());

    // 30초 안에 컨테이너가 종료되지 않으면 강제로 중지합니다.
    if c.shutdown(30).is_err() {
        c.stop().expect("컨테이너 종료 실패");
    }

    // 컨테이너를 삭제합니다.
    c.destroy().expect("컨테이너 삭제 실패");
}