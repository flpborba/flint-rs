mod creation {
    use crate::Int;

    #[test]
    fn maximmum_number_of_digits() {
        let a = Int::with_digits(&[
            9814047247794759920,
            8221081269568818299,
            12303974116353362217,
            13175780254668618652,
            4688020482074265576,
            309817869716111569,
            17990625622913091920,
            2375373922744667203,
            16874299996718624161,
            17289964844370016266,
            4173412299542918285,
            6878286933418990294,
            8961436815872852951,
            8037016722112227866,
            18060313362895502858,
            13725275493163208184,
            10082829642860676218,
            552651243827123816,
            13095941771964033476,
            6492680750758320784,
            17140280078544689415,
            2384340039255482062,
            16802007763768416901,
            12679008452125000717,
            17258473971286312462,
            16990403477761319891,
            3336720415011723153,
            11548981731734235277,
            11058441847142267231,
            12947534202124918389,
            556418016596579213,
            12558500929732769775,
        ]);

        let b = Int {
            dgt: [
                9814047247794759920,
                8221081269568818299,
                12303974116353362217,
                13175780254668618652,
                4688020482074265576,
                309817869716111569,
                17990625622913091920,
                2375373922744667203,
                16874299996718624161,
                17289964844370016266,
                4173412299542918285,
                6878286933418990294,
                8961436815872852951,
                8037016722112227866,
                18060313362895502858,
                13725275493163208184,
                10082829642860676218,
                552651243827123816,
                13095941771964033476,
                6492680750758320784,
                17140280078544689415,
                2384340039255482062,
                16802007763768416901,
                12679008452125000717,
                17258473971286312462,
                16990403477761319891,
                3336720415011723153,
                11548981731734235277,
                11058441847142267231,
                12947534202124918389,
                556418016596579213,
                12558500929732769775,
            ],
            len: 32,
        };

        assert_eq!(a, b);
    }

    #[test]
    fn more_digits_than_maxmimum() {
        let a = Int::with_digits(&[
            7035995116470561996,
            15799079827703118949,
            1196312812855362003,
            10621872891686176071,
            15644978289733344242,
            13688798866686301694,
            10251350567723002113,
            9394174513261490966,
            670746421531938600,
            7434398718895273617,
            2024091819726622262,
            14323709098783219607,
            7698195393548531523,
            3714208577660397351,
            9325002575965367575,
            8781879430809432221,
            1011377845902676056,
            11150757258136250607,
            18122702214507284468,
            15241674503557342566,
            4227185597983269565,
            18239621616700741467,
            3850119352676966523,
            4497145824425581918,
            12544262268603562726,
            14655078982132108033,
            14813410064045918259,
            11610346388382140154,
            16997438674886248347,
            2900696386142787616,
            4049038047075152576,
            1926599733630686310,
            12847120904398879255,
        ]);

        let b = Int {
            dgt: [
                7035995116470561996,
                15799079827703118949,
                1196312812855362003,
                10621872891686176071,
                15644978289733344242,
                13688798866686301694,
                10251350567723002113,
                9394174513261490966,
                670746421531938600,
                7434398718895273617,
                2024091819726622262,
                14323709098783219607,
                7698195393548531523,
                3714208577660397351,
                9325002575965367575,
                8781879430809432221,
                1011377845902676056,
                11150757258136250607,
                18122702214507284468,
                15241674503557342566,
                4227185597983269565,
                18239621616700741467,
                3850119352676966523,
                4497145824425581918,
                12544262268603562726,
                14655078982132108033,
                14813410064045918259,
                11610346388382140154,
                16997438674886248347,
                2900696386142787616,
                4049038047075152576,
                1926599733630686310,
            ],
            len: 32,
        };

        assert_eq!(a, b);
    }

    #[test]
    fn no_digits() {
        assert_eq!(Int::with_digits(&[]), Int::MIN);
    }

    #[test]
    fn less_digits_than_maxmimum() {
        let a = Int::with_digits(&[
            3846718537225852159,
            14440821564470045971,
            4993314731374841758,
        ]);

        let b = Int {
            dgt: [
                3846718537225852159,
                14440821564470045971,
                4993314731374841758,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            len: 3,
        };

        assert_eq!(a, b);
    }

    #[test]
    fn leading_zeros() {
        let a = Int::with_digits(&[
            8012955955319221078,
            13838156207890544942,
            7835385100763510655,
            0,
            0,
            0,
        ]);

        let b = Int::with_digits(&[
            8012955955319221078,
            13838156207890544942,
            7835385100763510655,
        ]);

        assert_eq!(a, b);
    }
}
