pub trait GestaltTwi
{
    type SdaPin;
    type SclPin;
    type Freq;
    type TxBuf;
    type RxBuf;

    fn set_sda      (&self, _: Self::SdaPin);
    fn set_scl      (&self, _: Self::SclPin);
    fn set_freq     (&self, _: Self::Freq);
    fn set_tx_buf   (&self, _: &[Self::TxBuf]);
    fn set_rx_buf   (&self, _: &[Self::RxBuf]);
    fn set_addr     (&self, _: usize);
    fn set_reg_addr	(&self, _: usize);
}