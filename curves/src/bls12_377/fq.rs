// Copyright (C) 2019-2022 Aleo Systems Inc.
// This file is part of the snarkVM library.

// The snarkVM library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkVM library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkVM library. If not, see <https://www.gnu.org/licenses/>.

use snarkvm_fields::{
    FftParameters,
    FieldParameters,
    Fp384,
    Fp384Parameters,
    PoseidonDefaultParameters,
    PoseidonDefaultParametersEntry,
};
use snarkvm_utilities::biginteger::BigInteger384 as BigInteger;

pub type Fq = Fp384<FqParameters>;

pub struct FqParameters;

impl Fp384Parameters for FqParameters {}

impl FftParameters for FqParameters {
    type BigInteger = BigInteger;

    #[rustfmt::skip]
    const POWERS_OF_G: &'static [BigInteger] = &[
        BigInteger([7385130745848031862, 701542640293853790, 7860911539678904085, 2081126899580975139, 1314744540578258676, 5505385934676702]),
        BigInteger([5134937160227832732, 2527472353192255794, 5983990541424952770, 12889027417780705039, 1827101640268914115, 35921860030846829]),
        BigInteger([17978844473626023153, 17876791103060992241, 14280117436792259086, 6075183100816533711, 14380438162445701085, 23634273502926963]),
        BigInteger([3042313168607755652, 2898683936875278397, 18142852711952420851, 3299128674474358658, 5587069763096216413, 57910226810639103]),
        BigInteger([8670062213291950983, 13231535783847415120, 13853374098618609475, 4375084111178468297, 15058863618213473729, 104302990866566346]),
        BigInteger([6782246736218306282, 13202884221908298588, 17690457268828644541, 1977382937194332749, 1043164677067163225, 66006011576170137]),
        BigInteger([14729707132561216529, 7604534147383175560, 8315993351605680848, 15266138444828105666, 6205165579526030584, 22022460061138669]),
        BigInteger([6933372435512853879, 17628910870098511585, 10234767657922363124, 14806887497637517507, 16162400383743571186, 72444598743672739]),
        BigInteger([8065937948938570075, 12349274400625613597, 9974581888245964682, 12097593823039447971, 5746133023516248483, 20558105834535994]),
        BigInteger([6276868676555924960, 11048343163423705260, 13200118183172813340, 11888317846591950664, 12394546186160613838, 53596618024763257]),
        BigInteger([13476941164363887320, 6529087504189619549, 13078956481460091614, 14557775281486146673, 5487124721358523033, 10819738202564788]),
        BigInteger([13364818876935230670, 13269779243477965971, 15761372403346801792, 7427170436970270783, 8383781585990660556, 23078685542227851]),
        BigInteger([17619255348814714837, 7566977047604138306, 16444835836418231544, 7587307705081412631, 13159035542584219759, 89641791553270776]),
        BigInteger([13411296197887831162, 949773516635351564, 17264420250194156204, 7803749772625501627, 7722287160777812231, 92384713559664914]),
        BigInteger([6568791503267352999, 11293051525851483970, 5706681168726160073, 11095454440956603812, 5019415166258911703, 35364342249184590]),
        BigInteger([7063460146736001849, 3851981843724025120, 4404051240421452066, 8747482441359976867, 11928295146298204946, 15143911508191343]),
        BigInteger([11326837853769895854, 12895752614942843615, 18082893035929964238, 6645302761511957642, 13726634636068843846, 44863353895094001]),
        BigInteger([11417549255766836924, 6538651435163630262, 13199868510547063205, 2112495793012567303, 11909382513093093621, 75895599265178156]),
        BigInteger([8401643149461645733, 12537573869478123761, 4341358729788488908, 10710884481768240625, 12971282953629475608, 45112253026753539]),
        BigInteger([1152007377126551531, 9007601379630786885, 1437653439713848702, 8068372743214382487, 5083343468186829292, 60684180583630844]),
        BigInteger([2248450227402695479, 14072713940474318953, 15235079680217091076, 10148034663319658399, 10948083137235044598, 49146251091271825]),
        BigInteger([8841623610086649495, 5088757700890560195, 9378044910544196932, 10224534985235096112, 9183104370154873548, 23033958700638443]),
        BigInteger([1631173013104702089, 7590789119163414124, 5824062274472647016, 17863642524987727381, 4255407151711471511, 64651884988346985]),
        BigInteger([9310047075170506565, 2104747671456310692, 2644219065673981138, 4856291027038128876, 8180550020249516800, 106383048458869756]),
        BigInteger([337370040796255196, 5211605300302596945, 7148574707434112052, 15794714402959134312, 15972085473587326639, 37109373511278407]),
        BigInteger([11471650965284872747, 2409290254183797046, 12611606615659248625, 352120820464993320, 17145040817448456307, 76649058183435327]),
        BigInteger([628222360320685885, 3316269393081920675, 8362470787126080828, 5039133592514299306, 14593231448297904645, 66448120101990539]),
        BigInteger([17207968786480344296, 15238487662000849007, 13594549201811747429, 1023321563714992538, 1640889964080741822, 93804327810640416]),
        BigInteger([7175959093718815137, 8275715052969182726, 6436112820578665714, 8138571789395785875, 6050826606617757421, 96865101988354652]),
        BigInteger([4070177388613442533, 17956834252521987703, 6950522450157943907, 12697584603990425156, 16587871642483005244, 57471560345127555]),
        BigInteger([10900985717020507711, 5295852191758189818, 11820298144983500636, 550003973110951803, 263308017339972560, 111125542229885816]),
        BigInteger([6096269296977312113, 9933768294483763101, 3745456635322318425, 1234445854684469083, 6265727993591295891, 90344874383011427]),
        BigInteger([14266056396848430283, 15657563994013446202, 2661332711830601499, 9394576428684896628, 2717774174090368664, 8155723651091523]),
        BigInteger([10525954341863739652, 9230764219066024507, 1476128509584006807, 9208851952254982823, 3442027277949361561, 31949044617252679]),
        BigInteger([511600968522897682, 7208634953489512620, 3530006720424357148, 14813261074251638594, 12630011612421075990, 88930841157402859]),
        BigInteger([2873670194202000053, 316233994604135445, 3504079781682637435, 16475221371051119220, 5511724617596116755, 48306385732647534]),
        BigInteger([18390923601239652091, 8383328236368904633, 16870035286970652539, 1713556008324362517, 10834798082861818717, 76626671700720801]),
        BigInteger([6453880472035760162, 9279136170086455551, 5255158191281631096, 9402683820780075235, 13294404117759409916, 28911609878178747]),
        BigInteger([15457678214934104727, 10257410602787968791, 10543920980663046204, 12873421307184168602, 3005171103040729111, 6608332313712721]),
        BigInteger([7669310927744901814, 10180432966734195262, 7102068305190679471, 3394102346354943911, 7728984932024735354, 113393442359243117]),
        BigInteger([332121547564755110, 4058914442193592743, 3261683794795430975, 14314300534785368151, 12189004563146915906, 76871270605085138]),
        BigInteger([5666933051403538098, 9026061949184173996, 2715769564174870087, 15173756192814126609, 9536623932835594166, 62873430292373036]),
        BigInteger([13313210803279475255, 648193293732931258, 17506987073671981935, 17186037581194467208, 9596589497284832987, 53781961547701120]),
        BigInteger([7836246735827273146, 13870095715782950813, 14107419625140408226, 4952953651681455070, 12502319229901919751, 7811766342450446]),
        BigInteger([7959784605059980387, 11049727206760807822, 13611648078312050530, 4612698621573455553, 13890566741340831409, 101342263646697418]),
    ];
    #[rustfmt::skip]
    const TWO_ADICITY: u32 = 46u32;
    #[rustfmt::skip]
    const TWO_ADIC_ROOT_OF_UNITY: BigInteger = BigInteger([
        2022196864061697551u64,
        17419102863309525423u64,
        8564289679875062096u64,
        17152078065055548215u64,
        17966377291017729567u64,
        68610905582439508u64,
    ]);
}

