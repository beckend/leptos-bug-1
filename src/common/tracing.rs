// use tracing_subscriber::fmt::{fmt, time::LocalTime};

pub fn init() -> anyhow::Result<()> {
  // outputs garbage on browser...
  if cfg!(feature = "hydrate") {
    // fmt()
    //   .with_writer(
    //     // To avoid trace events in the browser from showing their
    //     // JS backtrace, which is very annoying, in my opinion
    //     tracing_subscriber_wasm::MakeConsoleWriter::default()
    //       .map_trace_level_to(tracing::Level::DEBUG),
    //   )
    //   .with_timer(LocalTime::rfc_3339())
    //   .pretty()
    //   .init();

    // initializes logging using the `log` crate
    _ = console_log::init_with_level(log::Level::Info);
    console_error_panic_hook::set_once();
  } else {
    // fmt()
    //   .pretty()
    //   .with_thread_names(true)
    //   .with_timer(LocalTime::rfc_3339())
    //   .with_max_level(tracing::Level::TRACE)
    //   .init();

    simple_logger::init_with_level(log::Level::Info).expect("couldn't initialize logging");
  }

  Ok(())
}
