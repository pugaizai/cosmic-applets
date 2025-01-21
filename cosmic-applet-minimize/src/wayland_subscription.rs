// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

//! # DBus interface proxy for: `org.freedesktop.UPower.KbdBacklight`
//!
//! This code was generated by `zbus-xmlgen` `2.0.1` from DBus introspection data.
//! Source: `Interface '/org/freedesktop/UPower/KbdBacklight' from service 'org.freedesktop.UPower' on system bus`.
use cctk::{sctk::reexports::calloop, toplevel_info::ToplevelInfo};
use cosmic::{
    cctk::{self, cosmic_protocols},
    iced::{self, Subscription},
    iced_core::image::Bytes,
    iced_futures::{futures, stream},
};
use cosmic_protocols::toplevel_info::v1::client::zcosmic_toplevel_handle_v1::ZcosmicToplevelHandleV1;
use futures::SinkExt;
use image::EncodableLayout;
use std::fmt::Debug;

use crate::wayland_handler::wayland_handler;

pub fn wayland_subscription() -> iced::Subscription<WaylandUpdate> {
    Subscription::run_with_id(
        std::any::TypeId::of::<WaylandUpdate>(),
        stream::channel(1, move |mut output| async move {
            let (calloop_tx, calloop_rx) = calloop::channel::channel();
            let runtime = tokio::runtime::Handle::current();

            let _ = std::thread::spawn(move || {
                runtime.block_on(async move {
                    _ = output.send(WaylandUpdate::Init(calloop_tx)).await;
                    wayland_handler(output.clone(), calloop_rx);
                    tracing::error!("Wayland handler thread died");
                    _ = output.send(WaylandUpdate::Finished).await;
                });
            });

            futures::future::pending().await
        }),
    )
}

#[derive(Clone, Debug)]
pub enum WaylandUpdate {
    Init(calloop::channel::Sender<WaylandRequest>),
    Finished,
    Toplevel(ToplevelUpdate),
    Image(ZcosmicToplevelHandleV1, WaylandImage),
}

#[derive(Debug, Clone)]
pub struct WaylandImage {
    pub img: Bytes,
    pub width: u32,
    pub height: u32,
}

impl WaylandImage {
    pub fn new(img: image::RgbaImage) -> Self {
        Self {
            img: Bytes::copy_from_slice(img.as_bytes()),
            width: img.width(),
            height: img.height(),
        }
    }
}

impl AsRef<[u8]> for WaylandImage {
    fn as_ref(&self) -> &[u8] {
        self.img.as_bytes()
    }
}

#[derive(Clone, Debug)]
pub enum ToplevelUpdate {
    Add(ZcosmicToplevelHandleV1, ToplevelInfo),
    Update(ZcosmicToplevelHandleV1, ToplevelInfo),
    Remove(ZcosmicToplevelHandleV1),
}

#[derive(Clone, Debug)]
pub enum WaylandRequest {
    Toplevel(ToplevelRequest),
}

#[derive(Debug, Clone)]
pub enum ToplevelRequest {
    Activate(ZcosmicToplevelHandleV1),
}
