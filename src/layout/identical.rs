// Copyright 2024 FastLabs Developers
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

use std::fmt::Arguments;

use crate::layout::Layout;

/// A layout that returns log record as is.
///
/// This is mainly used as the default implementation for
/// [`Append::default_layout`][crate::append::Append::default_layout].
#[derive(Debug, Default, Clone, Copy)]
pub struct IdenticalLayout;

impl IdenticalLayout {
    pub(crate) fn format<F>(&self, record: &log::Record, f: &F) -> anyhow::Result<()>
    where
        F: Fn(Arguments) -> anyhow::Result<()>,
    {
        f(*record.args())
    }
}

impl From<IdenticalLayout> for Layout {
    fn from(layout: IdenticalLayout) -> Self {
        Layout::Identical(layout)
    }
}
