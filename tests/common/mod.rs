use rw_builder::string::AdhocWriter;
use rw_builder::RwBuilder;

pub fn write_and_read_string<B>(builder: B, input: &str) -> std::io::Result<String>
where
    B: RwBuilder,
{
    let string = builder.string();
    string.write_string(input)?;
    Ok(string.to_string())
}

#[allow(dead_code)]
pub fn test_string<B>(builder: B)
where
    B: RwBuilder,
{
    let text = String::from("This text is written from a String and read back into a String.");
    let actual = write_and_read_string(builder, &text).expect("String couldn't be written");
    assert_eq!(actual, text);
}
