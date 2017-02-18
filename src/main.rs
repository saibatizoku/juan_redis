extern crate redis;
use std::error::Error;
use redis::Commands;


fn do_set_key(con: &redis::Connection) -> redis::RedisResult<()> {
    let key = "the_key";
    let _: () = try!(con.set(key, 42));
    println!("Key set!: {}, {}", key, 42);

    Ok(())
}

fn do_run_code() -> redis::RedisResult<()> {
    let client = try!(redis::Client::open("redis://redis.juan.libres:6379/"));
    let con = try!(client.get_connection());

    try!(do_set_key(&con));

    Ok(())
}


fn main() {
    match do_run_code() {
      Err(err) => {
        println!("Could not execute libres redis client:");
        println!("  {}: {}", err.category(), err.description());
      }
      Ok(()) => {}
    }
}
