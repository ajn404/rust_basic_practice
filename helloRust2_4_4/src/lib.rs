#[cfg(test)]

mod tests {
    use std::net::{IpAddr, Ipv4Addr, TcpStream};

    #[test]

    fn enum_a_1() {
        assert_eq!(1, 1);
    }

    fn enum_basic() {
        #[derive(Debug)]
        enum PokerSuit {
            Clubs,
            Spades,
            Diamonds,
            Hearts,
        }

        let clubs = PokerSuit::Clubs;
        match clubs {
            PokerSuit::Clubs => {
                println!("hahihai");
            }
            _ => {
                println!("WAwa wa");
            }
        }
    }

    #[test]
    fn stdEnum() {
        //任何类型的数据都可以放入枚举成员中: 例如字符串、数值、结构体甚至另一个枚举。
        let x = IpAddr::V4(Ipv4Addr::new(100, 128, 22, 22));
        dbg!(x);
    }

    // #[test]
    // fn websocket_enum(){
    //     enum Websocket{
    //         Tcp(Websocket<TcpStream>),
    //         Tls(Websocket<native_tls::TlsStream<TcpStream>>),
    //     }

    //     let mut s = stream;
    //     if tls {
    //       s = negotiate_tls(stream)
    //     }

    //     // websocket是一个WebSocket<TcpStream>或者
    //     //   WebSocket<native_tls::TlsStream<TcpStream>>类型
    //     websocket = WebSocket::from_raw_socket(
    //       stream, ......)

    // }

    #[test]
    fn option_some_none() {
        let _x = Some(100);
        let _y = Some(String::from("wtm sss"));
        //  let z  = None;
        let _z: Option<String> = None;
        assert_eq!(_z.is_some(),false);
        //Option<T>和T是不同的类型
        // let sum = _x +100;
        //cannot add `{integer}` to `Option<{integer}>`
    }


    #[test]
    fn about_match(){
        enum Haihaihai{
            hh(i32),
            ww(i32),
        }

        let x = Haihaihai::hh(99);
        match x {
            Haihaihai::hh(i)=>{
                dbg!(i);
            },
            Haihaihai::ww(j)=>{
                dbg!(j);
            },
            _=>{
                eprintln!("jb error");
            }
        }

        
    }
}
