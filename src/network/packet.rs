extern crate rlibc;

use byteorder::{ByteOrder, BigEndian};
use std::cell::RefCell;
use std::io::Write;

pub struct SerializedBuffer {
    pub buffer: Vec<u8>,
    position: usize,
    limit: usize,
    capacity: usize,
    calculated_size_only: bool,
}

impl SerializedBuffer {
    pub fn new_with_size(size: usize) -> SerializedBuffer {
        let mut sb = SerializedBuffer {
            buffer: Vec::with_capacity(size),
            position: 0,
            limit: size,
            capacity: size,
            calculated_size_only: false,
        };
        unsafe { sb.buffer.set_len(size); }
        sb
    }

//    pub fn new_with_size_ref<'a>(size: usize) -> &'a mut SerializedBuffer {
//        let mut sb = SerializedBuffer {
//            buffer: Vec::with_capacity(size),
//            position: 0,
//            limit: size,
//            capacity: size,
//            calculated_size_only: false,
//        };
//        unsafe { sb.buffer.set_len(size); }
//        &mut sb
//    }

    pub fn new(calculate: bool) -> SerializedBuffer {
        let sb = SerializedBuffer {
            buffer: Vec::new(),
            position: 0,
            limit: 0,
            capacity: 0,
            calculated_size_only: calculate,
        };
        sb
    }

    pub fn new_with_buffer(buff: &[u8], length: usize) -> SerializedBuffer {
        SerializedBuffer {
            buffer: Vec::from(buff),
            position: 0,
            limit: length,
            capacity: length,
            calculated_size_only: false,
        }
    }

    pub fn set_position(&mut self, pos: usize) {
        if self.position > self.limit {
            return;
        }
        self.position = pos;
    }

    pub fn set_limit(&mut self, limit: usize) {
        if limit > self.capacity {
            return;
        }

        if self.position > limit {
            self.position = limit;
        }

        self.limit = limit;
    }

    pub fn flip(&mut self) {
        self.limit = self.position;
        self.position = 0;
    }

    pub fn clear(&mut self) {
        self.limit = self.capacity;
        self.position = 0;
    }