impl FieldParameters for FqParameters {
    #[rustfmt::skip]
    const CAPACITY: u32 = Self::MODULUS_BITS - 1;
    /// GENERATOR = -5
    #[rustfmt::skip]
    const GENERATOR: BigInteger = BigInteger([
        0xfc0b8000000002fa,
        0x97d39cf6e000018b,
        0x2072420fbfa05044,
        0xcbbcbd50d97c3802,
        0xbaf1ec35813f9eb,
        0x9974a2c0945ad2,
    ]);
    #[rustfmt::skip]
    const INV: u64 = 9586122913090633727u64;
    /// MODULUS = 258664426012969094010652733694893533536393512754914660539884262666720468348340822774968888139573360124440321458177
    #[rustfmt::skip]
    const MODULUS: BigInteger = BigInteger([
        0x8508c00000000001,
        0x170b5d4430000000,
        0x1ef3622fba094800,
        0x1a22d9f300f5138f,
        0xc63b05c06ca1493b,
        0x1ae3a4617c510ea,
    ]);
    #[rustfmt::skip]
    const MODULUS_BITS: u32 = 377;
    #[rustfmt::skip]
    const MODULUS_MINUS_ONE_DIV_TWO: BigInteger = BigInteger([
        0x4284600000000000,
        0xb85aea218000000,
        0x8f79b117dd04a400,
        0x8d116cf9807a89c7,
        0x631d82e03650a49d,
        0xd71d230be28875,
    ]);
    #[rustfmt::skip]
    const R: BigInteger = BigInteger([
        202099033278250856u64,
        5854854902718660529u64,
        11492539364873682930u64,
        8885205928937022213u64,
        5545221690922665192u64,
        39800542322357402u64,
    ]);
    #[rustfmt::skip]
    const R2: BigInteger = BigInteger([
        0xb786686c9400cd22,
        0x329fcaab00431b1,
        0x22a5f11162d6b46d,
        0xbfdf7d03827dc3ac,
        0x837e92f041790bf9,
        0x6dfccb1e914b88,
    ]);
    #[rustfmt::skip]
    const REPR_SHAVE_BITS: u32 = 7;
    // T and T_MINUS_ONE_DIV_TWO, where MODULUS - 1 = 2^S * T

