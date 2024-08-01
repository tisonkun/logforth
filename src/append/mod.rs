// Copyright 2024 tison <wander4096@gmail.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::dynlog::DynLog;
use crate::filter::FilterImpl;
use crate::layout;
use crate::layout::LayoutImpl;
pub use boxdyn::*;
#[cfg(feature = "fastrace")]
pub use fastrace::*;
#[cfg(feature = "file")]
pub use file::*;
pub use stdio::*;

mod boxdyn;
#[cfg(feature = "fastrace")]
mod fastrace;
#[cfg(feature = "file")]
mod file;
mod stdio;

pub trait Append {
    /// Dispatches a log record to the append target.
    fn try_append(&self, record: &log::Record) -> anyhow::Result<()>;

    /// Flushes any buffered records.
    fn flush(&self) {}

    /// Default layout to use when [Dispatch][crate::logger::Dispatch] does not configure a
    /// preferred layout.
    fn default_layout(&self) -> LayoutImpl {
        LayoutImpl::Identical(layout::Identical)
    }

    /// Default filters associated to this append. [log::Log] is mixed with
    /// [Filter][crate::filter::Filter] and [Append].
    fn default_filters(&self) -> Option<Vec<FilterImpl>> {
        None
    }
}

#[derive(Debug)]
pub enum AppendImpl {
    BoxDyn(BoxDyn),
    DynLog(DynLog),
    #[cfg(feature = "fastrace")]
    Fastrace(Fastrace),
    #[cfg(feature = "file")]
    RollingFile(RollingFile),
    Stdout(Stdout),
    Stderr(Stderr),
}

impl Append for AppendImpl {
    fn try_append(&self, record: &log::Record) -> anyhow::Result<()> {
        match self {
            AppendImpl::BoxDyn(append) => append.try_append(record),
            AppendImpl::DynLog(append) => append.try_append(record),
            #[cfg(feature = "fastrace")]
            AppendImpl::Fastrace(append) => append.try_append(record),
            #[cfg(feature = "file")]
            AppendImpl::RollingFile(append) => append.try_append(record),
            AppendImpl::Stdout(append) => append.try_append(record),
            AppendImpl::Stderr(append) => append.try_append(record),
        }
    }

    fn flush(&self) {
        match self {
            AppendImpl::BoxDyn(append) => append.flush(),
            AppendImpl::DynLog(append) => append.flush(),
            #[cfg(feature = "fastrace")]
            AppendImpl::Fastrace(append) => append.flush(),
            #[cfg(feature = "file")]
            AppendImpl::RollingFile(append) => append.flush(),
            AppendImpl::Stdout(append) => append.flush(),
            AppendImpl::Stderr(append) => append.flush(),
        }
    }

    fn default_layout(&self) -> LayoutImpl {
        match self {
            AppendImpl::BoxDyn(append) => append.default_layout(),
            AppendImpl::DynLog(append) => append.default_layout(),
            #[cfg(feature = "fastrace")]
            AppendImpl::Fastrace(append) => append.default_layout(),
            #[cfg(feature = "file")]
            AppendImpl::RollingFile(append) => append.default_layout(),
            AppendImpl::Stdout(append) => append.default_layout(),
            AppendImpl::Stderr(append) => append.default_layout(),
        }
    }

    fn default_filters(&self) -> Option<Vec<FilterImpl>> {
        match self {
            AppendImpl::BoxDyn(append) => append.default_filters(),
            AppendImpl::DynLog(append) => append.default_filters(),
            #[cfg(feature = "fastrace")]
            AppendImpl::Fastrace(append) => append.default_filters(),
            #[cfg(feature = "file")]
            AppendImpl::RollingFile(append) => append.default_filters(),
            AppendImpl::Stdout(append) => append.default_filters(),
            AppendImpl::Stderr(append) => append.default_filters(),
        }
    }
}
