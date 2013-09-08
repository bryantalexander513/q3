/*
    Copyright 2013 Jesse 'Jeaye' Wilkerson
    See licensing in LICENSE file, or at:
        http://www.opensource.org/licenses/BSD-3-Clause

    File: util/log_macros.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      Usage of logging utilities is generally
      by means of the macros: log_debug!() and
      log_error!().
*/

macro_rules! log_debug
(
  ($message:expr) => 
  ({
    let module = Log::get_module(file!());
    Log::debug(fmt!("[%s]:%u", module, line!()), $message);
  });
  ($($message:expr),+) =>
  ({
    let module = Log::get_module(file!());
    Log::debug(fmt!("[%s]:%u", module, line!()), fmt!($($message),+));
  });
)

macro_rules! log_info
(
  ($message:expr) => 
  ({
    let module = Log::get_module(file!());
    Log::info(fmt!("[%s]:%u", module, line!()), $message);
  });
  ($($message:expr),+) =>
  ({
    let module = Log::get_module(file!());
    Log::info(fmt!("[%s]:%u", module, line!()), fmt!($($message),+));
  });
)

macro_rules! log_error
(
  ($message:expr) => 
  ({
    let module = Log::get_module(file!());
    Log::error(fmt!("[%s]:%u", module, line!()), $message);
  });
  ($($message:expr),+) =>
  ({
    let module = Log::get_module(file!());
    Log::error(fmt!("[%s]:%u", module, line!()), fmt!($($message),+));
  });
)

macro_rules! log_push
(
  () => 
  ({
    Log::push();
  });
)

macro_rules! log_pop
(
  () => 
  ({
    Log::pop();
  });
)

macro_rules! log_fail
(
  ($message:expr) => 
  ({
    let module = Log::get_module(file!());
    Log::error(fmt!("[%s]:%u", module, line!()), $message);
    Log::error(fmt!("[%s]:%u", module, line!()), "Failing");
    fail!("Exiting");
  });
  ($($message:expr),+) =>
  ({
    let module = Log::get_module(file!());
    Log::error(fmt!("[%s]:%u", module, line!()), fmt!($($message),+));
    Log::error(fmt!("[%s]:%u", module, line!()), "Failing");
    fail!("Exiting");
  });
)

macro_rules! log_assert
(
  ($val:expr) => 
  ({
    if !$val
    { log_fail!(fmt!("Assertion failed: (%s)", stringify!($val))); }
  });
  ($val:expr, $message:expr) => 
  ({
    if !$val
    { log_fail!($message); }
  });
  ($val:expr, $($message:expr),+) =>
  ({
    if !$val
    { log_fail!(fmt!($($message),+)); }
  });
)

