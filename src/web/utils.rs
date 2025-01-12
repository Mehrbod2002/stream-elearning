pub fn encode_request(protobuf_message: &[u8]) -> Vec<u8> {
    let mut body = Vec::new();
    body.push(0);
    let len = protobuf_message.len() as u32;
    body.extend(&len.to_be_bytes());
    body.extend(protobuf_message);
    body
}

pub fn decode_request(data: &[u8]) -> Result<Vec<u8>, String> {
    if data.is_empty() {
        return Err("Empty gRPC-Web response".to_string());
    }

    if data[0] != 0x00 {
        return Err(format!("Unexpected frame type: {}", data[0]));
    }

    if data.len() < 5 {
        return Err("Incomplete gRPC-Web frame".to_string());
    }

    let len = u32::from_be_bytes(data[1..5].try_into().unwrap()) as usize;

    if data.len() < 5 + len {
        return Err(format!(
            "Incomplete payload: expected {} bytes, got {}",
            len,
            data.len() - 5
        ));
    }

    let payload = data[5..5 + len].to_vec();

    if data.len() > 5 + len {
        let trailers_start = 5 + len;
        if data[trailers_start] == 0x80 {
            println!("Trailers detected: {:?}", &data[trailers_start..]);
        }
    }

    Ok(payload)
}
