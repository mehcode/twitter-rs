use egg_mode::tweet;
use egg_mode::error::Error;
use common;

use std::{thread, time};

pub fn test_all(c: &common::Config) {
    println!("Starting tweet tests...");
    test_post_delete(c);
}

fn test_post_delete(c: &common::Config) {
    println!("making posts and replies, and deleting tweets...");

    let parent = tweet::DraftTweet::new("if you can see this, i'm testing my library. shhh")
                                   .send(&c.con_token, &c.access_token).unwrap();

    thread::sleep(time::Duration::from_millis(1000));

    let reply = tweet::DraftTweet::new("writing a test reply. shhh")
                                  .in_reply_to(parent.response.id)
                                  .send(&c.con_token, &c.access_token).unwrap();

    assert_eq!(reply.response.in_reply_to_status_id, Some(parent.response.id));

    thread::sleep(time::Duration::from_millis(1000));

    tweet::delete(reply.response.id, &c.con_token, &c.access_token).unwrap();

    match tweet::show(reply.response.id, &c.con_token, &c.access_token) {
        Ok(_) => panic!("this tweet should not exist"),
        Err(Error::TwitterError(e)) => {
            if !e.errors.iter().any(|err| err.code == 144) {
                //error code 144 - "No status found with that ID."
                panic!("when looking up a deleted tweet: {:#?}", e);
            }
        },
        Err(e) => panic!("when looking up a deleted tweet: {:#?}", e),
    }

    thread::sleep(time::Duration::from_millis(1000));

    tweet::delete(parent.response.id, &c.con_token, &c.access_token).unwrap();
}
