/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

#![feature(async_await, await_macro, generators)]

use std::io;
use std::collections::HashMap;

use futures::SinkExt;
use futures::channel::mpsc::{self, UnboundedSender, UnboundedReceiver};
use futures::executor::{self, ThreadPool};
use futures::task::SpawnExt;
use futures_util::stream::StreamExt;
use romio::{TcpListener, TcpStream};
use futures_util::io::AsyncReadExt;
use futures_util::io::WriteHalf;
use std::sync::Arc;

use calculator::process_client;

mod calculator;

// async fn maintain_client_list(mut streams: HashMap<bool, bool>, mut rx: UnboundedReceiver<(bool, &WriteHalf<TcpStream>)>) -> io::Result<()> {
//     // let mut streams = Vec::new();
//     // while let Ok(Some((connected, addr))) = rx.try_next() {
//     //     dbg!(connected,addr);
//     //     if connected {
//     //         streams.push(addr);
//     //     } else {
//     //         streams.remove_item(addr);
//     //     }
//     //     yield
//     // }

//     Ok(())
// }

fn main() -> io::Result<()> {
    executor::block_on(async {
        let mut threadpool = ThreadPool::new()?;

        let mut listener = TcpListener::bind(&"127.0.0.1:7878".parse().unwrap())?;
        let mut incoming = listener.incoming();

        let streams = Arc::new(HashMap::new());

        println!("Listening on 127.0.0.1:7878");

        while let Some(stream) = await!(incoming.next()) {
            let stream = stream?;
            let addr = stream.peer_addr()?;

            let ss = Arc::clone(&streams);
            threadpool.spawn(async move {
                println!("Accepting stream from: {}", addr);
                // let client_tx = stream.split().1;

                if let Ok(mut s) = Arc::try_unwrap(ss) {
                    s.insert(&addr, &stream);

                    await!(process_client(stream, s)).unwrap();

                }
                println!("Closing stream from: {}", addr);
            }).unwrap();
        }

        Ok(())
    })
}
