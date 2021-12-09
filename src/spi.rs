pub trait GestaltSpi
{
    type MisoPin;
    type MosiPin;
    type ClkPin;
    type Freq;
    type TxBuf;
    type RxBuf;

    fn set_miso     (&self, _: Self::MisoPin);
    fn set_mosi     (&self, _: Self::MosiPin);
    fn set_scl      (&self, _: Self::ClkPin);
    fn set_freq     (&self, _: Self::Freq);
    fn set_tx_buf   (&self, _: &[Self::TxBuf]);
    fn set_rx_buf   (&self, _: &[Self::RxBuf]);

}