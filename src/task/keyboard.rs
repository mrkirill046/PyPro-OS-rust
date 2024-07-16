use crate::print;
use crate::*;
use conquer_once::spin::OnceCell;
use core::{
    pin::Pin,
    task::{Context, Poll},
};
use crossbeam_queue::ArrayQueue;
use futures_util::stream::Stream;
use futures_util::stream::StreamExt;
use futures_util::task::AtomicWaker;
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};

pub async fn print_key_presses() {
    let mut scan_codes = ScanCodeStream::new();
    let mut keyboard = Keyboard::new(
        ScancodeSet1::new(),
        layouts::Us104Key,
        HandleControl::Ignore,
    );

    while let Some(scan_code) = scan_codes.next().await {
        if let Ok(Some(key_event)) = keyboard.add_byte(scan_code) {
            if let Some(key) = keyboard.process_keyevent(key_event) {
                match key {
                    DecodedKey::Unicode(character) => {
                        if (character.is_alphanumeric()
                            || character.is_ascii_punctuation()
                            || character.is_whitespace())
                            && character != '\t'
                        {
                            print!("{}", character);
                        }

                        if character == '\t' {
                            print!("    ");
                        }
                    }
                    DecodedKey::RawKey(_key) => {}
                }
            }
        }
    }
}

static WAKER: AtomicWaker = AtomicWaker::new();
static SCAN_CODE_QUEUE: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();

impl Stream for ScanCodeStream {
    type Item = u8;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<u8>> {
        let queue = SCAN_CODE_QUEUE
            .try_get()
            .expect("scancode queue not initialized");

        if let Some(scan_code) = queue.pop() {
            return Poll::Ready(Some(scan_code));
        }

        WAKER.register(&cx.waker());
        match queue.pop() {
            Some(scan_code) => {
                WAKER.take();
                Poll::Ready(Some(scan_code))
            }
            None => Poll::Pending,
        }
    }
}

pub(crate) fn add_scan_code(scan_code: u8) {
    if let Ok(queue) = SCAN_CODE_QUEUE.try_get() {
        if let Err(_) = queue.push(scan_code) {
            print_warn!("WARNING: scan_code queue full; dropping keyboard input");
        } else {
            WAKER.wake();
        }
    } else {
        print_warn!("WARNING: scan_code queue uninitialized");
    }
}

pub struct ScanCodeStream {
    _private: (),
}

impl ScanCodeStream {
    pub fn new() -> Self {
        SCAN_CODE_QUEUE
            .try_init_once(|| ArrayQueue::new(100))
            .expect("ScanCodeStream::new should only be called once");
        ScanCodeStream { _private: () }
    }
}
