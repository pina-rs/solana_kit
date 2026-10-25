//! Count-prefix codecs preflight the known minimum byte extent before allocation.
pub(super) const SOURCE: &str = r#"
struct CheckedCounted<P,C> { prefix:P, item:C, minimum:usize }
impl<P:Encoder<u32>,C:Encoder<T>,T> Encoder<[T]> for CheckedCounted<P,C> {
fn encoded_len(&self,value:&[T])->Result<usize,Error> {
Counted {prefix:&self.prefix,item:&self.item}.encoded_len(value)
}
fn write(&self,value:&[T],writer:&mut Writer<'_>)->Result<(),Error> {
Counted {prefix:&self.prefix,item:&self.item}.write(value,writer)
}
}
impl<P:Decoder<u32>,C:Decoder<T>,T> Decoder<Vec<T>> for CheckedCounted<P,C> {
fn read(&self,reader:&mut Reader<'_>)->Result<Vec<T>,Error> {
let count=usize::try_from(self.prefix.read(reader)?).map_err(|_|Error::Overflow)?;
let minimum=count.checked_mul(self.minimum).ok_or(Error::Overflow)?;

if minimum>reader.remaining().len() {return Err(Error::Truncated);}

solana_kit_core::codecs::Sequence {item:&self.item,count:solana_kit_core::codecs::Count::Fixed(count)}.read(reader)
}
}
"#;
