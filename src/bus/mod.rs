use core::default::Default;

pub trait Bus {
    type Error : Default;

    fn lock(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn unlock(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn send(&mut self, _data: u8) -> Result<(), Self::Error> {
        Err( Default::default() )
    }

    fn recv(&mut self) -> Result<u8, Self::Error> {
        Err( Default::default() )
    }

    fn transfer<'a>(&mut self, tx: &'a [u8], rx: &'a mut [u8]) -> Result<(), Self::Error> {
        if tx.len() != rx.len() {
            return Err( Default::default() );
        }

        for (tx_byte, rx_byte) in tx.iter().zip(rx.iter_mut()) {
            self.send(*tx_byte)?;
            *rx_byte = self.recv()?;
        }

        Ok(())
    }
}
