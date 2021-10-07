#![allow(unused)]


use std::thread;
use std::time::Duration;
use std::sync::Mutex;

fn main() {
    shared_state();
}

fn shared_state() {
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("{:?}", m);
}

fn message_passing() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("{} from Spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("Hi there. This is {} from main thread!", i);
        thread::sleep(Duration::from_millis(2));
    }

    handle.join().unwrap();

    println!("----------------");
    let v = vec![1, 2, 3, 4];
    // without move, the thread may outlive the borrowed value v.
    let handle = thread::spawn(move || {
        println!("{:?}", v);
    });

    handle.join().unwrap();


    /*
    Message passing:
    Do not communicate by sharing memory; instead, share memory by communicating
    */

    // multiple producer, single consumer
    use std::sync::mpsc;
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let duck = String::from("Rubber Ducky");
        tx.send(duck).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    println!("rx as iterator");
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    thread::spawn(move || {
        let ducks = vec![
            String::from("Abseedee"),
            String::from("Pookai"),
            String::from("Sumpo"),
        ];
        for duck in ducks {
            tx.send(duck).unwrap();
            thread::sleep(Duration::from_secs(2));
        }
    });

    thread::spawn(move || {
        let ducks = vec![
            String::from("Shunpo"),
            String::from("Taitai"),
            String::from("Maokao"),
        ].into_iter().for_each(|duck| {
            tx1.send(duck);
            thread::sleep(Duration::from_millis(700));
        });
    });

    for received in rx {
        println!("New Duck Arrived! They are {}", received);
    }
}
