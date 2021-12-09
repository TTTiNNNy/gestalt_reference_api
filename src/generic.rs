pub trait GenericEnable
{
    fn enable   (&self, _: bool);
}

pub trait GenericStatus
{
    type Status;

    fn get_status (&self) -> Self::Status;
}

pub trait GenericNew
{
    type Data;

    fn new(data: Self::Data) -> Self;
}

pub trait GenericToUsize{
    fn to_usize() -> usize;
}

pub trait GenericEvent
{
    type Event;
    fn is_event_active(&self, status: Self::Event) -> bool;
    fn flush_event(&self, status: Self::Event);
}