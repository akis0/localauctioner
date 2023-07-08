use rocket::post;
use rusqlite::{params,Connection,Result};
use std::io;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use rocket::tokio::task::spawn_blocking;
use rocket;
use rocket::tokio::time::{sleep, Duration};
// users table (userid unique,name text,balance int)
// items table (itemid unique,name text,startprice int,currentprice int,ownerid int,putupdate,deadlinedate)
// bids table (bidid unique,itemid int,userid int,date,price int, success bool )
// balancechange table (balancechangeid unique,payuserid int,getuserid, date,amount int)

/* 
get userid : userid,name,balance,items
get itemid : itemid,name,startprice,currentprice,ownerid,putupdate,deadlinedate
get items : all items that meet condition

post itemid price : bid with price 
post item : put up new item 
post item state: item shall be nocked down

*/






fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
        println!("connection");
    }
    println!("connection closed");
    io::stdout().flush().unwrap();
}
fn open_db()->Result<Connection,rusqlite::Error>{
    let path = "./auctionerdb.db3";
    let con = Connection::open(&path)?;
    println!("{}",con.is_autocommit());
    Ok(con)
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let greeting = format!("hello. to exit, send \"exit\"\n");
    stream.write(greeting.as_bytes()).unwrap();
    stream.flush().unwrap();

    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let headadduser = b"adduser";
    let headsell = b"sell";
    let headbid = b"bid";
    let headfinishbid = b"finishbid";
    let headtopup = b"topup";
    let headshowitems = b"showitems";
    let headexit = b"exit";
    if buffer.starts_with(headadduser) {
        handle_adduser(stream);
    } else if buffer.starts_with(headsell) {
        handle_sell(stream);
    // } else if buffer.starts_with(headbid) {
    //     handle_bid(stream);
    // } else if buffer.starts_with(headfinishbid) {
    //     handle_finishbid(stream);
    // } else if buffer.starts_with(headtopup) {
    //     handle_topup(stream);
    // } else if buffer.starts_with(headshowitems) {
    //     handle_showitems(stream);
    // 
    }else if buffer.starts_with(headexit) {
        let response = format!("successfully exited\n");
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        let response = format!("invalid\n");
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }

}

fn handle_adduser(mut stream: TcpStream) -> i32 {
    

    let askname = format!("your name:");
    stream.write(askname.as_bytes()).unwrap();
    stream.flush().unwrap();
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    if buffer.starts_with(b"exit"){
        return -1;
    }
    let username =format!("{}",String::from_utf8_lossy(&buffer[..])).replace("\r","").replace("\n","");
    let askbalance = format!("your balance:");
    stream.write(askbalance.as_bytes()).unwrap();
    stream.flush().unwrap();
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    if buffer.starts_with(b"exit"){
        return -1;
    }
    let balance =format!("{}",String::from_utf8_lossy(&buffer[..])).replace("\r","").replace("\n","");
    print!("{}",balance);

    let dbcon = open_db().unwrap();
    //dbにユーザーを追加する処理。
    let mut l:usize=0;
    match dbcon.query_row("select count (?1) from Users",params!["id"],|row| row.get(0),){
        Ok(re)=>l=re,
        Err(err) => println!("error{}",err)
    }
    
    dbcon.execute("insert into Users (id,name,balance) values (?1, ?2 , ?3 )",params![l+1,username,balance]).unwrap();

    Connection::close(dbcon).unwrap();

    let tellid = format!("your id:{}\n",l+1);
    stream.write(tellid.as_bytes()).unwrap();
    stream.flush().unwrap();
    return 0;
}

fn handle_sell(mut stream: TcpStream) -> i32 {
    let asksell = format!("your user id:");
    stream.write(asksell.as_bytes()).unwrap();
    stream.flush().unwrap();
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    if buffer.starts_with(b"exit"){
        return -1;
    }
    let userid =format!("{}",String::from_utf8_lossy(&buffer[..])).replace("\r","").replace("\n","");
    
    let askname = format!("your item name:");
    stream.write(askname.as_bytes()).unwrap();
    stream.flush().unwrap();
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    if buffer.starts_with(b"exit"){
        return -1;
    }
    let item_name =format!("{}",String::from_utf8_lossy(&buffer[..])).replace("\r","").replace("\n","");

    let askprice = format!("start price:");
    stream.write(askprice.as_bytes()).unwrap();
    stream.flush().unwrap();
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    if buffer.starts_with(b"exit"){
        return -1;
    }
    let start_price =format!("{}",String::from_utf8_lossy(&buffer[..])).replace("\r","").replace("\n","");


    let dbcon = open_db().unwrap();
    let mut l:usize=0;
    match dbcon.query_row("select count (?1) from Items",params!["id"],|row| row.get(0),){
        Ok(re)=>l=re,
        Err(err) => println!("error{}",err)
    }
    dbcon.execute("insert into Items (id,itemname,ownerid,startprice) values (?1, ?2 , ?3,?4 )",params![l+1,item_name,userid,start_price]).unwrap();
    Connection::close(dbcon).unwrap();
    
    let tellid = format!("your item id:{}\n",l+1);
    stream.write(tellid.as_bytes()).unwrap();
    stream.flush().unwrap();
    return 0;
}
// handle_bid(mut stream: TcpStream){

// }
// handle_finishbid(mut stream: TcpStream);
// handle_topup(mut stream: TcpStream);
// handle_showitems(mut stream: TcpStream);

// fn content_in_message(message: Message) -> u32 {
//     match message {
//         Message::User => 1,
//         Message::Sell => 2,
//         Message::Bid => 3,
//         Message::FinishBid => 4,
//         Message::TopUp => 5,
//         Message::ShowItems => 6,
//     }
// }

enum Message {
    AddUser,
    Sell,
    Bid,
    FinishBid,
    TopUp,
    ShowItems,
}



struct Sell {
    userid: i32,
    item_name: String,
    start_price: i32,
}

struct Bid {
    userid: i32,
    item_id: i32,
    bid_price: i32,
}

struct FinishBid {
    userid: i32,
    item_id: i32,
    confirm: String,
}

struct TopUp {
    userid: i32,
    balance: i32,
}

struct ShowItems {
    highestprice: i32,
}
