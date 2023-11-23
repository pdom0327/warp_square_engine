use bitflags::bitflags;

bitflags! {
    #[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Hash)]
    pub struct BitLevel: u16 {
        const White = 1;
        const Neutral = 1 << 1;
        const Black = 1 << 2;
        const QL1 = 1 << 3;
        const QL2 = 1 << 4;
        const QL3 = 1 << 5;
        const QL4 = 1 << 6;
        const QL5 = 1 << 7;
        const QL6 = 1 << 8;
        const KL1 = 1 << 9;
        const KL2 = 1 << 10;
        const KL3 = 1 << 11;
        const KL4 = 1 << 12;
        const KL5 = 1 << 13;
        const KL6 = 1 << 14;
    }

    #[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Debug, Hash)]
    pub struct BitBoard: u64 {
        const Empty = 0;

        /// a1
        const W0 = 1;
        /// b1
        const W1 = 1 << 1;
        /// c1
        const W2 = 1 << 2;
        /// d1
        const W3 = 1 << 3;
        /// a2
        const W4 = 1 << 4;
        /// b2
        const W5 = 1 << 5;
        /// c2
        const W6 = 1 << 6;
        /// d2
        const W7 = 1 << 7;
        /// a3
        const W8 = 1 << 8;
        /// b3
        const W9 = 1 << 9;
        /// c3
        const WA = 1 << 10;
        /// d3
        const WB = 1 << 11;
        /// a4
        const WC = 1 << 12;
        /// b4
        const WD = 1 << 13;
        /// c4
        const WE = 1 << 14;
        /// d4
        const WF = 1 << 15;

        const W = Self::W0.bits() | Self::W1.bits() | Self::W2.bits() | Self::W3.bits() |
                  Self::W4.bits() | Self::W5.bits() | Self::W6.bits() | Self::W7.bits() |
                  Self::W8.bits() | Self::W9.bits() | Self::WA.bits() | Self::WB.bits() |
                  Self::WC.bits() | Self::WD.bits() | Self::WE.bits() | Self::WF.bits();

        const N0 = 1 << 16;
        const N1 = 1 << 17;
        const N2 = 1 << 18;
        const N3 = 1 << 19;
        const N4 = 1 << 20;
        const N5 = 1 << 21;
        const N6 = 1 << 22;
        const N7 = 1 << 23;
        const N8 = 1 << 24;
        const N9 = 1 << 25;
        const NA = 1 << 26;
        const NB = 1 << 27;
        const NC = 1 << 28;
        const ND = 1 << 29;
        const NE = 1 << 30;
        const NF = 1 << 31;

        const N = Self::N0.bits() | Self::N1.bits() | Self::N2.bits() | Self::N3.bits() |
        Self::N4.bits() | Self::N5.bits() | Self::N6.bits() | Self::N7.bits() |
        Self::N8.bits() | Self::N9.bits() | Self::NA.bits() | Self::NB.bits() |
        Self::NC.bits() | Self::ND.bits() | Self::NE.bits() | Self::NF.bits();

        const B0 = 1 << 32;
        const B1 = 1 << 33;
        const B2 = 1 << 34;
        const B3 = 1 << 35;
        const B4 = 1 << 36;
        const B5 = 1 << 37;
        const B6 = 1 << 38;
        const B7 = 1 << 39;
        const B8 = 1 << 40;
        const B9 = 1 << 41;
        const BA = 1 << 42;
        const BB = 1 << 43;
        const BC = 1 << 44;
        const BD = 1 << 45;
        const BE = 1 << 46;
        const BF = 1 << 47;

        const B = Self::B0.bits() | Self::B1.bits() | Self::B2.bits() | Self::B3.bits() |
        Self::B4.bits() | Self::B5.bits() | Self::B6.bits() | Self::B7.bits() |
        Self::B8.bits() | Self::B9.bits() | Self::BA.bits() | Self::BB.bits() |
        Self::BC.bits() | Self::BD.bits() | Self::BE.bits() | Self::BF.bits();

        const QW0 = 1 << 48;
        const QW1 = 1 << 49;
        const QW2 = 1 << 50;
        const QW3 = 1 << 51;

        const QW = Self::QW0.bits() | Self::QW1.bits() | Self::QW2.bits() | Self::QW3.bits();

        const KW0 = 1 << 52;
        const KW1 = 1 << 53;
        const KW2 = 1 << 54;
        const KW3 = 1 << 55;

        const KW = Self::KW0.bits() | Self::KW1.bits() | Self::KW2.bits() | Self::KW3.bits();

        const QB0 = 1 << 56;
        const QB1 = 1 << 57;
        const QB2 = 1 << 58;
        const QB3 = 1 << 59;

        const QB = Self::QB0.bits() | Self::QB1.bits() | Self::QB2.bits() | Self::QB3.bits();

        const KB0 = 1 << 60;
        const KB1 = 1 << 61;
        const KB2 = 1 << 62;
        const KB3 = 1 << 63;

        const KB = Self::KB0.bits() | Self::KB1.bits() | Self::KB2.bits() | Self::KB3.bits();
    }
}
