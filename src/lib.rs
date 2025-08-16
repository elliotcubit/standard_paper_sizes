// Copyright 2025 Elaine Cubit
//
// This file is part of standard_paper_sizes.
//
// standard_paper_sizes is free software: you can redistribute it and/or
// modify it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or (at your
// option) any later version.
//
// standard_paper_sizes is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
// or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for
// more details.
//
// You should have received a copy of the GNU General Public License along with
// this program. If not, see <https://www.gnu.org/licenses/>. 

use std::fmt;
use std::fmt::Display;

#[non_exhaustive]
#[derive(Debug, Copy, Clone)]
pub enum Type {
    // Standard US Sizes
    UsLetter,  // Same as AnsiA
    UsLegal,
    UsLedger,  // Same as AnsiB
    UsTabloid, // Same as AnsiB
    UsExecutive,
    UsJuniorLegal,
    UsHalfLetter,
    UsGovernmentLetter,
    UsGovernmentLegal,

    // US Envelope Sizes
    UsEnvelope6_25,
    UsEnvelope6_75,
    UsEnvelope7,
    UsEnvelope7_75Monarch,
    UsEnvelope8_625,
    UsEnvelope9,
    UsEnvelope10,
    UsEnvelope11,
    UsEnvelope12,
    UsEnvelope14,
    UsEnvelope16,
    UsEnvelopeA1,
    UsEnvelopeA2LadyGrey,
    UsEnvelopeA4,
    UsEnvelopeA6Thompsons,
    UsEnvelopeA7Besselheim,
    UsEnvelopeA8Carrs,
    UsEnvelopeA9Diplomat,
    UsEnvelopeA10Willow,
    UsEnvelopeALong,
    UsEnvelope1,
    UsEnvelope1_75,
    UsEnvelope3,
    UsEnvelope6,
    UsEnvelope8,
    UsEnvelope9_75,
    UsEnvelope10_5,
    UsEnvelope12_5,
    UsEnvelope13_5,
    UsEnvelope14_5,
    UsEnvelope15,
    UsEnvelope15_5,

    // ISO Envelopes
    IsoEnvelopeDL,
    IsoEnvelopeB4,
    IsoEnvelopeB5,
    IsoEnvelopeB6,
    IsoEnvelopeC3,
    IsoEnvelopeC4,
    IsoEnvelopeC4M,
    IsoEnvelopeC5,
    IsoEnvelopeC6,
    IsoEnvelopeC64M,
    IsoEnvelopeC7,
    IsoEnvelopeCE4,
    IsoEnvelopeCE64,
    IsoEnvelopeE4,
    IsoEnvelopeEC45,
    IsoEnvelopeEC5,
    IsoEnvelopeE5,
    IsoEnvelopeE56,
    IsoEnvelopeE6,
    IsoEnvelopeE65,
    IsoEnvelopeR7,
    IsoEnvelopeS4,
    IsoEnvelopeS5,
    IsoEnvelopeS65,
    IsoEnvelopeX5,
    IsoEnvelopeEX5,

    // Business cards
    UsBusinessCard,
    Iso216BusinessCard,
    EuropeanBusinessCard,
    ScandinavianBusinessCard,
    ChineseBusinessCard,
    JapaneseBusinessCard,
    IranianBusinessCarad,
    HungarianBusinessCard,

    // IDs and credit cards
    Iso7810Id1,
    CR80,          // Same as Iso7810Id1
    TD1,           // Same as Iso7810Id1
    CreditCard,    // Same as Iso7810Id1
    Iso7810Id2,
    Visa,          // Same as Iso7810Id2
    Iso7810Id3,
    Passport,      // Same as Iso7810Id3
    Iso7810Id0000,
    MiniSIM,       // Same as Iso7810Id0000

    // ISO A Sizes
    IsoA0,
    IsoA1,
    IsoA2,
    IsoA3,
    IsoA4,
    IsoA5,
    IsoA6,
    IsoA7,
    IsoA8,
    IsoA9,
    IsoA10,
    IsoA11,
    IsoA12,
    IsoA13,
    Iso2A0,
    Iso4A0,
    IsoA0Plus,
    IsoA1Plus,
    IsoA3Plus,

    // ISO B Sizes
    IsoB0,
    IsoB1,
    IsoB2,
    IsoB3,
    IsoB4,
    IsoB5,
    IsoB6,
    IsoB7,
    IsoB8,
    IsoB9,
    IsoB10,
    IsoB11,
    IsoB12,
    IsoB13,
    IsoB0Plus,
    IsoB1Plus,
    IsoB2Plus,

    // ISO C Sizes
    IsoC0,
    IsoC1,
    IsoC2,
    IsoC3,
    IsoC4,
    IsoC5,
    IsoC6,
    IsoC7,
    IsoC8,
    IsoC9,
    IsoC10,

    // North American ANSI Sizes
    AnsiE,
    AnsiD,
    AnsiC,
    AnsiB,
    AnsiA,

