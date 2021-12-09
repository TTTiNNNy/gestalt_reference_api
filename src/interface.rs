//
// pub trait RawInterface<F:FnOnce<()>>
// {
//     fn write(&self, func: F);
//     fn read(&self, func: F);
// }

// pub trait ProtocolInerface<F:FnOnce<()>>
// {
//     fn write(&self, func: F);
//     fn read(&self, func: F);
// }

pub trait  GenericInterface
{
    fn write(&mut self);
    fn read(&mut self);
}