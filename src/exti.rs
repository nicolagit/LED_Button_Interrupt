pub mod gpio
{
    pub enum EdgeTrigger {
        Rising,
        Falling,
    }

    pub  fn set_edge(pin: u32, edge: EdgeTrigger) {
        // TODO
    }
}