    // North American ARCH Sizes
    ArchE3,
    ArchE2,
    ArchE1,
    ArchE,
    ArchD,
    ArchC,
    ArchB,
    ArchA,
}

impl Type {
    pub fn size(&self) -> Dimensions {
        self.millimeters()
    }

    pub fn millimeters(&self) -> Dimensions {
        Dimensions::from(
            match self {
                // Standard US Sizes
                Self::UsLetter => (216.0, 279.0),
                Self::UsLegal => (215.9, 355.6),
                Self::UsLedger => (279.0, 432.0),
                Self::UsTabloid => (279.0, 432.0),
                Self::UsExecutive => (190.5, 254.0),
                Self::UsJuniorLegal => (203.2, 127.0),
                Self::UsHalfLetter => (140.0, 216.0),
                Self::UsGovernmentLetter => (203.0, 267.0),
                Self::UsGovernmentLegal => (216.0, 330.0),

                // Business cards
                Self::UsBusinessCard => (88.9, 50.8),
                Self::Iso216BusinessCard => (74.0, 52.0),
                Self::EuropeanBusinessCard => (85.0, 55.0),
                Self::ScandinavianBusinessCard => (90.0, 55.0),
                Self::ChineseBusinessCard => (90.0, 54.0),
                Self::JapaneseBusinessCard => (91.0, 55.0),
                Self::IranianBusinessCarad => (85.0, 48.0),
                Self::HungarianBusinessCard => (90.0, 50.0),

                // IDs and credit cards
                Self::Iso7810Id1 => (85.6, 54.0),
                Self::CR80 => (85.6, 54.0),
                Self::TD1 => (85.6, 54.0),
                Self::CreditCard => (85.6, 54.0),
                Self::Iso7810Id2 => (105.0, 74.0),
                Self::Visa => (105.0, 74.0),
                Self::Iso7810Id3 => (125.0, 88.0),
                Self::Passport => (125.0, 88.0),
                Self::Iso7810Id0000 => (25.0, 15.0),
                Self::MiniSIM => (25.0, 15.0),

                // US Envelope Sizes
                Self::UsEnvelope6_25 => (152.0, 89.0),
                Self::UsEnvelope6_75 => (165.0, 92.0),
                Self::UsEnvelope7 => (172.0, 95.0),
                Self::UsEnvelope7_75Monarch => (191.0, 98.0),
                Self::UsEnvelope8_625 => (219.0, 92.0),
                Self::UsEnvelope9 => (225.0, 98.0),
                Self::UsEnvelope10 => (241.0, 104.0),
                Self::UsEnvelope11 => (264.0, 114.0),
                Self::UsEnvelope12 => (279.0, 121.0),
                Self::UsEnvelope14 => (292.0, 127.0),
                Self::UsEnvelope16 => (305.0, 152.0),
                Self::UsEnvelopeA1 => (92.0, 130.0),
                Self::UsEnvelopeA2LadyGrey => (146.0, 111.0),
                Self::UsEnvelopeA4 => (159.0, 108.0),
                Self::UsEnvelopeA6Thompsons => (165.0, 121.0),
                Self::UsEnvelopeA7Besselheim => (184.0, 133.0),
                Self::UsEnvelopeA8Carrs => (206.0, 140.0),
                Self::UsEnvelopeA9Diplomat => (222.0, 146.0),
                Self::UsEnvelopeA10Willow => (241.0, 152.0),
                Self::UsEnvelopeALong => (225.0, 98.0),
                Self::UsEnvelope1 => (229.0, 152.0),
                Self::UsEnvelope1_75 => (241.0, 152.0),
                Self::UsEnvelope3 => (254.0, 178.0),
                Self::UsEnvelope6 => (267.0, 191.0),
                Self::UsEnvelope8 => (286.0, 210.0),
                Self::UsEnvelope9_75 => (286.0, 222.0),
                Self::UsEnvelope10_5 => (305.0, 229.0),
                Self::UsEnvelope12_5 => (318.0, 241.0),
                Self::UsEnvelope13_5 => (330.0, 254.0),
                Self::UsEnvelope14_5 => (368.0, 292.0),
                Self::UsEnvelope15 => (381.0, 254.0),
                Self::UsEnvelope15_5 => (394.0, 305.0),

                // ISO Envelopes
                Self::IsoEnvelopeDL => (110.0, 220.0),
                Self::IsoEnvelopeB4 => (250.0, 353.0),
                Self::IsoEnvelopeB5 => (176.0, 250.0),
                Self::IsoEnvelopeB6 => (125.0, 176.0),
                Self::IsoEnvelopeC3 => (324.0, 458.0),
                Self::IsoEnvelopeC4 => (229.0, 324.0),
                Self::IsoEnvelopeC4M => (318.0, 229.0),
                Self::IsoEnvelopeC5 => (162.0, 229.0),
                Self::IsoEnvelopeC6 => (114.0, 162.0),
                Self::IsoEnvelopeC64M => (318.0, 114.0),
                Self::IsoEnvelopeC7 => (81.0, 114.0),
                Self::IsoEnvelopeCE4 => (229.0, 310.0),
                Self::IsoEnvelopeCE64 => (114.0, 310.0),
                Self::IsoEnvelopeE4 => (220.0, 312.0),
                Self::IsoEnvelopeEC45 => (220.0, 229.0),
                Self::IsoEnvelopeEC5 => (155.0, 229.0),
                Self::IsoEnvelopeE5 => (115.0, 220.0),
                Self::IsoEnvelopeE56 => (155.0, 155.0),
                Self::IsoEnvelopeE6 => (110.0, 155.0),
                Self::IsoEnvelopeE65 => (110.0, 220.0),
                Self::IsoEnvelopeR7 => (120.0, 135.0),
                Self::IsoEnvelopeS4 => (250.0, 330.0),
                Self::IsoEnvelopeS5 => (185.0, 255.0),
                Self::IsoEnvelopeS65 => (110.0, 225.0),
                Self::IsoEnvelopeX5 => (105.0, 216.0),
                Self::IsoEnvelopeEX5 => (155.0, 216.0),

                // ISO A Sizes
                Self::IsoA0 => (841.0, 1189.0),
                Self::IsoA1 => (594.0, 841.0),
                Self::IsoA2 => (420.0, 594.0),
                Self::IsoA3 => (297.0, 420.0),
                Self::IsoA4 => (210.0, 297.0),
                Self::IsoA5 => (148.0, 210.0),
                Self::IsoA6 => (105.0, 148.0),
                Self::IsoA7 => (74.0, 105.0),
                Self::IsoA8 => (52.0, 74.0),
                Self::IsoA9 => (37.0, 52.0),
                Self::IsoA10 => (26.0, 37.0),
                Self::IsoA11 => (18.0, 24.0),
                Self::IsoA12 => (13.0, 18.0),
                Self::IsoA13 => (9.0, 13.0),
                Self::Iso2A0 => (1189.0, 1682.0),
                Self::Iso4A0 => (1682.0, 2378.0),
                Self::IsoA0Plus => (914.0, 1292.0),
                Self::IsoA1Plus => (609.0, 914.0),
                Self::IsoA3Plus => (329.0, 483.0),

                // ISO B Sizes
                Self::IsoB0 => (1000.0, 1414.0),
                Self::IsoB1 => (707.0, 1000.0),
                Self::IsoB2 => (500.0, 707.0),
                Self::IsoB3 => (353.0, 500.0),
                Self::IsoB4 => (250.0, 353.0),
                Self::IsoB5 => (176.0, 250.0),
                Self::IsoB6 => (125.0, 176.0),
                Self::IsoB7 => (88.0, 125.0),
                Self::IsoB8 => (62.0, 88.0),
                Self::IsoB9 => (44.0, 62.0),
                Self::IsoB10 => (31.0, 44.0),
                Self::IsoB11 => (22.0, 31.0),
                Self::IsoB12 => (15.0, 22.0),
                Self::IsoB13 => (11.0, 15.0),
                Self::IsoB0Plus => (1118.0, 1580.0),
                Self::IsoB1Plus => (720.0, 1020.0),
                Self::IsoB2Plus => (520.0, 720.0),

                // ISO C Sizes
                Self::IsoC0 => (917.0, 1297.0),
                Self::IsoC1 => (648.0, 917.0),
                Self::IsoC2 => (458.0, 648.0),
                Self::IsoC3 => (324.0, 458.0),
                Self::IsoC4 => (229.0, 324.0),
                Self::IsoC5 => (162.0, 229.0),
                Self::IsoC6 => (114.0, 162.0),
                Self::IsoC7 => (81.0, 114.0),
                Self::IsoC8 => (57.0, 81.0),
                Self::IsoC9 => (40.0, 57.0),
                Self::IsoC10 => (28.0, 40.0),

                // North American ANSI Sizes
                Self::AnsiE => (1118.0, 864.0),
                Self::AnsiD => (559.0, 864.0),
                Self::AnsiC => (432.0, 559.0),
                Self::AnsiB => (279.0, 432.0),
                Self::AnsiA => (216.0, 279.0),

                // North American ARCH Sizes
                Self::ArchE3 => (686.0, 991.0),
                Self::ArchE2 => (660.0, 965.0),
                Self::ArchE1 => (762.0, 1067.0),
                Self::ArchE => (1914.0, 1219.0),
                Self::ArchD => (610.0, 914.0),
                Self::ArchC => (457.0, 610.0),
                Self::ArchB => (305.0, 457.0),
                Self::ArchA => (229.0, 305.0),
            }
        )
    }
}

pub type Millimeters = f64;

#[derive(Debug, Copy, Clone)]
pub struct Dimensions {
    pub width: Millimeters,
    pub height: Millimeters,
}

impl Display for Dimensions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}x{}", self.width, self.height)
    }
}

impl From<(f64, f64)> for Dimensions {
    fn from(value: (f64, f64)) -> Self {
        Self{
            width: value.0,
            height: value.1,
        }
    }
}
