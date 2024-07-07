
fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let input = rtftp::getmessage("one").unwrap();

    let processed = ddshift::transform(input.as_str());

    let struct_str = format!("{:#?}", processed);
    println!("{}", struct_str);
    let _ = rbmq::send_msg(&struct_str);
    Ok(())    
}