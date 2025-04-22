use bbqueue::{BBBuffer, Consumer, Error, Producer};

use super::messages::Message;

const PACKET_SIZE: usize = std::mem::size_of::<Message>();
pub const QUEUE_SIZE: usize = 32 * PACKET_SIZE;
pub static BB: BBBuffer<QUEUE_SIZE> = BBBuffer::new();

pub fn send(prod: &mut Producer<QUEUE_SIZE>, packet: Message) {
    let size = std::mem::size_of::<Message>();
    // Request space for one byte
    let mut wgr = prod.grant_exact(size).unwrap();
    let packet_buf: [u8; std::mem::size_of::<Message>()] = unsafe { std::mem::transmute(packet) };

    for i in 0..packet_buf.len() {
        wgr[i] = packet_buf[i];
    }

    // Make the data ready for consuming
    wgr.commit(size);
}

pub fn consume(cons: &mut Consumer<QUEUE_SIZE>) -> Option<Message> {
    let rgr = cons.read();

    if rgr.is_err() {
        return None;
    }

    let rgr = rgr.unwrap();

    let mut packet_buf: [u8; std::mem::size_of::<Message>()] = [0; std::mem::size_of::<Message>()];

    //copy the buffer
    packet_buf.copy_from_slice(rgr.buf());
    // Release the buffer
    rgr.release(PACKET_SIZE);

    // transform the buffer into a Message
    let packet = unsafe { std::mem::transmute(packet_buf) };

    Some(packet)
}