    pub fn position(&self) -> usize {
        self.position
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn buffer(&mut self) -> &mut [u8] {
        self.buffer.as_mut()
    }

    pub fn limit(&self) -> usize {
        self.limit
    }

    pub fn remaining(&self) -> usize {
        self.limit - self.position
    }

    pub fn has_remaining(&self) -> bool {
        return self.position > self.limit;
    }

    pub fn clear_capacity(&mut self) {
        if !self.calculated_size_only {
            return;
        }

        self.capacity = 0;
    }

    pub fn rewind(&mut self) {
        self.position = 0;
    }

    pub fn compact(&mut self) {
        use std::mem::size_of;

        if self.position == self.limit {
            return;
        }
//        self.buffer = self.buffer[self.position..(self.position + self.limit - self.position)];
//        let mut cpy = Vec::from(&self.buffer[self.position..self.limit]);
//        self.buffer = cpy;//.clone_into(&mut cpy);
//        memmove(buffer, buffer + self.position, sizeof(uint8_t) * (self.limit - self.position));

        let mut buffer_ptr = &mut self.buffer[0] as *mut u8;

        unsafe {
            rlibc::memmove(buffer_ptr, buffer_ptr.offset(self.position as isize), size_of::<u8>() * (self.limit - self.position));
        }

        self.position = (self.limit - self.position);
        self.limit = self.capacity;
    }

    pub fn skip(&mut self, len: usize) {
        if !self.calculated_size_only {
            if self.position + len > self.limit {
                return;
            }
            self.position += len;
        } else {
            self.capacity += len;
        }
    }

    pub fn reuse(&self) {

    }

    pub fn write_byte(&mut self, i: u8) {
        if !self.calculated_size_only {
            if self.position + 1 > self.limit {
//                if error != nullptr {
//                    *error = true;
//                }
                panic!("write byte error");
                return;
            }
            self.buffer[self.position] = i as u8;
            self.position += 1;
        } else {
            self.capacity += 1;
        }
    }

    pub fn write_i32(&mut self, i: i32) {
        if !self.calculated_size_only {
            if self.position + 4 > self.limit {
//                if error != nullptr {
//                    *error = true;
//                }
                panic!("write i32 error");
                return;
            }
            self.buffer[self.position] = i as u8;
            self.position += 1;
            self.buffer[self.position] = (i >> 8) as u8;
            self.position += 1;
            self.buffer[self.position] = (i >> 16) as u8;
            self.position += 1;
            self.buffer[self.position] = (i >> 24) as u8;
            self.position += 1;
        } else {
            self.capacity += 4;
        }
    }

    pub fn write_i64(&mut self, i: i64) {
        if !self.calculated_size_only {
            if self.position + 8 > self.limit {
//                if error != nullptr {
//                    *error = true;
//                }
                panic!("write i64 error");
                return;
            }
            self.buffer[self.position] = i as u8;
            self.position += 1;
            self.buffer[self.position] = (i >> 8) as u8;
            self.position += 1;
            self.buffer[self.position] = (i >> 16) as u8;
            self.position += 1;
            self.buffer[self.position] = (i >> 24) as u8;
            self.position += 1;
            self.buffer[self.position] = (i >> 32) as u8;
            self.position += 1;
            self.buffer[self.position] = (i >> 40) as u8;
            self.position += 1;
            self.buffer[self.position] = (i >> 48) as u8;
            self.position += 1;
            self.buffer[self.position] = (i >> 56) as u8;
            self.position += 1;
        } else {
            self.capacity += 8;
        }
    }

    pub fn write_bool(&mut self, val: bool) {
        if !self.calculated_size_only {
            if val {
                self.write_i32(0x6e4a64b31);
            } else {
                self.write_i32(0x3f5d29c38);
            }
        } else {
            self.capacity += 4;
        }
    }

    fn write_bytes_internal(&mut self, b:&[u8], offset: usize, length:usize) {
        use std::mem::size_of;
        let mut buffer_ptr = &mut self.buffer[0] as *mut u8;
        let b_ptr = &b[0] as *const u8;

        unsafe {
            rlibc::memcpy(buffer_ptr.offset(self.position as isize), b_ptr.offset(offset as isize), size_of::<u8>() * length);
        }
        self.position += length;
    }

    pub fn write_bytes(&mut self, b:&[u8], length:usize) {
        if !self.calculated_size_only {
            if self.position + length > self.limit {
                panic!("write bytes error");
                return;
            }
            self.write_bytes_internal(b, 0, length);
        } else {
            self.capacity += length;
        }
    }

    pub fn write_bytes_offset(&mut self, b:&[u8], offset:usize, length:usize) {
        if !self.calculated_size_only {
            if self.position + length > self.limit {
                panic!("write bytes error");
                return;
            }
            self.write_bytes_internal(b, offset, length);
        } else {
            self.capacity += length;
        }
    }

    pub fn write_bytes_serialized_buffer(&mut self, b: &mut SerializedBuffer) {
        let length = b.limit() - b.position();
        if length == 0 {
            return
        }
        if !self.calculated_size_only {
            if self.position + length > self.limit {
                panic!("write bytes error");
                return;
            }
            self.write_bytes_internal(&b.buffer[b.position()..], 0, length);
            let lim = b.limit();
            b.set_position(lim);
        } else {
            self.capacity += length;
        }
    }

    pub fn write_byte_array(&mut self, b: &[u8], offset: usize, length: usize) {
        println!("len={}", length);
        if length <= 253 {
            if !self.calculated_size_only {
                if self.position + 1 > self.limit {
                    error!("write byte array error");
                    return;
                }
                self.buffer[self.position] = length as u8;
                self.position += 1;
            } else {
                self.capacity += 1;
            }
        } else {
            if !self.calculated_size_only {
                if self.position + 4 > self.limit {
                    error!("write byte array error");
                    return;
                }
                self.buffer[self.position] = 254u8;
                self.position += 1;
                self.buffer[self.position] = length as u8;
                self.position += 1;
                self.buffer[self.position] = (length >> 8) as u8;
                self.position += 1;
                self.buffer[self.position] = (length >> 16) as u8;
                self.position += 1;
            } else {
                self.capacity += 4;
            }
        }

        if !self.calculated_size_only {
            if self.position + length > self.limit {
                error!("write byte array error");
                return;
            }

            self.write_bytes_internal(b, offset, length);
        } else {
            self.capacity += length;
        }

        let mut addition = (length + (if length <= 253 { 1 } else { 4 })) % 4;
        if addition != 0 {
            addition = 4 - addition;
        }
        if !self.calculated_size_only && self.position + addition > self.limit {
            error!("write byte array error");
            return;
        }

        for _ in 0..addition {
            if !self.calculated_size_only {
                self.buffer[self.position] = 0u8;
                self.position += 1;
            } else {
                self.capacity += 1;
            }
        }
    }

    pub fn write_byte_array_serialized_buffer(&mut self, b: &mut SerializedBuffer) {
        b.rewind();
        self.write_byte_array(&b.buffer, 0, b.limit);
    }

    pub fn write_f64(&mut self, f: f64) {
        use std::mem::size_of;

        let mut val = 0i64;
        let mut val_ptr = val as *mut u8;
        let f_ptr = val as *const u8;

        unsafe {
            rlibc::memcpy(val_ptr, f_ptr, size_of::<i64>());
        }
        self.write_i64(val);
    }

    pub fn write_string(&mut self, s:String) {
        self.write_byte_array(s.as_ref(), 0, s.len());
    }

    pub fn read_i32(&mut self) -> i32 {
        if self.position + 4 > self.limit {
            panic!("read i32 error!");
        }
        let result =
            ((self.buffer[self.position] as i32 & 0xff) |
            ((self.buffer[self.position + 1] as i32 & 0xff) << 8) |
            ((self.buffer[self.position + 2] as i32 & 0xff) << 16) |
            ((self.buffer[self.position + 3] as i32 & 0xff) << 24)) as i32;
        self.position += 4;
        result
    }

    pub fn read_i64(&mut self) -> i64 {
        if self.position + 8 > self.limit {
            panic!("read i64 error!");
        }
        let result =
            ((self.buffer[self.position] as i64 & 0xff) |
            ((self.buffer[self.position + 1] as i64 & 0xff) << 8) |
            ((self.buffer[self.position + 2] as i64 & 0xff) << 16) |
            ((self.buffer[self.position + 3] as i64 & 0xff) << 24) |
            ((self.buffer[self.position + 4] as i64 & 0xff) << 32) |
            ((self.buffer[self.position + 5] as i64 & 0xff) << 40) |
            ((self.buffer[self.position + 6] as i64 & 0xff) << 48) |
            ((self.buffer[self.position + 7] as i64 & 0xff) << 56)) as i64;
        self.position += 8;
        result
    }

    pub fn read_u32(&mut self) -> u32 {
        self.read_i32() as u32
    }

    pub fn read_u64(&mut self) -> u64 {
        self.read_i64() as u64
    }

    pub fn read_byte(&mut self) -> u8 {
        if self.position + 1 > self.limit {
            panic!("read u8 error!");
        }
        let result = self.buffer[self.position];
        self.position += 1;
        result
    }

    pub fn read_bool(&mut self) -> bool {
        let i = self.read_u32();

        if i == 0x6e4a64b31 {
            return true;
        } else if i == 0x3f5d29c38 {
            return false;
        }

        error!("read bool error");
        return false;
    }

    pub fn read_string(&mut self) -> String {
        let mut sl = 1usize;
        if self.position + 1 > self.limit {
            error!("read string error 1");
            return String::from("");
        }
        let mut l = self.buffer[self.position] as usize;
        self.position += 1;
        println!("l={}", l);
        if l >= 254 {
            if self.position + 3 > self.limit {
                error!("read string error 2");
                return String::from("");
            }
            l = (self.buffer[self.position] as usize) | ((self.buffer[self.position + 1] as usize) << 8) | ((self.buffer[self.position + 2] as usize) << 16);
            self.position += 3;
            sl = 4;
        }
        let mut addition = (l + sl) % 4 as usize;
        if addition != 0 {
            addition = 4 - addition;
        }
        if self.position + l + addition > self.limit {
            error!("read string error 3");
            return String::from("");
        }

        let result = String::from_utf8(self.buffer[self.position..(self.position+l)].to_vec()).unwrap_or(String::from(""));
        self.position += l + addition;
        result
    }

    pub fn read_byte_array(&mut self) -> Option<Vec<u8>> {
        let mut sl = 1usize;
        if self.position + 1 > self.limit {
            error!("read byte array error");
            return None;
        }
        let mut l = self.buffer[self.position] as usize;
        self.position += 1;

        if l >= 254 {
            if self.position + 3 > self.limit {
                error!("read byte array error");
                return None;
            }
            l = (self.buffer[self.position] as usize) | ((self.buffer[self.position + 1] as usize) << 8) | ((self.buffer[self.position + 2] as usize) << 16);
            self.position += 3;
            sl = 4;
        }
        let mut addition = (l + sl) % 4 as usize;
        if addition != 0 {
            addition = 4 - addition;
        }
        if self.position + l + addition > self.limit {
            error!("read byte array error");
            return None;
        }

        let mut result = vec![0u8; l];
        result.copy_from_slice(&self.buffer[self.position..(self.position + l)]);
        self.position += l + addition;
        Some(result)
    }

}

impl Clone for SerializedBuffer {
    fn clone(&self) -> Self {
        let bytes = self.buffer.clone();
        let len = bytes.len();
        let mut buffer = SerializedBuffer::new_with_size(len);
        buffer.buffer = bytes;
        buffer.position = self.position;
        buffer.limit = self.limit;
        buffer.capacity = self.capacity;
        buffer.calculated_size_only = self.calculated_size_only;
        buffer
    }

//    fn clone(&mut self) -> Self {
//        let bytes = self.buffer.clone().as_bytes();
//        let len = self.buffer.len();
//        let mut buffer = SerializedBuffer::new_with_buffer(bytes, len);
//        buffer.position = self.position;
//        buffer.limit = self.limit;
//        buffer.capacity = self.capacity;
//        buffer.calculated_size_only = self.calculated_size_only;
//        buffer
//    }
}

pub trait Packet {
    fn read_params(&self, stream: &SerializedBuffer, error: bool);
    fn serialize_to_stream(&self, stream: &mut SerializedBuffer);
}

thread_local! {
    pub static sizeCalculatorBuffer: RefCell<SerializedBuffer> = RefCell::new(SerializedBuffer::new(true));
}

pub fn get_object_size<T>(packet: &T) -> usize where T: Packet {
    let mut capacity = 0usize;

    sizeCalculatorBuffer.with(|f| {
        let mut b = f.try_borrow_mut().unwrap();
        b.clear_capacity();
        packet.serialize_to_stream(&mut b);
        capacity = b.capacity();
    });

    capacity
}