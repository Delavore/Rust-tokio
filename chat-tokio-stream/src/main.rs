use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast,
};

fn give_me_default<T>() -> T
where
    T: Default,
{
    Default::default();
}

#[tokio::main] // we wrap main funcion with this macro
async fn main() {
    let value = give_me_default::<i32>(); // this call turbo fish or fish
    let listener = TcpListener::bind("localhost:8080").await.unwrap(); //  a TCP socket server, listening for connections; await means suspend this current task until our Future is ready to be acted on; with .unwrap we return only TcpListener without error Result(TL, error))
                                                                       // let (tx, _rx) = broadcast::channel::<String>(10);
    let (tx, _rx) = broadcast::channel(10);
    loop {
        let tx = tx.clone();
        let mut rx = tx.subscribe();
        let (mut socket, addr) = listener.accept().await.unwrap(); // accepts a new incoming connection from the TCP listener
        tokio::spawn(async move {
            // async blo
            let (read, mut write) = socket.split();

            // let mut buffer: [u8; 1024] = [0u8; 1024];
            let mut reader = BufReader::new(read); // adds buffering to any
            let mut line: String = String::new();
            loop {
                tokio::select! {
                    result = reader.read_line(&mut line) => {
                        if result.unwrap() == 0 {
                            break;
                        }
                        tx.send((line.clone(), addr)).unwrap();
                        line.clear();
                    }
                   result = rx.recv() => {
                        let (msg, other_addr) = result.unwrap(); // await alredy because it in select!
                        if addr != other_addr {
                        write.write_all (msg.as_bytes()).await.unwrap();
                        }
                   }
                } // you can run multiple asunc things at the same time
                  // redering

                //let bytes_read: usize = socket.read(&mut buffer).await.unwrap();  // pulls some bytes from this source into the specified buffer and returning how many bytes were read

                //socket.write_all(&buffer[..bytes_read]).await.unwrap();  // it doest write message to every socket, instead it writes every single
                // byte that in input buffer out (onto the output buffer)
            }
        });
    }
}
