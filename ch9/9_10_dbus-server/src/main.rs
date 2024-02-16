use dbus::blocking::Connection;
use dbus_crossroads::{Crossroads, Context};
use std::error::Error;

struct Hello { 
    called_count: u32 
}

fn main() -> Result<(), Box<dyn Error>> {
    // 새로운 DBus 세션 연결을 생성합니다.
    let c = Connection::new_session()?;
    // "com.example.dbustest"라는 이름을 요청하여 DBus에 등록합니다.
    c.request_name("com.example.dbustest", false, true, false)?;

    // Crossroads 인스턴스를 생성합니다. 
    let mut cr = Crossroads::new();
    
    // com.example.dbustest인터페이스 등록
    let iface_token = cr.register("com.example.dbustest", |b| {
        // "HelloHappened"라는 dbus signal을 설정합니다.
        let hello_happened = b.signal::<(String,), _>("HelloHappened", ("sender",)).msg_fn();
        
        // "Hello" 메서드를 정의합니다. 
        b.method("Hello", ("name",), ("reply",), move |ctx: &mut Context, hello: &mut Hello, (name,): (String,)| {
            println!("클라이언트 쿼리: 안녕 {}!", name);
            hello.called_count += 1;
            let reply = format!("안녕 {}! API호출 횟수: {}", name, hello.called_count);
            let signal_msg = hello_happened(ctx.path(), &(name,));
            ctx.push_msg(signal_msg);
            Ok((reply,))
        });
    });

    // "/hello"라는 경로에 Hello 구조체 인스턴스를 삽입하고 인터페이스 토큰을 연결합니다.
    cr.insert("/hello", &[iface_token], Hello { called_count: 0});

    // 생성된 Crossroads 인스턴스를 사용하여 DBus 연결에서 메시지를 처리하도록 서비스를 시작합니다.
    cr.serve(&c)?;
    
    Ok(())
}
