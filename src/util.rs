use std::error::Error;
use std::collections::VecDeque;

pub fn to_vec(data: &mut VecDeque<u8>, length: usize) -> Vec<u8>
{
    let mut result: Vec<u8> = Vec::new();
    for _i in 0..length
    {
        result.push(data.pop_front().unwrap());
    }

    result
}

pub fn to_u64(data: &mut VecDeque<u8>) -> u64
{
    let mut result: u64 = 0;
    result = data.pop_front().unwrap() as u64;
    for _i in 0..7
    {
        result = data.pop_front().unwrap() as u64 + (result << 8);
    }

    result
}

pub fn to_u32(data: &mut VecDeque<u8>) -> u32
{
    let mut result: u32 = 0;
    result = data.pop_front().unwrap() as u32 + result;
    for _i in 0..3
    {
        result = data.pop_front().unwrap() as u32 + (result << 8);
    }

    result
}

pub fn to_u16(data: &mut VecDeque<u8>) -> u16
{
    let mut result: u16 = 0;
    result = data.pop_front().unwrap() as u16 + result;
    result = data.pop_front().unwrap() as u16 + (result << 8);
    result
}

pub fn to_u8(data: &mut VecDeque<u8>) -> u8 { data.pop_front().unwrap() }



