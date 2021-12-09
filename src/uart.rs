use crate::generic::GenericToUsize;

pub trait GenericUart
{
    type TxPin;
    type RxPin;
    type Baud;
    type TxBuf;
    type RxBuf;

    fn set_rx       (&self, _: Self::RxPin);
    fn set_tx       (&self, _: Self::TxPin);
    fn set_baud     (&self, _: Self::Baud);
    fn set_tx_buf   (&self, _: &[Self::TxBuf]);
    fn set_rx_buf   (&self, _: &[Self::RxBuf]);
}

#[derive(Clone, Copy, PartialEq)]
pub enum GenericUartStatus
{
    Init,
    Write,
    Read,
    Idle
}

#[derive(Clone, Copy, PartialEq)]
pub enum FullGenericUartStatus<T: Clone + Copy> {
    Other(T),
    generic_uart_status(GenericUartStatus)
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub enum GenericUartInterrupt{
    Rx,
    Tx,
}

pub enum FullGenericUartInterrupt<T>{
    generic_uart_interrupt(GenericUartInterrupt),
    Other(T)
}

