// Copyright (c) 2022 Jan Holthuis
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy
// of the MPL was not distributed with this file, You can obtain one at
// http://mozilla.org/MPL/2.0/.
//
// SPDX-License-Identifier: MPL-2.0

//! Parser for Rekordbox `*SETTING.DAT` files.

use nom::IResult;

#[derive(Debug)]
/// TODO
pub struct Setting {
    /// Size of the string data field (should be always 96).
    pub len_stringdata: u32,
    /// Name of the company ("PIONEER").
    pub company: String,
    /// Name of the software ("rekordbox").
    pub software: String,
    /// Some kind of version number.
    pub version: String,
    /// Size of the `unknown1` data in bytes.
    pub len_unknown1: u32,
    /// Unknon field.
    pub unknown1: Vec<u8>,
    /// Unknon field.
    pub unknown2: u32,
}

impl Setting {
    /// Parses the Setting file and returns the structure.
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, len_stringdata) = nom::number::complete::le_u32(input)?;
        let len_stringdatasection = (len_stringdata as usize) / 3;
        let (input, company) = nom::bytes::complete::take(len_stringdatasection)(input)?;
        let company = std::str::from_utf8(company).unwrap().trim_end_matches('\0').to_owned();
        let (input, software) = nom::bytes::complete::take(len_stringdatasection)(input)?;
        let software = std::str::from_utf8(software).unwrap().trim_end_matches('\0').to_owned();
        let (input, version) = nom::bytes::complete::take(len_stringdatasection)(input)?;
        let version = std::str::from_utf8(version).unwrap().trim_end_matches('\0').to_owned();

        let (input, len_unknown1) = nom::number::complete::le_u32(input)?;
        let (input, unknown1) = nom::bytes::complete::take(len_unknown1 as usize)(input)?;
        let unknown1 = unknown1.to_vec();

        let (input, unknown2) = nom::number::complete::le_u32(input)?;
        assert!(input.is_empty());

        Ok((
            input,
            Self {
                len_stringdata,
                company,
                software,
                version,
                len_unknown1,
                unknown1,
                unknown2,
            },
        ))
    }
}
