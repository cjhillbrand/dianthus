use std::error::Error;

pub fn to_vec<'a, I>(data: &mut I, length: usize) -> Result<Vec<u8>, Box<dyn Error>>
    where I: Iterator<Item=&'a u8>
{
    let mut result: Vec<u8> = Vec::new();
    for _i in 0..length
    {
        result.push(*data.next().unwrap());
    }

    return Ok(result);
}

pub fn to_u64<'a, I>(data: &mut I) -> Result<u64, Box<dyn Error>>
    where I: Iterator<Item=&'a u8>
{
    let mut result: u64 = 0;
    result = *data.next().unwrap() as u64 + result;
    for _i in 0..7
    {
        result = *data.next().unwrap() as u64 + (result << 8);
    }

    return Ok(result);
}

pub fn to_u32<'a, I>(data: &mut I) -> Result<u32, Box<dyn Error>>
    where I: Iterator<Item=&'a u8>
{
    let mut result: u32 = 0;
    result = *data.next().unwrap() as u32 + result;
    for _i in 0..3
    {
        result = *data.next().unwrap() as u32 + (result << 8);
    }

    return Ok(result);
}

pub fn to_u16<'a, I>(data: &mut I) -> Result<u16, Box<dyn Error>>
    where I: Iterator<Item=&'a u8>
{
    let mut result: u16 = 0;
    result = *data.next().unwrap() as u16 + result;
    result = *data.next().unwrap() as u16 + (result << 8);
    return Ok(result);
}

pub fn to_u8<'a, I>(data: &mut I) -> Result<u8, Box<dyn Error>>
    where I: Iterator<Item=&'a u8>
{
    Ok(*data.next().unwrap())
}



