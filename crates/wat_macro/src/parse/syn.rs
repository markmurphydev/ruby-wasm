mod parse_buffer;
mod token_buffer;

pub type ParseStream<'a> = &'a ParseBuffer<'a>;