    /// T = (MODULUS - 1) // 2^S =
    /// 3675842578061421676390135839012792950148785745837396071634149488243117337281387659330802195819009059
    #[rustfmt::skip]
    const T: BigInteger = BigInteger([
        0x7510c00000021423,
        0x88bee82520005c2d,
        0x67cc03d44e3c7bcd,
        0x1701b28524ec688b,
        0xe9185f1443ab18ec,
        0x6b8,
    ]);
    /// (T - 1) // 2 =
    /// 1837921289030710838195067919506396475074392872918698035817074744121558668640693829665401097909504529
    #[rustfmt::skip]
    const T_MINUS_ONE_DIV_TWO: BigInteger = BigInteger([
        0xba88600000010a11,
        0xc45f741290002e16,
        0xb3e601ea271e3de6,
        0xb80d94292763445,
        0x748c2f8a21d58c76,
        0x35c,
    ]);
}

impl PoseidonDefaultParameters for FqParameters {
    const PARAMS_OPT_FOR_CONSTRAINTS: [PoseidonDefaultParametersEntry; 7] = [
        PoseidonDefaultParametersEntry::new(2, 17, 8, 31, 0),
        PoseidonDefaultParametersEntry::new(3, 5, 8, 56, 0),
        PoseidonDefaultParametersEntry::new(4, 5, 8, 56, 0),
        PoseidonDefaultParametersEntry::new(5, 5, 8, 57, 0),
        PoseidonDefaultParametersEntry::new(6, 5, 8, 57, 0),
        PoseidonDefaultParametersEntry::new(7, 5, 8, 57, 0),
        PoseidonDefaultParametersEntry::new(8, 5, 8, 57, 0),
    ];
}

#[cfg(test)]
mod tests {
    use super::*;
    use snarkvm_fields::{FftField, Field, PrimeField};

    #[test]
    fn test_powers_of_g() {
        let two = Fq::from(2u8);

        // Compute the expected powers of G.
        let g = Fq::two_adic_root_of_unity();
        let powers = (0..FqParameters::TWO_ADICITY - 1)
            .map(|i| g.pow(two.pow(Fq::from(i as u64).to_repr()).to_repr()).to_repr())
            .collect::<Vec<_>>();

        // Ensure the correct number of powers of G are present.
        assert_eq!(FqParameters::POWERS_OF_G.len() as u64, (FqParameters::TWO_ADICITY - 1) as u64);
        assert_eq!(FqParameters::POWERS_OF_G.len(), powers.len());

        // Ensure the expected and candidate powers match.
        for (expected, candidate) in powers.iter().zip(FqParameters::POWERS_OF_G.iter()) {
            println!("{:?} =?= {:?}", expected, candidate);
            assert_eq!(expected, candidate);
        }
    }
}
