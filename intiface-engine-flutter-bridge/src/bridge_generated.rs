#![allow(
  non_camel_case_types,
  unused,
  clippy::redundant_closure,
  clippy::useless_conversion,
  clippy::unit_arg,
  clippy::double_parens,
  non_snake_case,
  clippy::too_many_arguments
)]
// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`@ 1.78.0.

use crate::api::*;
use core::panic::UnwindSafe;
use flutter_rust_bridge::*;
use std::ffi::c_void;
use std::sync::Arc;

// Section: imports

// Section: wire functions

fn wire_run_engine_impl(
  port_: MessagePort,
  args: impl Wire2Api<EngineOptionsExternal> + UnwindSafe,
) {
  FLUTTER_RUST_BRIDGE_HANDLER.wrap(
    WrapInfo {
      debug_name: "run_engine",
      port: Some(port_),
      mode: FfiCallMode::Stream,
    },
    move || {
      let api_args = args.wire2api();
      move |task_callback| run_engine(task_callback.stream_sink(), api_args)
    },
  )
}
fn wire_send_impl(port_: MessagePort, msg_json: impl Wire2Api<String> + UnwindSafe) {
  FLUTTER_RUST_BRIDGE_HANDLER.wrap(
    WrapInfo {
      debug_name: "send",
      port: Some(port_),
      mode: FfiCallMode::Normal,
    },
    move || {
      let api_msg_json = msg_json.wire2api();
      move |task_callback| Ok(send(api_msg_json))
    },
  )
}
fn wire_stop_engine_impl(port_: MessagePort) {
  FLUTTER_RUST_BRIDGE_HANDLER.wrap(
    WrapInfo {
      debug_name: "stop_engine",
      port: Some(port_),
      mode: FfiCallMode::Normal,
    },
    move || move |task_callback| Ok(stop_engine()),
  )
}
fn wire_send_backend_server_message_impl(
  port_: MessagePort,
  msg: impl Wire2Api<String> + UnwindSafe,
) {
  FLUTTER_RUST_BRIDGE_HANDLER.wrap(
    WrapInfo {
      debug_name: "send_backend_server_message",
      port: Some(port_),
      mode: FfiCallMode::Normal,
    },
    move || {
      let api_msg = msg.wire2api();
      move |task_callback| Ok(send_backend_server_message(api_msg))
    },
  )
}
fn wire_get_user_device_configs_impl(
  port_: MessagePort,
  device_config_json: impl Wire2Api<String> + UnwindSafe,
  user_config_json: impl Wire2Api<String> + UnwindSafe,
) {
  FLUTTER_RUST_BRIDGE_HANDLER.wrap(
    WrapInfo {
      debug_name: "get_user_device_configs",
      port: Some(port_),
      mode: FfiCallMode::Normal,
    },
    move || {
      let api_device_config_json = device_config_json.wire2api();
      let api_user_config_json = user_config_json.wire2api();
      move |task_callback| {
        Ok(get_user_device_configs(
          api_device_config_json,
          api_user_config_json,
        ))
      }
    },
  )
}
fn wire_generate_user_device_config_file_impl(
  port_: MessagePort,
  user_config: impl Wire2Api<Vec<ExposedUserDeviceConfig>> + UnwindSafe,
) {
  FLUTTER_RUST_BRIDGE_HANDLER.wrap(
    WrapInfo {
      debug_name: "generate_user_device_config_file",
      port: Some(port_),
      mode: FfiCallMode::Normal,
    },
    move || {
      let api_user_config = user_config.wire2api();
      move |task_callback| Ok(generate_user_device_config_file(api_user_config))
    },
  )
}
// Section: wrapper structs

#[derive(Clone)]
struct mirror_UserConfigDeviceIdentifier(UserConfigDeviceIdentifier);

// Section: static checks

const _: fn() = || {
  let UserConfigDeviceIdentifier = None::<UserConfigDeviceIdentifier>.unwrap();
  let _: String = UserConfigDeviceIdentifier.address;
  let _: String = UserConfigDeviceIdentifier.protocol;
  let _: Option<String> = UserConfigDeviceIdentifier.identifier;
};
// Section: allocate functions

// Section: related functions

// Section: impl Wire2Api

pub trait Wire2Api<T> {
  fn wire2api(self) -> T;
}

impl<T, S> Wire2Api<Option<T>> for *mut S
where
  *mut S: Wire2Api<T>,
{
  fn wire2api(self) -> Option<T> {
    (!self.is_null()).then(|| self.wire2api())
  }
}

impl Wire2Api<bool> for bool {
  fn wire2api(self) -> bool {
    self
  }
}

impl Wire2Api<u16> for u16 {
  fn wire2api(self) -> u16 {
    self
  }
}
impl Wire2Api<u32> for u32 {
  fn wire2api(self) -> u32 {
    self
  }
}
impl Wire2Api<u8> for u8 {
  fn wire2api(self) -> u8 {
    self
  }
}

// Section: impl IntoDart

impl support::IntoDart for ExposedUserConfig {
  fn into_dart(self) -> support::DartAbi {
    vec![self.specifiers.into_dart(), self.configurations.into_dart()].into_dart()
  }
}
impl support::IntoDartExceptPrimitive for ExposedUserConfig {}

impl support::IntoDart for ExposedUserDeviceConfig {
  fn into_dart(self) -> support::DartAbi {
    vec![
      mirror_UserConfigDeviceIdentifier(self.identifier).into_dart(),
      self.name.into_dart(),
      self.display_name.into_dart(),
      self.allow.into_dart(),
      self.deny.into_dart(),
      self.reserved_index.into_dart(),
    ]
    .into_dart()
  }
}
impl support::IntoDartExceptPrimitive for ExposedUserDeviceConfig {}

impl support::IntoDart for ExposedUserDeviceSpecifiers {
  fn into_dart(self) -> support::DartAbi {
    vec![self.websocket.into_dart()].into_dart()
  }
}
impl support::IntoDartExceptPrimitive for ExposedUserDeviceSpecifiers {}

impl support::IntoDart for ExposedWebsocketSpecifier {
  fn into_dart(self) -> support::DartAbi {
    vec![self.names.into_dart()].into_dart()
  }
}
impl support::IntoDartExceptPrimitive for ExposedWebsocketSpecifier {}

impl support::IntoDart for mirror_UserConfigDeviceIdentifier {
  fn into_dart(self) -> support::DartAbi {
    vec![
      self.0.address.into_dart(),
      self.0.protocol.into_dart(),
      self.0.identifier.into_dart(),
    ]
    .into_dart()
  }
}
impl support::IntoDartExceptPrimitive for mirror_UserConfigDeviceIdentifier {}

// Section: executor

support::lazy_static! {
    pub static ref FLUTTER_RUST_BRIDGE_HANDLER: support::DefaultHandler = Default::default();
}

#[cfg(not(target_family = "wasm"))]
#[path = "bridge_generated.io.rs"]
mod io;
#[cfg(not(target_family = "wasm"))]
pub use io::*;
