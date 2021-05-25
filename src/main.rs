// 有使用到下面的 module。
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};

fn handle_echo_client(mut stream: TcpStream) {
    // using 50 byte buffer
    // 50 byte 來做 buffer 接收 data
    let mut data = [0 as u8; 50];
    // 讀取傳進來的 data
    while match stream.read(&mut data) {
        Ok(size) => {
            // echo everything!
            // 將傳進來的資料寫回去給 client
            stream.write(&data[0..size]).unwrap();
            true
        },
        Err(_) => {
            // 如果無法讀取資料就印出錯誤訊息，並且關閉 stream
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn main() {
    // TcpListener 用來監聽 TCP 的連結，並監聽 0.0.0.0:3333 這個位置。
    // 這邊的 bind 類似 new 這個方式，通常網路領域連結到監聽的port會稱作綁定到某一端口。
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    
    // accept connections and process them, spawning a new thread for each one
    // TcpListener 的 incoming 方法返回一个迭代器，它提供了一系列的流（更准确的说是 TcpStream 类型的流）。流（stream）代表一个客户端和服务端之间打开的连接。连接（connection）代表客户端连接服务端、服务端生成响应以及服务端关闭连接的全部请求 / 响应过程。为此，TcpStream 允许我们读取它来查看客户端发送了什么，并可以编写响应。总体来说，这个 for 循环会依次处理每个连接并产生一系列的流供我们处理。
    for stream in listener.incoming() {
        // 因為 stream 连接可能会因为很多原因不能成功，大部分是操作系统相关的。例如，很多系统限制同时打开的连接数；新连接尝试产生错误，直到一些打开的连接关闭为止。 
        // 所以是有可能回傳 Error 的。
        // 這時候就需要去使用 match 來做配對
        match stream {
            // OK則連帶將 stream 的值往 block 傳
            Ok(stream) => {
                // 印出連結的 IP 為何
                println!("New connection: {}", stream.peer_addr().unwrap());
                // 處理使用者傳遞來的消息
                handle_echo_client(stream)
            }
            Err(e) => {
                /* connection failed */
                // 如果 stream 連結有問題就印出錯誤
                println!("Error: {}", e);
            }
        }
    }
    
    // close the socket server
    // 等不再連結後將會釋放 listener 的記憶體
    drop(listener);
}

