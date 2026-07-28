#![allow(non_camel_case_types)]

use std::sync::LazyLock;

use bc_signals::prelude::*;
use bc_signals::{
    change_signal::CHANGE_SIGNAL, change_src::CHANGE_SRC, convert::CONVERT, copy::COPY,
    filter::FILTER, invert::INVERT, th::TH, repeat::REPEAT,
    set_probability::SET_PROBABILITY,
};
use bc_utils_lg::types::maps::{PACK};

use bc_utils_lg::structs::settings::SETTINGS_SIGNAL;

pub type PACK_TYPE_SIGN = PACK<SETTINGS_SIGNAL, Box<dyn SignalReady>>;

pub static PACK: LazyLock<PACK_TYPE_SIGN> = LazyLock::new(|| {
        MAP::from_iter([
            (
                "th",
                (|setting: &SETTINGS_SIGNAL| {
                    let mut df = TH::default();
                    df.params.th_min = setting.kwargs_f64.get("th_min").copied().unwrap_or(df.params.th_min);
                    df.params.th_max = setting.kwargs_f64.get("th_max").copied().unwrap_or(df.params.th_max);
                    df.params.limit = setting.kwargs_f64.get("limit").copied().unwrap_or(df.params.limit);
                    df.params.index_min = 
                        setting
                            .kwargs_usize
                            .get("index_min")
                            .copied()
                            .unwrap_or(df.params.index_min);
                    df.params.index_max = 
                        setting
                            .kwargs_usize
                            .get("index_max")
                            .copied().unwrap_or(df.params.index_max);
                    df.params.index_normal = 
                        setting
                            .kwargs_usize
                            .get("index_normal")
                            .copied().unwrap_or(df.params.index_normal);
                    Box::new(df) as Box<dyn SignalReady>
                }) as fn(&SETTINGS_SIGNAL) -> Box<dyn SignalReady>,
            ),
            (
                "set_probability",
                (|_: &SETTINGS_SIGNAL| Box::new(SET_PROBABILITY) as Box<dyn SignalReady>),
            ),
            (
                "change_signal",
                (|_: &SETTINGS_SIGNAL| Box::new(CHANGE_SIGNAL::default()) as Box<dyn SignalReady>),
            ),
            (
                "change_src",
                (|setting: &SETTINGS_SIGNAL| {
                    let mut df = CHANGE_SRC::default();
                    df.params.signal_short =
                        setting
                            .kwargs_f64
                            .get("signal_short")
                            .copied().unwrap_or(df.params.signal_short);
                    df.params.signal_long = 
                        setting
                            .kwargs_f64
                            .get("signal_long")
                            .copied().unwrap_or(df.params.signal_long);
                    df.params.signal_hold = 
                        setting
                            .kwargs_f64
                            .get("signal_hold")
                            .copied().unwrap_or(df.params.signal_hold);
                    Box::new(df) as Box<dyn SignalReady>
                }),
            ),
            (
                "convert",
                (|_: &SETTINGS_SIGNAL| Box::new(CONVERT) as Box<dyn SignalReady>),
            ),
            (
                "invert",
                (|setting: &SETTINGS_SIGNAL| {
                    let mut df = INVERT::default();
                    df.signal_short = 
                        setting
                            .kwargs_f64
                            .get("signal_short")
                            .copied().unwrap_or(df.signal_short);
                    df.signal_long = 
                        setting
                            .kwargs_f64
                            .get("signal_long")
                            .copied().unwrap_or(df.signal_long);
                    df.signal_hold = 
                        setting
                            .kwargs_f64
                            .get("signal_hold")
                            .copied().unwrap_or(df.signal_hold);
                    Box::new(df) as Box<dyn SignalReady>
                }),
            ),
            (
                "filter",
                (|_: &SETTINGS_SIGNAL| Box::new(FILTER) as Box<dyn SignalReady>),
            ),
            (
                "copy",
                (|_: &SETTINGS_SIGNAL| Box::new(COPY) as Box<dyn SignalReady>),
            ),
            (
                "repeat",
                (|setting: &SETTINGS_SIGNAL| {
                    let mut df = REPEAT::default();
                    df.value_signal = 
                        setting
                            .kwargs_f64
                            .get("value_signal")
                            .copied().unwrap_or(df.value_signal);
                    df.value_probability = 
                        setting
                            .kwargs_f64
                            .get("value_probability")
                            .copied().unwrap_or(df.value_probability);
                    Box::new(df) as Box<dyn SignalReady>
                }),
            ),
        ])
});
