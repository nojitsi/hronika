pub trait Serialize<Input, Output> {
    fn serialize(serializable: &Input) -> Output;
}
