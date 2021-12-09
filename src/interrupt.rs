pub trait  InterruptHandle
{
    type Interrupts;
    type Pin;
    fn set_interrupt(&mut self, interrupt: Self::Interrupts, pin: Self::Pin);
    fn clear_interrupt(&mut self, _: Self::Pin);

}
