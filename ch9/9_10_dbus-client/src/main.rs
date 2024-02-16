use dbus::blocking::Connection;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 새로운 DBus 세션 연결을 생성합니다.
    let conn = Connection::new_session()?;
    
    // DBus 프록시를 생성합니다. 
    // com.example.dbustest 서비스의 /hello 경로에 접근합니다.
    let proxy = conn.with_proxy("com.example.dbustest", "/hello", Duration::from_millis(5000));

    // Hello를 호출합니다.
    let (hello,): (String,) = proxy.method_call("com.example.dbustest", "Hello", ("luna",))?;
    
    println!("수신: {}", hello);

    Ok(())
}
