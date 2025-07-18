use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn QRinput_new2(version: libc::c_int, level: QRecLevel) -> *mut QRinput;
    fn printQRinput(input: *mut QRinput);
    fn testReport(tests: libc::c_int);
    fn testEnd(result: libc::c_int);
    fn testStartReal(func: *const libc::c_char, name: *const libc::c_char);
    fn testInit(tests: libc::c_int);
    fn BitStream_new() -> *mut BitStream;
    fn BitStream_free(bstream: *mut BitStream);
    fn QRinput_mergeBitStream(
        input: *mut QRinput,
        bstream: *mut BitStream,
    ) -> libc::c_int;
    fn QRinput_free(input: *mut QRinput);
    fn Split_splitStringToQRinput(
        string: *const libc::c_char,
        input: *mut QRinput,
        hint: QRencodeMode,
        casesensitive: libc::c_int,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type QRencodeMode = libc::c_int;
pub const QR_MODE_FNC1SECOND: QRencodeMode = 7;
pub const QR_MODE_FNC1FIRST: QRencodeMode = 6;
pub const QR_MODE_ECI: QRencodeMode = 5;
pub const QR_MODE_STRUCTURE: QRencodeMode = 4;
pub const QR_MODE_KANJI: QRencodeMode = 3;
pub const QR_MODE_8: QRencodeMode = 2;
pub const QR_MODE_AN: QRencodeMode = 1;
pub const QR_MODE_NUM: QRencodeMode = 0;
pub const QR_MODE_NUL: QRencodeMode = -1;
pub type QRecLevel = libc::c_uint;
pub const QR_ECLEVEL_H: QRecLevel = 3;
pub const QR_ECLEVEL_Q: QRecLevel = 2;
pub const QR_ECLEVEL_M: QRecLevel = 1;
pub const QR_ECLEVEL_L: QRecLevel = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _QRinput {
    pub version: libc::c_int,
    pub level: QRecLevel,
    pub head: *mut QRinput_List,
    pub tail: *mut QRinput_List,
    pub mqr: libc::c_int,
    pub fnc1: libc::c_int,
    pub appid: libc::c_uchar,
}
pub type QRinput_List = _QRinput_List;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _QRinput_List {
    pub mode: QRencodeMode,
    pub size: libc::c_int,
    pub data: *mut libc::c_uchar,
    pub bstream: *mut BitStream,
    pub next: *mut QRinput_List,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BitStream {
    pub length: size_t,
    pub datasize: size_t,
    pub data: *mut libc::c_uchar,
}
pub type QRinput = _QRinput;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TestSet {
    pub expected_length: size_t,
    pub url: *mut libc::c_char,
}
static mut testset: [TestSet; 1266] = [
    {
        let mut init = TestSet {
            expected_length: 3132 as libc::c_int as size_t,
            url: b"http://ads.bluelithium.com/clk?3,eJytjcFqwzAQRL8mNyO0luUoiB7WcQUGS8RFbXBuqVEcxxbKwSDy93HS0P5AH3uYYXd2AKT4hpOjJ-d4njGWdxIySB2nnB0hoVJKRvkmhTXb8GRyIWBVdw0W01bRAh-Y3Vfb4ItS4Ee.fcqdQBJLQsTg2.5nmzOT479QirH4fOkKcfnfL9WDMk09vhd.ZxpM2cSD17G2yh8uetZWTfoGQ7tXY22N1xa53lfc2DYz.W.yLUnO83xdMVylapkYI7kdzyGQLvjF3wHZQ1eK,http%3A%2F%2Fomicronpath.com%2F%3Fa%3D448%26c%3D3201%26s1%3DYahooStream_917_Nevada4\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 3745 as libc::c_int as size_t,
            url: b"http://ads.bluelithium.com/clk?3,eJytjd2OgjAQhZ.GO9K0lEpJ48WwUNekdIWoBO8UCfKX7gUJ4tNv3bj6AvtlLs7knDlDiDgzSi8sqDzOAtfnviAecSuGGT25DhZCUGwN4tOAOX1lDGxUmULYZT0O4cGnyYoankQcsvrjV245oClCiDfDn8.UYQn.QsS7cP.UGwDbX9vXjdSp6uLwHUuIjtL5ay1btZPDsU3GZCf7ZCZNkcte5YdG3.c33cZY5wU7pq.LleNcx.F7QWHhSjvTNKH5dDUGlWaw-w-Jy1ek,http%3A%2F%2Ftanphysics.com%2Farticles%2Freview1.php%3Futm_source%3DUSA%26utm_medium%3DYHO%26utm_content%3D917-STRM4%26utm_campaign%3DYHO%26keyword%3DYHO-917-STRM4\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 896 as libc::c_int as size_t,
            url: b"http://amigo.geneontology.org/cgi-bin/amigo/go.cgi?view=details&search_constraint=terms&depth=0&query=GO:0042438\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 611 as libc::c_int as size_t,
            url: b"http://amigo.geneontology.org/cgi-bin/amigo/gp-assoc.cgi?gp=UniProtKB:P30046\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 212 as libc::c_int as size_t,
            url: b"http://answers.yahoo.com/\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 196 as libc::c_int as size_t,
            url: b"http://autos.yahoo.com/\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 348 as libc::c_int as size_t,
            url: b"http://baseball.fantasysports.yahoo.com/b1\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 234 as libc::c_int as size_t,
            url: b"http://biogps.org/gene/1652/\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 644 as libc::c_int as size_t,
            url: b"http://blog.washingtonpost.com/dcsportsbog/2007/05/a_caron_butler_surprise.html\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 340 as libc::c_int as size_t,
            url: b"http://ca.wikipedia.org/wiki/Caron_Butler\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 212 as libc::c_int as size_t,
            url: b"http://careers.yahoo.com/\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 404 as libc::c_int as size_t,
            url: b"http://commons.wikimedia.org/wiki/Atlas_of_Turkey\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 380 as libc::c_int as size_t,
            url: b"http://creativecommons.org/licenses/by-sa/3.0/\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 380 as libc::c_int as size_t,
            url: b"http://da.wikipedia.org/wiki/Johann_David_Wyss\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 244 as libc::c_int as size_t,
            url: b"http://de.wikipedia.org/wiki/\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 340 as libc::c_int as size_t,
            url: b"http://de.wikipedia.org/wiki/Caron_Butler\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 380 as libc::c_int as size_t,
            url: b"http://de.wikipedia.org/wiki/Johann_David_Wyss\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 340 as libc::c_int as size_t,
            url: b"http://de.wikipedia.org/wiki/Massey_Lopes\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 512 as libc::c_int as size_t,
            url: b"http://de.wikipedia.org/wiki/T%C3%BCrkische_Verfassung_von_1921\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 336 as libc::c_int as size_t,
            url: b"http://dx.doi.org/10.1002%2Felps.11501301201\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 334 as libc::c_int as size_t,
            url: b"http://dx.doi.org/10.1006%2Fbbrc.1993.2524\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 296 as libc::c_int as size_t,
            url: b"http://dx.doi.org/10.1007%2Fs003359900858\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 304 as libc::c_int as size_t,
            url: b"http://dx.doi.org/10.1021%2Fbi982184o\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 402 as libc::c_int as size_t,
            url: b"http://dx.doi.org/10.1034%2Fj.1600-0625.2003.120307.x\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 255 as libc::c_int as size_t,
            url: b"http://dx.doi.org/10.1038%2F990031\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 296 as libc::c_int as size_t,
            url: b"http://dx.doi.org/10.1038%2Fmsb4100134\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 282 as libc::c_int as size_t,
            url: b"http://dx.doi.org/10.1038%2Fnbt810\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 329 as libc::c_int as size_t,
            url: b"http://dx.doi.org/10.1073%2Fpnas.242603899\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 326 as libc::c_int as size_t,
            url: b"http://dx.doi.org/10.1074%2Fjbc.M203220200\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 307 as libc::c_int as size_t,
            url: b"http://dx.doi.org/10.1101%2Fgr.2596504\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 360 as libc::c_int as size_t,
            url: b"http://dx.doi.org/10.1186%2Fgb-2004-5-10-r84\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 228 as libc::c_int as size_t,
            url: b"http://education.yahoo.com/\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 364 as libc::c_int as size_t,
            url: b"http://en.m.wikipedia.org/wiki/Ambrose_Dyson\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 356 as libc::c_int as size_t,
            url: b"http://en.m.wikipedia.org/wiki/Caron_Butler\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 340 as libc::c_int as size_t,
            url: b"http://en.m.wikipedia.org/wiki/DDT_(gene)\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 396 as libc::c_int as size_t,
            url: b"http://en.m.wikipedia.org/wiki/Johann_David_Wyss\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 332 as libc::c_int as size_t,
            url: b"http://en.m.wikipedia.org/wiki/Lady_Anne\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 492 as libc::c_int as size_t,
            url: b"http://en.m.wikipedia.org/wiki/Sir_Massey_Lopes,_3rd_Baronet\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 308 as libc::c_int as size_t,
            url: b"http://en.m.wikipedia.org/wiki/Tumula\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 480 as libc::c_int as size_t,
            url: b"http://en.m.wikipedia.org/wiki/Turkish_Constitution_of_1921\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 556 as libc::c_int as size_t,
            url: b"http://en.m.wikipedia.org/wiki/Vietnam_University_Admission_Rankings\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 444 as libc::c_int as size_t,
            url: b"http://en.m.wikipedia.org/wiki/Vy%C5%A1n%C3%BD_Slavkov\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 196 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 816 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=2013%E2%80%9314_Milwaukee_Bucks_season&action=edit&redlink=1\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 548 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Ambrose_Dyson&action=edit\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 628 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Ambrose_Dyson&action=edit&section=1\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 572 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Ambrose_Dyson&action=history\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 548 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Ambrose_Dyson&action=info\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 552 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Ambrose_Dyson&oldid=562636681\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 564 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Ambrose_Dyson&printable=yes\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 564 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Ambrose_Dyson&veaction=edit\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 644 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Ambrose_Dyson&veaction=edit&section=1\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 540 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Caron_Butler&action=edit\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 620 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Caron_Butler&action=edit&section=1\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 564 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Caron_Butler&action=history\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 540 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Caron_Butler&action=info\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 544 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Caron_Butler&oldid=571697645\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 556 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Caron_Butler&printable=yes\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 556 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Caron_Butler&veaction=edit\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 636 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Caron_Butler&veaction=edit&section=1\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 780 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Court_of_Jurisdictional_Conflict&action=edit&redlink=1\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 524 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=DDT_(gene)&action=edit\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 604 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=DDT_(gene)&action=edit&section=1\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 548 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=DDT_(gene)&action=history\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 524 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=DDT_(gene)&action=info\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 528 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=DDT_(gene)&oldid=453962249\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 540 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=DDT_(gene)&printable=yes\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 540 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=DDT_(gene)&veaction=edit\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 620 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=DDT_(gene)&veaction=edit&section=1\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 724 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Devonshire_County_Council&action=edit&redlink=1\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 812 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=H%C3%A0_T%C4%A9nh_Gifted_High_School&action=edit&redlink=1\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 820 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=H%C6%B0ng_Y%C3%AAn_Gifted_High_School&action=edit&redlink=1\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 580 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Johann_David_Wyss&action=edit\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 660 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Johann_David_Wyss&action=edit&section=1\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 660 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Johann_David_Wyss&action=edit&section=2\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 604 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Johann_David_Wyss&action=history\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 580 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Johann_David_Wyss&action=info\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 584 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Johann_David_Wyss&oldid=568638498\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 596 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Johann_David_Wyss&printable=yes\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 596 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Johann_David_Wyss&veaction=edit\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 676 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Johann_David_Wyss&veaction=edit&section=1\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 676 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Johann_David_Wyss&veaction=edit&section=2\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 684 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Johann_Emmanuel_Wyss&action=edit&redlink=1\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 652 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Josh_Oppenheimer&action=edit&redlink=1\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 588 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Lady_Ann&action=edit&redlink=1\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 516 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Lady_Anne&action=edit\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 596 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Lady_Anne&action=edit&section=1\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 540 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Lady_Anne&action=history\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 516 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Lady_Anne&action=info\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 520 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Lady_Anne&oldid=372343156\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 532 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Lady_Anne&printable=yes\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 532 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Lady_Anne&veaction=edit\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 612 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Lady_Anne&veaction=edit&section=1\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 740 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Military_Court_of_Cassation&action=edit&redlink=1\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 820 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Military_High_Court_of_Administration&action=edit&redlink=1\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 620 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Monte_Mathis&action=edit&redlink=1\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 764 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Quang_Trung_Gifted_High_School&action=edit&redlink=1\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 740 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Robert_Hackett_(basketball)&action=edit&redlink=1\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 676 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Sir_Massey_Lopes,_3rd_Baronet&action=edit\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 756 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Sir_Massey_Lopes,_3rd_Baronet&action=edit&section=1\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 700 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Sir_Massey_Lopes,_3rd_Baronet&action=history\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 676 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Sir_Massey_Lopes,_3rd_Baronet&action=info\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 680 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Sir_Massey_Lopes,_3rd_Baronet&oldid=545042390\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 692 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Sir_Massey_Lopes,_3rd_Baronet&printable=yes\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 692 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Sir_Massey_Lopes,_3rd_Baronet&veaction=edit\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 772 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Sir_Massey_Lopes,_3rd_Baronet&veaction=edit&section=1\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 772 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Sir_Massey_Lopes,_3rd_Baronet&veaction=edit&section=2\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 772 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Sir_Massey_Lopes,_3rd_Baronet&veaction=edit&section=3\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 788 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Special:Book&bookcmd=book_creator&referer=Ambrose+Dyson\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 780 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Special:Book&bookcmd=book_creator&referer=Caron+Butler\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 796 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Special:Book&bookcmd=book_creator&referer=DDT+%28gene%29\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 820 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Special:Book&bookcmd=book_creator&referer=Johann+David+Wyss\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 756 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Special:Book&bookcmd=book_creator&referer=Lady+Anne\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 932 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Special:Book&bookcmd=book_creator&referer=Sir+Massey+Lopes%2C+3rd+Baronet\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 732 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Special:Book&bookcmd=book_creator&referer=Tumula\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 908 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Special:Book&bookcmd=book_creator&referer=Turkish+Constitution+of+1921\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 980 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Special:Book&bookcmd=book_creator&referer=Vietnam+University+Admission+Rankings\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 868 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Special:Book&bookcmd=book_creator&referer=Vy%C5%A1n%C3%BD+Slavkov\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 1004 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Special:Book&bookcmd=render_article&arttitle=Ambrose+Dyson&oldid=562636681&writer=rl\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 996 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Special:Book&bookcmd=render_article&arttitle=Caron+Butler&oldid=571697645&writer=rl\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 1012 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Special:Book&bookcmd=render_article&arttitle=DDT+%28gene%29&oldid=453962249&writer=rl\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 1036 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Special:Book&bookcmd=render_article&arttitle=Johann+David+Wyss&oldid=568638498&writer=rl\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 972 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Special:Book&bookcmd=render_article&arttitle=Lady+Anne&oldid=372343156&writer=rl\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 1148 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Special:Book&bookcmd=render_article&arttitle=Sir+Massey+Lopes%2C+3rd+Baronet&oldid=545042390&writer=rl\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 948 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Special:Book&bookcmd=render_article&arttitle=Tumula&oldid=401070860&writer=rl\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 1124 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Special:Book&bookcmd=render_article&arttitle=Turkish+Constitution+of+1921&oldid=566504847&writer=rl\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 1196 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Special:Book&bookcmd=render_article&arttitle=Vietnam+University+Admission+Rankings&oldid=568532236&writer=rl\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 1084 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Special:Book&bookcmd=render_article&arttitle=Vy%C5%A1n%C3%BD+Slavkov&oldid=541325469&writer=rl\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 672 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Special:Cite&page=Ambrose_Dyson&id=562636681\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 664 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Special:Cite&page=Caron_Butler&id=571697645\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 680 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Special:Cite&page=DDT_%28gene%29&id=453962249\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 704 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Special:Cite&page=Johann_David_Wyss&id=568638498\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 640 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Special:Cite&page=Lady_Anne&id=372343156\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 816 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Special:Cite&page=Sir_Massey_Lopes%2C_3rd_Baronet&id=545042390\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 616 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Special:Cite&page=Tumula&id=401070860\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 792 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Special:Cite&page=Turkish_Constitution_of_1921&id=566504847\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 864 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Special:Cite&page=Vietnam_University_Admission_Rankings&id=568532236\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 752 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Special:Cite&page=Vy%C5%A1n%C3%BD_Slavkov&id=541325469\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 668 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Special:UserLogin&returnto=Ambrose+Dyson\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 764 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Special:UserLogin&returnto=Ambrose+Dyson&type=signup\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 660 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Special:UserLogin&returnto=Caron+Butler\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 756 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Special:UserLogin&returnto=Caron+Butler&type=signup\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 676 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Special:UserLogin&returnto=DDT+%28gene%29\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 772 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Special:UserLogin&returnto=DDT+%28gene%29&type=signup\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 700 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Special:UserLogin&returnto=Johann+David+Wyss\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 796 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Special:UserLogin&returnto=Johann+David+Wyss&type=signup\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 636 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Special:UserLogin&returnto=Lady+Anne\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 732 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Special:UserLogin&returnto=Lady+Anne&type=signup\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 812 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Special:UserLogin&returnto=Sir+Massey+Lopes%2C+3rd+Baronet\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 908 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Special:UserLogin&returnto=Sir+Massey+Lopes%2C+3rd+Baronet&type=signup\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 612 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Special:UserLogin&returnto=Tumula\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 708 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Special:UserLogin&returnto=Tumula&type=signup\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 788 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Special:UserLogin&returnto=Turkish+Constitution+of+1921\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 884 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Special:UserLogin&returnto=Turkish+Constitution+of+1921&type=signup\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 860 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Special:UserLogin&returnto=Vietnam+University+Admission+Rankings\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 956 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Special:UserLogin&returnto=Vietnam+University+Admission+Rankings&type=signup\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 748 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Special:UserLogin&returnto=Vy%C5%A1n%C3%BD+Slavkov\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 844 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Special:UserLogin&returnto=Vy%C5%A1n%C3%BD+Slavkov&type=signup\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 692 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Special:WhatLinksHere/Lady_Anne&namespace=0\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 636 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Talk:Lady_Anne&action=edit&redlink=1\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 628 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Template:2002_NBA_Draft&action=edit\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 1028 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Template:Big_East_Conference_Men%27s_Basketball_Player_of_the_Year_navbox&action=edit\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 636 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Template:Cartoonist-stub&action=edit\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 692 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Template:Constitution_of_Turkey&action=edit\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 872 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Template:Dallas_Mavericks_2010%E2%80%9311_NBA_champions&action=edit\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 612 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Template:Gene-22-stub&action=edit\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 676 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Template:Levo%C4%8Da_District&action=edit\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 756 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Template:Milwaukee_Bucks_current_roster&action=edit\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 636 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Template:Noctuoidea-stub&action=edit\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 580 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Template:PBB/1652&action=edit\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 604 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Template:PDB_Gallery&action=edit\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 660 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Template:Politics_of_Turkey&action=edit\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 676 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Template:Pre%C5%A1ov-geo-stub&action=edit\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 700 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Template:Switzerland-writer-stub&action=edit\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 716 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Template:The_Swiss_Family_Robinson&action=edit\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 700 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Template_talk:PBB/1652&action=edit&redlink=1\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 836 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Template_talk:The_Swiss_Family_Robinson&action=edit&redlink=1\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 492 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Tumula&action=edit\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 572 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Tumula&action=edit&section=1\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 516 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Tumula&action=history\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 492 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Tumula&action=info\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 496 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Tumula&oldid=401070860\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 508 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Tumula&printable=yes\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 508 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Tumula&veaction=edit\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 588 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Tumula&veaction=edit&section=1\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 668 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Turkish_Constitution_of_1921&action=edit\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 748 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Turkish_Constitution_of_1921&action=edit&section=1\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 692 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Turkish_Constitution_of_1921&action=history\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 668 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Turkish_Constitution_of_1921&action=info\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 672 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Turkish_Constitution_of_1921&oldid=566504847\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 684 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Turkish_Constitution_of_1921&printable=yes\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 684 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Turkish_Constitution_of_1921&veaction=edit\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 764 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Turkish_Constitution_of_1921&veaction=edit&section=1\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 724 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Turkish_Court_of_Accounts&action=edit&redlink=1\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 740 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Vietnam_University_Admission_Rankings&action=edit\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 820 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Vietnam_University_Admission_Rankings&action=edit&section=1\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 764 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Vietnam_University_Admission_Rankings&action=history\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 740 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Vietnam_University_Admission_Rankings&action=info\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 744 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Vietnam_University_Admission_Rankings&oldid=568532236\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 756 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Vietnam_University_Admission_Rankings&printable=yes\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 756 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Vietnam_University_Admission_Rankings&veaction=edit\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 836 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Vietnam_University_Admission_Rankings&veaction=edit&section=1\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 628 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Vy%C5%A1n%C3%BD_Slavkov&action=edit\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 708 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Vy%C5%A1n%C3%BD_Slavkov&action=edit&section=1\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 652 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Vy%C5%A1n%C3%BD_Slavkov&action=history\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 628 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Vy%C5%A1n%C3%BD_Slavkov&action=info\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 632 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Vy%C5%A1n%C3%BD_Slavkov&oldid=541325469\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 644 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Vy%C5%A1n%C3%BD_Slavkov&printable=yes\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 644 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Vy%C5%A1n%C3%BD_Slavkov&veaction=edit\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 724 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Vy%C5%A1n%C3%BD_Slavkov&veaction=edit&section=1\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 568 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/w/index.php?title=Vy\xC5\xA1n\xC3\xBD_Slavkov&oldid=541325469\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 525 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/%C4%90%E1%BA%AFk_L%E1%BA%AFk_Province\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 469 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/%C4%90%E1%BB%93ng_Nai_Province\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 685 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/2001%E2%80%9302_NCAA_Division_I_men%27s_basketball_season\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 493 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/2002%E2%80%9303_Miami_Heat_season\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 437 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/2002%E2%80%9303_NBA_season\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 356 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/2002_NBA_Draft\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 356 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/2002_NBA_draft\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 493 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/2003%E2%80%9304_Miami_Heat_season\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 437 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/2003%E2%80%9304_NBA_season\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 557 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/2004%E2%80%9305_Los_Angeles_Lakers_season\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 437 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/2004%E2%80%9305_NBA_season\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 380 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/2004_NBA_Playoffs\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 437 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/2005%E2%80%9306_NBA_season\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 557 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/2005%E2%80%9306_Washington_Wizards_season\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 437 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/2006%E2%80%9307_NBA_season\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 557 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/2006%E2%80%9307_Washington_Wizards_season\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 364 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/2006_NBA_Finals\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 380 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/2006_NBA_Playoffs\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 437 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/2007%E2%80%9308_NBA_season\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 557 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/2007%E2%80%9308_Washington_Wizards_season\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 420 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/2007_NBA_All-Star_Game\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 437 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/2008%E2%80%9309_NBA_season\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 557 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/2008%E2%80%9309_Washington_Wizards_season\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 420 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/2008_NBA_All-Star_Game\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 380 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/2008_NBA_Playoffs\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 541 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/2009%E2%80%9310_Dallas_Mavericks_season\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 437 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/2009%E2%80%9310_NBA_season\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 557 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/2009%E2%80%9310_Washington_Wizards_season\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 541 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/2010%E2%80%9311_Dallas_Mavericks_season\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 437 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/2010%E2%80%9311_NBA_season\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 380 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/2010_NBA_Playoffs\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 573 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/2011%E2%80%9312_Los_Angeles_Clippers_season\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 437 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/2011%E2%80%9312_NBA_season\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 364 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/2011_NBA_Finals\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 380 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/2011_NBA_Playoffs\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 573 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/2012%E2%80%9313_Los_Angeles_Clippers_season\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 437 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/2012%E2%80%9313_NBA_season\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 380 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/2012_NBA_Playoffs\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 437 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/2013%E2%80%9314_NBA_season\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 380 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/2013_NBA_Playoffs\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 380 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Abdullah_G%C3%BCl\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 572 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Accession_of_Turkey_to_the_European_Union\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 308 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Adelaide\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 332 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Albert_Pell\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 308 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Alderman\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 396 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Alfredton,_Victoria\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 340 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/All_American\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 412 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Allies_of_World_War_I\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 364 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Alonzo_Mourning\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 396 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Amar%27e_Stoudemire\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 420 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Amateur_Athletic_Union\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 348 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Ambrose_Dyson\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 356 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Ambrose_Dyson#\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 444 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Ambrose_Dyson#cite_note-2\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 516 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Ambrose_Dyson#cite_note-lindesay-1\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 436 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Ambrose_Dyson#cite_ref-2\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 524 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Ambrose_Dyson#cite_ref-lindesay_1-0\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 524 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Ambrose_Dyson#cite_ref-lindesay_1-1\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 460 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Ambrose_Dyson#mw-navigation\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 420 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Ambrose_Dyson#p-search\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 292 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Animal\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 580 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Anne,_Duchess_of_Cumberland_and_Strathearn\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 324 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Anne_Bacon\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 332 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Anne_Brewis\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 492 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Anne_Conway,_Viscountess_Conway\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 340 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Anne_Halkett\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 316 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Anne_Hyde\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 340 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Anne_Lambton\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 500 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Anne_Lennard,_Countess_of_Sussex\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 500 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Anne_Stanley,_Countess_of_Ancram\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 540 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Anne_Stanley,_Countess_of_Castlehaven\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 332 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Anne_Wilson\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 356 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Antawn_Jamison\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 276 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Area\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 316 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Arthropod\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 396 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Assist_(basketball)\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 436 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Atat%C3%BCrk%27s_Reforms\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 380 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Authority_control\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 669 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/B%C3%A0_R%E1%BB%8Ba%E2%80%93V%C5%A9ng_T%C3%A0u_Province\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 520 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/B%C3%ACnh_%C4%90%E1%BB%8Bnh_Province\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 528 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/B%C3%ACnh_Ph%C6%B0%E1%BB%9Bc_Province\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 450 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/B%E1%BA%AFc_Giang_Province\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 442 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/B%E1%BA%AFc_Ninh_Province\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 308 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Baldovce\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 308 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Ballarat\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 388 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Ballarat,_Victoria\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 364 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Baron_Roborough\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 324 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Basketball\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 316 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Beharovce\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 356 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Ben_Hansbrough\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 380 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Benjamin_Disraeli\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 476 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Beverly_Hills_Family_Robinson\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 539 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Big_East_Conference_(1979%E2%80%932013)\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 700 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Big_East_Conference_Men%27s_Basketball_Player_of_the_Year\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 316 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Bijacovce\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 644 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Bill_Russell_NBA_Finals_Most_Valuable_Player_Award\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 332 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Billy_Owens\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 444 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Biological_classification\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 388 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Block_(basketball)\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 284 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Blogs\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 404 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Bo%C5%A1tjan_Nachbar\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 324 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Bob_Bender\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 356 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Brandin_Knight\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 460 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Brandon_Knight_(basketball)\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 364 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Brendan_Haywood\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 356 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Brian_Cardinal\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 332 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Brian_Grant\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 308 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Brutovce\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 308 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Buglovce\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 332 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Burger_King\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 340 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/C._J._Dennis\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 380 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Cabinet_of_Turkey\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 292 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Caliph\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 316 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Caliphate\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 372 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Carlo_Pellegrini\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 348 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Carlos_Boozer\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 356 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Carlos_Delfino\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 340 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Caron_Butler\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 348 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Caron_Butler#\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 460 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Caron_Butler#College_career\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 428 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Caron_Butler#Early_life\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 460 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Caron_Butler#External_links\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 428 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Caron_Butler#NBA_career\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 516 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Caron_Butler#NBA_career_statistics\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 452 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Caron_Butler#Personal_life\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 412 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Caron_Butler#Playoffs\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 428 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Caron_Butler#References\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 460 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Caron_Butler#Regular_season\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 444 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Caron_Butler#cite_note-10\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 436 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Caron_Butler#cite_note-5\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 524 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Caron_Butler#cite_note-AllStories-7\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 466 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Caron_Butler#cite_note-BK-20\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 492 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Caron_Butler#cite_note-oprah1-1\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 492 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Caron_Butler#cite_note-oprah2-2\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 492 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Caron_Butler#cite_note-straws-3\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 524 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Caron_Butler#cite_note-toughjuice-8\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 516 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Caron_Butler#cite_note-washtimes-9\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 556 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Caron_Butler#cite_note-wp-greatescape-4\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 436 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Caron_Butler#cite_ref-10\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 428 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Caron_Butler#cite_ref-5\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 532 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Caron_Butler#cite_ref-AllStories_7-0\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 532 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Caron_Butler#cite_ref-AllStories_7-1\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 476 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Caron_Butler#cite_ref-BK_20-0\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 476 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Caron_Butler#cite_ref-BK_20-1\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 500 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Caron_Butler#cite_ref-oprah1_1-0\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 500 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Caron_Butler#cite_ref-oprah2_2-0\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 500 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Caron_Butler#cite_ref-straws_3-0\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 500 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Caron_Butler#cite_ref-straws_3-1\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 500 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Caron_Butler#cite_ref-straws_3-2\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 532 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Caron_Butler#cite_ref-toughjuice_8-0\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 524 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Caron_Butler#cite_ref-washtimes_9-0\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 564 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Caron_Butler#cite_ref-wp-greatescape_4-0\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 452 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Caron_Butler#mw-navigation\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 412 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Caron_Butler#p-search\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 324 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Cartoonist\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 356 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Casey_Jacobsen\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 404 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:1743_births\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 404 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:1818_births\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 404 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:1818_deaths\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 404 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:1876_births\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 404 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:1908_deaths\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 404 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:1913_deaths\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 524 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:1921_in_the_Ottoman_Empire\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 404 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:1980_births\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 492 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:19th-century_novelists\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 576 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:Accuracy_disputes_from_March_2012\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 596 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:African-American_basketball_players\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 572 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:All_article_disambiguation_pages\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 636 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:All_articles_lacking_reliable_references\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 652 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:All_articles_needing_additional_references\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 572 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:All_articles_needing_coordinates\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 532 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:All_articles_to_be_expanded\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 620 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:All_articles_with_unsourced_statements\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 508 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:All_disambiguation_pages\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 620 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:All_pages_needing_factual_verification\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 644 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:Articles_containing_Turkish-language_text\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 752 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:Articles_lacking_reliable_references_from_February_2013\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 760 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:Articles_needing_additional_references_from_January_2011\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 716 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:Articles_needing_translation_from_German_Wikipedia\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 632 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:Articles_to_be_expanded_from_August_2013\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 636 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:Articles_with_%27species%27_microformats\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 744 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:Articles_with_unsourced_statements_from_September_2010\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 492 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:Australian_cartoonists\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 500 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:Australian_illustrators\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 476 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:Australian_satirists\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 700 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:Baronets_in_the_Baronetage_of_the_United_Kingdom\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 580 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:Basketball_players_from_Wisconsin\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 444 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:Cartoonist_stubs\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 524 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:Cartoonists_from_Melbourne\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 508 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:Chromosome_22_gene_stubs\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 684 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:Connecticut_Huskies_men%27s_basketball_players\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 532 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:Conservative_Party_(UK)_MPs\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 500 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:Constitutions_of_Turkey\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 508 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:Dallas_Mavericks_players\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 484 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:Defunct_constitutions\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 476 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:Disambiguation_pages\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 492 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:High_Sheriffs_of_Devon\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 620 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:High_schools_for_the_gifted_in_Vietnam\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 500 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:High_schools_in_Vietnam\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 428 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:Human_proteins\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 420 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:Living_people\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 540 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:Los_Angeles_Clippers_players\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 524 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:Los_Angeles_Lakers_players\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 852 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:Members_of_the_United_Kingdom_Parliament_for_English_constituencies\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 492 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:Miami_Heat_draft_picks\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 460 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:Miami_Heat_players\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 428 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:Micronoctuidae\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 644 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:National_Basketball_Association_All-Stars\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 444 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:Noctuoidea_stubs\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 556 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:Pages_with_no_translate_target\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 444 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:People_from_Bern\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 548 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:People_from_Racine,_Wisconsin\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 588 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:Pre%C5%A1ov_Region_geography_stubs\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 460 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:Schools_in_Vietnam\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 436 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:Shooting_guards\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 428 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:Small_forwards\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 386 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:Spi%C5%A1\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 524 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:Swiss_children%27s_writers\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 436 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:Swiss_novelists\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 460 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:Swiss_writer_stubs\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 479 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:UK_MPs_1857%E2%80%931859\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 479 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:UK_MPs_1859%E2%80%931865\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 479 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:UK_MPs_1868%E2%80%931874\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 479 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:UK_MPs_1874%E2%80%931880\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 479 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:UK_MPs_1880%E2%80%931885\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 548 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:United_States_Navy_ship_names\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 592 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:Use_British_English_from_March_2012\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 560 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:Use_dmy_dates_from_January_2012\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 536 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:Use_dmy_dates_from_July_2013\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 660 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:Vietnam_articles_missing_geocoordinate_data\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 724 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:Villages_and_municipalities_in_Levo%C4%8Da_District\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 524 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:Washington_Wizards_players\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 964 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:Wikipedia_articles_incorporating_an_LRPP-Bt_template_without_an_unnamed_parameter\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 816 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:Wikipedia_articles_needing_factual_verification_from_March_2012\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 848 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:Wikipedia_articles_needing_page_number_citations_from_February_2013\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 636 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Category:Wikipedia_articles_with_VIAF_identifiers\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 412 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Cemil_%C3%87i%C3%A7ek\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 540 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Charles_Smith_(basketball,_born_1965)\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 540 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Charles_Smith_(basketball,_born_1967)\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 364 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Chris_Jefferies\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 444 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Chris_Mullin_(basketball)\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 436 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Chris_Owens_(basketball)\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 340 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Chris_Wilcox\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 348 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Chromosome_22\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 348 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Chucky_Atkins\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 380 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Clare_Sewell_Read\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 396 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Cleveland_Cavaliers\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 548 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Connecticut_Huskies_men%27s_basketball\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 499 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Conservative_Government_1874-1880\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 428 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Conservative_Party_(UK)\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 340 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Constitution\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 420 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Constitution_of_Turkey\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 484 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Constitutional_Court_of_Turkey\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 500 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Constitutional_history_of_Turkey\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 428 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Constitutional_monarchy\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 340 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Corey_Brewer\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 364 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Corsley_Edwards\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 460 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Court_of_Cassation_(Turkey)\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 372 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Curtis_Borchardt\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 516 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/D%C3%BAbrava,_Levo%C4%8Da_District\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 324 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/DDT_(gene)\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 332 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/DDT_(gene)#\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 476 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/DDT_(gene)#cite_note-entrez-3\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 506 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/DDT_(gene)#cite_note-pmid9480844-1\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 484 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/DDT_(gene)#cite_ref-entrez_3-0\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 484 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/DDT_(gene)#cite_ref-entrez_3-1\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 518 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/DDT_(gene)#cite_ref-pmid9480844_1-0\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 436 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/DDT_(gene)#mw-navigation\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 396 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/DDT_(gene)#p-search\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 300 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Da_Nang\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 348 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Dajuan_Wagner\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 372 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Dallas_Mavericks\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 372 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Dan_Callandrillo\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 324 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Dan_Dickau\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 340 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Dan_Gadzuric\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 340 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Daniel_Defoe\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 364 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Darius_Songaila\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 380 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Darrell_Armstrong\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 300 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Das_Bus\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 356 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/David_Andersen\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 340 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/DeJuan_Blair\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 380 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/DeShawn_Stevenson\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 364 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Derrick_Coleman\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 380 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Deshawn_Stevenson\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 444 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Digital_object_identifier\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 348 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Dirk_Nowitzki\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 447 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Dlh%C3%A9_Str%C3%A1%C5%BEe\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 508 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Do%C4%BEany,_Levo%C4%8Da_District\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 356 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Doma%C5%88ovce\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 364 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Dominique_Jones\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 372 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Donyell_Marshall\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 292 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Dravce\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 332 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Drew_Gooden\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 364 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Duke_University\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 332 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Dwane_Casey\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 436 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Eastern_Conference_(NBA)\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 380 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Economy_of_Turkey\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 444 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Eddie_Jordan_(basketball)\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 404 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Edward_Ambrose_Dyson\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 340 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Edward_Dyson\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 316 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Ekpe_Udoh\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 396 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Elections_in_Turkey\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 540 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Elections_in_Turkey#General_elections\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 524 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Elections_in_Turkey#Local_elections\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 316 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Elevation\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 300 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Elite_8\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 340 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Emeka_Okafor\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 300 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Ensembl\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 356 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Entente_powers\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 292 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Entrez\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 292 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Enzyme\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 436 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Enzyme_Commission_number\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 436 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Erdo%C4%9Fan_III_Cabinet\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 340 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Eric_Bledsoe\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 396 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Ersan_%C4%B0lyasova\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 580 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Establishment_of_Turkish_national_movement\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 604 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/European_Union_%E2%80%93_Turkey_Customs_Union\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 420 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Executive_(government)\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 500 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/FIBA_Under-21_World_Championship\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 372 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Family_(biology)\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 396 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Federico_Kammerichs\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 428 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Field_goal_(basketball)\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 484 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/File:A_coloured_voting_box.svg\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 508 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/File:Bryant_Fades_Over_Butler.jpg\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 412 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/File:Caron_Butler.JPG\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 428 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/File:Creatorballoon.png\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 380 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/File:DNA_stub.png\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 420 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/File:Disambig_gray.svg\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 428 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/File:Flag_of_Turkey.svg\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 610 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/File:Massey_Lopes,_Vanity_Fair,_1875-05-15.jpg\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 468 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/File:Noctua.pronuba.7199.jpg\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 412 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/File:Okres_levoca.png\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 514 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/File:PBB_GE_DDT_202929_s_at_tn.png\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 412 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/File:PDB_1dpt_EBI.jpg\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 356 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/File:P_vip.svg\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 476 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/File:Protein_DDT_PDB_1dpt.png\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 420 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/File:Quill_and_ink.svg\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 460 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/File:Slovakiatynk%C3%A4.svg\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 412 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/File:Speaker_Icon.svg\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 540 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/File:Translation_to_english_arrow.svg\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 436 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/File:Wikisource-logo.svg\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 524 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Financial_Secretary_to_the_Treasury\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 524 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Foreign_Language_Specialized_School\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 460 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Foreign_relations_of_Turkey\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 564 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Francis_Cottington,_1st_Baron_Cottington\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 460 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Frank_Williams_(basketball)\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 428 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Fred_Jones_(basketball)\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 324 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Free_throw\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 456 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/French_Constitution_of_1791\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 316 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Gary_Neal\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 276 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Gene\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 316 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/GeneCards\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 348 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Gene_Ontology\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 284 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Genus\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 468 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Geographic_coordinate_system\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 412 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Geography_of_Slovakia\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 412 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Giannis_Antetokounmpo\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 356 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Gilbert_Arenas\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 396 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Gran%C4%8D-Petrovce\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 508 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Grand_National_Assembly_of_Turkey\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 412 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Great_Western_Railway\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 404 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/H%C3%A0_Nam_Province\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 452 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/H%C3%A0_T%C4%A9nh_Province\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 460 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/H%C6%B0ng_Y%C3%AAn_Province\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 523 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/H%E1%BA%A3i_D%C6%B0%C6%A1ng_Province\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 316 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Hai_Phong\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 348 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Hakim_Warrick\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 284 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Hanoi\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 540 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Hanoi_%E2%80%93_Amsterdam_High_School\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 316 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Harakovce\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 364 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Hasheem_Thabeet\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 348 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Help:Category\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 348 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Help:Contents\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 396 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Help:Disambiguation\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 396 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Help:IPA_for_German\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 516 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Help:Introduction_to_referencing/1\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 524 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Henry_Chaplin,_1st_Viscount_Chaplin\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 476 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Henry_Lopes,_1st_Baron_Ludlow\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 500 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Henry_Lopes,_1st_Baron_Roborough\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 604 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Her_Majesty%27s_Most_Honourable_Privy_Council\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 812 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/High_School_for_Gifted_Students,_Hanoi_National_University_of_Education\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 724 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/High_School_for_Gifted_Students,_Hanoi_University_of_Science\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 452 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/High_School_for_the_Gifted\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 412 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/High_Sheriff_of_Devon\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 300 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/History\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 380 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/History_of_Turkey\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 572 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/History_of_Turkish_presidential_elections\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 372 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Ho_Chi_Minh_City\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 324 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/HomoloGene\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 444 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Human_Genome_Organisation\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 332 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Ian_Mahinmi\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 292 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Insect\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 372 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Internet_Archive\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 316 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Ish_Smith\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 332 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Islamic_Law\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 340 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/J._J._Redick\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 308 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Jablonov\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 332 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Jae_Crowder\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 348 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Jamal_Sampson\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 468 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/James_Singleton_(basketball)\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 468 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/James_Wilson_(UK_politician)\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 340 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Jared_Dudley\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 356 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Jared_Jeffries\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 460 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Jason_Jennings_(basketball)\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 324 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Jason_Kidd\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 332 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Jason_Terry\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 444 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Jay_Williams_(basketball)\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 428 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Jeff_Green_(basketball)\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 407 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Ji%C5%99%C3%AD_Welsch\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 332 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Jim_Calhoun\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 340 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Jim_Cleamons\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 380 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Johann_David_Wyss\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 388 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Johann_David_Wyss#\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 476 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Johann_David_Wyss#cite_note-1\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 468 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Johann_David_Wyss#cite_ref-1\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 492 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Johann_David_Wyss#mw-navigation\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 452 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Johann_David_Wyss#p-search\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 388 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Johann_Rudolf_Wyss\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 436 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/John_Bagley_(basketball)\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 420 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/John_Carpenter_Garnier\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 324 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/John_Duren\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 436 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/John_Henson_(basketball)\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 380 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/John_Lewis_Phipps\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 492 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/John_Russell,_Viscount_Amberley\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 340 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/John_Salmons\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 460 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/John_Tremayne_(1825_-_1901)\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 540 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/John_Yarde-Buller,_1st_Baron_Churston\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 404 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Jos%C3%A9_Juan_Barea\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 332 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Josh_Howard\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 500 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Juan_Carlos_Navarro_(basketball)\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 324 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Juan_Dixon\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 332 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Kareem_Rush\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 543 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Kemal_K%C4%B1l%C4%B1%C3%A7daro%C4%9Flu\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 348 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Kerry_Kittles\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 324 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Kew_Asylum\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 468 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Kh%C3%A1nh_H%C3%B2a_Province\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 364 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Khris_Middleton\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 324 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Kl%C4%8Dov\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 332 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Kobe_Bryant\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 338 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Korytn%C3%A9\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 308 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Kurimany\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 332 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Kwame_Brown\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 530 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/L%C3%BA%C4%8Dka,_Levo%C4%8Da_District\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 316 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Lady_Anne\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 324 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Lady_Anne#\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 484 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Lady_Anne#Fictional_characters\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 372 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Lady_Anne#People\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 388 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Lady_Anne#See_also\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 364 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Lady_Anne#Ships\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 428 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Lady_Anne#mw-navigation\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 388 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Lady_Anne#p-search\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 412 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Lady_Anne-Marie_Byrne\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 380 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Lady_Anne_Barnard\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 364 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Lady_Anne_Berry\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 364 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Lady_Anne_Blunt\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 468 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Lady_Anne_Cavendish-Bentinck\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 396 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Lady_Anne_Churchill\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 388 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Lady_Anne_Clifford\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 500 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Lady_Anne_Farquharson-MacKintosh\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 356 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Lady_Anne_Rhys\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 396 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Lam_Son_High_School\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 324 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Lamar_Odom\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 340 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Laron_Profit\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 324 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Larry_Drew\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 452 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Larry_Sanders_(basketball)\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 492 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Lawrence_Palk,_1st_Baron_Haldon\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 340 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/LeBron_James\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 444 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Le_Hong_Phong_High_School\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 556 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Leader_of_the_Main_Opposition_of_Turkey\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 548 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Legal_System_in_the_Republic_of_Turkey\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 756 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Legal_System_in_the_Republic_of_Turkey#Turkish_Court_of_Accounts\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 548 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Legal_system_of_the_Republic_of_Turkey\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 332 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Legislative\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 332 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Lepidoptera\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 332 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Levo%C4%8Da\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 404 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Levo%C4%8Da_District\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 308 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/LibriVox\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 412 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/List_of_NBA_champions\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 468 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/List_of_Presidents_of_Turkey\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 692 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/List_of_Presidents_of_the_Constitutional_Court_of_Turkey\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 508 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/List_of_Prime_Ministers_of_Turkey\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 596 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/List_of_Speakers_of_the_Parliament_of_Turkey\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 444 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/List_of_Turkish_diplomats\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 540 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/List_of_diplomatic_missions_of_Turkey\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 452 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/List_of_extant_baronetcies\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 524 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/List_of_political_parties_in_Turkey\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 372 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Long_An_Province\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 340 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Lonny_Baxter\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 516 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Lord_Commissioner_of_the_Admiralty\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 404 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Los_Angeles_Clippers\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 388 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Los_Angeles_Lakers\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 348 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Lost_in_Space\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 324 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Luis_Scola\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 356 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Luke_Harangody\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 340 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Luke_Ridnour\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 548 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Macrophage_migration_inhibitory_factor\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 604 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Macrophage_migration_inhibitory_factor_domain\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 316 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Main_Page\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 428 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Maine_Central_Institute\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 356 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Marcus_Haislip\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 348 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Marcus_Taylor\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 332 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Mario_Kasun\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 388 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Maryland_Terrapins\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 332 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Matt_Barnes\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 372 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Melbourne_Herald\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 364 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Melbourne_Punch\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 324 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Melvin_Ely\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 468 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Mendelian_Inheritance_in_Man\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 324 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Miami_Heat\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 356 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Micronoctuidae\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 388 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Mike_Dunleavy,_Jr.\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 426 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Milo%C5%A1_Vujani%C4%87\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 364 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Milwaukee_Bucks\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 404 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Ministries_of_Turkey\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 596 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Ministry_of_Education_and_Training_(Vietnam)\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 532 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Ministry_of_Foreign_Affairs_(Turkey)\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 388 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Miroslav_Raduljica\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 412 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Mladen_%C5%A0ekularac\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 300 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Monarch\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 276 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Moth\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 340 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Mountain_Dew\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 436 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Mouse_Genome_Informatics\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 340 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Municipality\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 452 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Mustafa_Kemal_Atat%C3%BCrk\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 268 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/NBA\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 396 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/NBA_All-Rookie_Team\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 380 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/NBA_All-Star_Game\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 292 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Nahiye\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 472 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Nam_%C4%90%E1%BB%8Bnh_Province\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 340 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Nate_Wolters\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 540 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/National_Basketball_Association_draft\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 516 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/National_Security_Council_(Turkey)\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 404 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/National_sovereignty\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 348 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Neme%C5%A1any\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 314 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Nen%C3%AA\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 378 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Nenad_Krsti%C4%87\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 420 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/New_Orleans,_Louisiana\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 364 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/New_York_Knicks\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 436 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Ngh%E1%BB%87_An_Province\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 484 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Nguyen_Thuong_Hien_High_School\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 460 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Ni%C5%BEn%C3%A9_Repa%C5%A1e\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 348 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Nick_Van_Exel\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 404 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Nikoloz_Tskitishvili\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 324 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Noctuoidea\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 378 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/O%C4%BE%C5%A1avica\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 324 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/O._J._Mayo\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 340 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Open_Library\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 316 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Ordzovany\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 412 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Oriel_College,_Oxford\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 332 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Otto_Porter\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 356 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Ottoman_Empire\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 364 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Ottoman_dynasty\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 372 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Ottoman_language\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 620 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Outline_of_political_science#Politics_by_region\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 500 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Parliament_of_the_United_Kingdom\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 332 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Pat_Garrity\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 348 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Patrick_Ewing\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 340 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Pav%C4%BEany\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 402 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Peja_Stojakovi%C4%87\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 388 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Perils_of_the_Wild\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 332 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Peter_Fehse\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 476 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Ph%C3%BA_Th%E1%BB%8D_Province\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 452 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Ph%C3%BA_Y%C3%AAn_Province\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 340 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Phoenix_Suns\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 380 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Pittsfield,_Maine\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 356 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Po%C4%BEanovce\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 388 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Politics_of_Turkey\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 524 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Politics_of_Turkey#Executive_branch\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 540 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Politics_of_Turkey#Legislative_branch\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 372 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Pongr%C3%A1covce\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 324 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Population\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 372 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Portal:Biography\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 364 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Portal:Contents\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 412 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Portal:Current_events\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 428 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Portal:Featured_content\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 364 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Portal:Politics\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 348 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Portal:Turkey\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 388 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Pre%C5%A1ov_Region\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 396 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/President_of_Turkey\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 436 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Prime_Minister_of_Turkey\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 380 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Project_Gutenberg\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 380 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Protein_Data_Bank\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 292 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/PubMed\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 356 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/PubMed_Central\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 380 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/PubMed_Identifier\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 372 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Public_relations\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 284 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Qadaa\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 460 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Qu%E1%BA%A3ng_Ninh_Province\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 340 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Quinton_Ross\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 636 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Quoc_Hoc_%E2%80%93_Hue_High_School_for_the_Gifted\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 340 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Qyntel_Woods\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 380 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Racine,_Wisconsin\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 428 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Racine_Park_High_School\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 324 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Randy_Foye\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 348 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Randy_Holcomb\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 348 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Rasual_Butler\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 316 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Ray_Allen\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 404 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Rebound_(basketball)\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 444 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Recep_Tayyip_Erdo%C4%9Fan\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 556 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Reggie_Williams_(basketball,_born_1964)\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 308 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Republic\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 388 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Republic_of_Turkey\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 476 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Richard_Hamilton_(basketball)\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 348 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Rick_Carlisle\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 372 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Robert_Archibald\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 604 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Robert_Haldane-Duncan,_3rd_Earl_of_Camperdown\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 364 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Robinson_Crusoe\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 340 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Rod_Grizzard\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 380 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Rodrigue_Beaubois\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 372 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Roger_Mason,_Jr.\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 348 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Ronald_Murray\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 348 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Ryan_Humphrey\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 364 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Sam_Clancy,_Jr.\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 436 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Samuel_Trehawke_Kekewich\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 380 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/San_Antonio_Spurs\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 460 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Scott_Williams_(basketball)\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 404 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Secularism_in_Turkey\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 348 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Sephardi_Jews\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 388 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Shaquille_O%27Neal\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 340 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Shawn_Marion\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 548 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Sir_Manasseh_Masseh_Lopes,_1st_Baronet\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 476 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Sir_Massey_Lopes,_3rd_Baronet\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 484 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Sir_Massey_Lopes,_3rd_Baronet#\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 572 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Sir_Massey_Lopes,_3rd_Baronet#cite_note-1\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 564 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Sir_Massey_Lopes,_3rd_Baronet#cite_ref-1\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 588 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Sir_Massey_Lopes,_3rd_Baronet#mw-navigation\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 548 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Sir_Massey_Lopes,_3rd_Baronet#p-search\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 468 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Sir_Ralph_Lopes,_2nd_Baronet\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 308 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Slovakia\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 348 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Small_forward\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 564 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/South_Devon_(UK_Parliament_constituency)\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 444 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/South_Melbourne,_Victoria\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 332 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Sovereignty\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 412 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Space_Family_Robinson\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 463 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Special:BookSources/0091354609\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 356 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Special:Random\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 412 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Special:RecentChanges\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 572 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Special:RecentChangesLinked/Ambrose_Dyson\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 564 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Special:RecentChangesLinked/Caron_Butler\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 548 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Special:RecentChangesLinked/DDT_(gene)\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 604 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Special:RecentChangesLinked/Johann_David_Wyss\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 540 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Special:RecentChangesLinked/Lady_Anne\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 700 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Special:RecentChangesLinked/Sir_Massey_Lopes,_3rd_Baronet\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 516 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Special:RecentChangesLinked/Tumula\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 688 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Special:RecentChangesLinked/Turkish_Constitution_of_1921\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 764 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Special:RecentChangesLinked/Vietnam_University_Admission_Rankings\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 652 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Special:RecentChangesLinked/Vy%C5%A1n%C3%BD_Slavkov\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 404 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Special:SpecialPages\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 524 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Special:WhatLinksHere/Ambrose_Dyson\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 516 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Special:WhatLinksHere/Caron_Butler\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 500 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Special:WhatLinksHere/DDT_(gene)\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 556 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Special:WhatLinksHere/Johann_David_Wyss\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 492 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Special:WhatLinksHere/Lady_Anne\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 652 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Special:WhatLinksHere/Sir_Massey_Lopes,_3rd_Baronet\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 468 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Special:WhatLinksHere/Tumula\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 640 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Special:WhatLinksHere/Turkish_Constitution_of_1921\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 716 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Special:WhatLinksHere/Vietnam_University_Admission_Rankings\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 604 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Special:WhatLinksHere/Vy%C5%A1n%C3%BD_Slavkov\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 460 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Spi%C5%A1sk%C3%A9_Podhradie\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 484 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Spi%C5%A1sk%C3%BD_%C5%A0tvrtok\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 428 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Spi%C5%A1sk%C3%BD_Hrhov\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 388 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Steal_(basketball)\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 332 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Steve_Logan\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 404 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Stranded_(2002_film)\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 492 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Studenec_(Levo%C4%8Da_District)\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 292 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Sultan\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 524 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Supreme_Electoral_Council_of_Turkey\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 444 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Swiss_Family_Guy_Robinson\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 508 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Swiss_Family_Robinson_(1940_film)\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 508 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Swiss_Family_Robinson_(1960_film)\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 548 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Swiss_Family_Robinson_(1974_TV_series)\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 332 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Switzerland\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 292 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Sydney\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 324 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Tactusinae\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 388 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Talk:Ambrose_Dyson\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 380 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Talk:Caron_Butler\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 364 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Talk:DDT_(gene)\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 420 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Talk:Johann_David_Wyss\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 516 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Talk:Sir_Massey_Lopes,_3rd_Baronet\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 332 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Talk:Tumula\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 504 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Talk:Turkish_Constitution_of_1921\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 580 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Talk:Vietnam_University_Admission_Rankings\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 468 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Talk:Vy%C5%A1n%C3%BD_Slavkov\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 324 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Tamar_Slay\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 364 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Tayshaun_Prince\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 428 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Template:2002_NBA_Draft\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 828 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Template:Big_East_Conference_Men%27s_Basketball_Player_of_the_Year_navbox\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 436 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Template:Cartoonist-stub\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 436 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Template:Citation_needed\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 492 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Template:Constitution_of_Turkey\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 672 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Template:Dallas_Mavericks_2010%E2%80%9311_NBA_champions\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 412 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Template:Gene-22-stub\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 476 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Template:Levo%C4%8Da_District\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 556 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Template:Milwaukee_Bucks_current_roster\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 436 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Template:Noctuoidea-stub\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 371 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Template:PBB/1652\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 404 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Template:PDB_Gallery\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 460 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Template:Politics_of_Turkey\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 476 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Template:Pre%C5%A1ov-geo-stub\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 500 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Template:Switzerland-writer-stub\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 516 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Template:The_Swiss_Family_Robinson\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 396 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Template:Translated\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 468 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Template_talk:2002_NBA_Draft\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 868 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Template_talk:Big_East_Conference_Men%27s_Basketball_Player_of_the_Year_navbox\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 476 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Template_talk:Cartoonist-stub\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 532 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Template_talk:Constitution_of_Turkey\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 712 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Template_talk:Dallas_Mavericks_2010%E2%80%9311_NBA_champions\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 452 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Template_talk:Gene-22-stub\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 516 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Template_talk:Levo%C4%8Da_District\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 596 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Template_talk:Milwaukee_Bucks_current_roster\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 476 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Template_talk:Noctuoidea-stub\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 444 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Template_talk:PDB_Gallery\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 500 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Template_talk:Politics_of_Turkey\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 516 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Template_talk:Pre%C5%A1ov-geo-stub\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 540 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Template_talk:Switzerland-writer-stub\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 340 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Terry_Dehere\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 340 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Terry_Stotts\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 468 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Th%C3%A1i_B%C3%ACnh_Province\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 660 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Th%E1%BB%ABa_Thi%C3%AAn%E2%80%93Hu%E1%BA%BF_Province\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 356 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Thanh_H%C3%B3a\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 428 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Thanh_H%C3%B3a_Province\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 340 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/The_Bulletin\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 444 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/The_Castaways_of_the_Flag\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 388 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/The_London_Gazette\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 444 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/The_Swiss_Family_Robinson\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 700 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/The_Swiss_Family_Robinson:_Flone_of_the_Mysterious_Island\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 580 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/The_Swiss_Family_Robinson_(1975_TV_series)\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 500 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Thomas_Brassey,_1st_Earl_Brassey\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 420 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Three-point_field_goal\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 340 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Tim_Grgurich\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 420 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Tim_James_(basketball)\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 332 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Tito_Maddox\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 324 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Tom_Durkin\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 300 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Torysky\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 452 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Tran_Dai_Nghia_High_School\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 388 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Treaty_of_Lausanne\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 412 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Treaty_of_S%C3%A8vres\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 316 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Troy_Bell\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 332 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Troy_Murphy\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 292 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Tumula\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 300 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Tumula#\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 404 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Tumula#mw-navigation\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 364 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Tumula#p-search\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 388 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Tumula_flavicollis\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 292 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Turkey\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 660 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Turkey%27s_membership_of_international_organizations\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 404 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Turkish_Armed_Forces\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 464 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Turkish_Constitution_of_1921\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 476 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Turkish_Constitution_of_1921#\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 556 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Turkish_Constitution_of_1921#Background\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 588 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Turkish_Constitution_of_1921#External_links\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 540 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Turkish_Constitution_of_1921#Overview\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 556 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Turkish_Constitution_of_1921#References\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 644 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Turkish_Constitution_of_1921#Text_.28as_enacted.29\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 540 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Turkish_Constitution_of_1921#Timeline\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 564 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Turkish_Constitution_of_1921#cite_note-1\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 556 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Turkish_Constitution_of_1921#cite_ref-1\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 580 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Turkish_Constitution_of_1921#mw-navigation\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 540 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Turkish_Constitution_of_1921#p-search\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 464 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Turkish_Constitution_of_1924\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 464 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Turkish_Constitution_of_1961\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 464 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Turkish_Constitution_of_1982\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 436 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Turkish_Council_of_State\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 436 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Turkish_Independence_War\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 460 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Turkish_War_of_Independence\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 480 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Turkish_general_election,_2011\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 372 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Turkish_language\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 472 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Turkish_local_elections,_2009\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 520 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Turkish_presidential_election,_2007\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 356 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Tyson_Chandler\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 420 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/USS_Lady_Anne_(SP-154)\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 324 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Ulo%C5%BEa\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 300 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/UniProt\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 348 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/United_States\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 548 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/United_States_national_basketball_team\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 444 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/University_of_Connecticut\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 468 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/V%C4%A9nh_Ph%C3%BAc_Province\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 567 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Vanity_Fair_(British_magazine_1868-1914)\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 396 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Viacheslav_Kravtsov\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 540 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Vietnam_University_Admission_Rankings\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 548 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Vietnam_University_Admission_Rankings#\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 636 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Vietnam_University_Admission_Rankings#cite_note-1\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 628 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Vietnam_University_Admission_Rankings#cite_ref-1\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 652 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Vietnam_University_Admission_Rankings#mw-navigation\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 612 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Vietnam_University_Admission_Rankings#p-search\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 340 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Vil%C3%A2yet\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 300 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Village\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 380 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Vincent_Yarbrough\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 532 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Virtual_International_Authority_File\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 460 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Vy%C5%A1n%C3%A9_Repa%C5%A1e\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 428 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Vy%C5%A1n%C3%BD_Slavkov\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 436 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Vy%C5%A1n%C3%BD_Slavkov#\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 540 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Vy%C5%A1n%C3%BD_Slavkov#mw-navigation\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 500 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Vy%C5%A1n%C3%BD_Slavkov#p-search\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 444 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Walter_Berry_(basketball)\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 372 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Washington,_D.C.\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 460 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Washington_Park_High_School\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 388 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Washington_Wizards\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 460 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Wesley_Johnson_(basketball)\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 540 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Westbury_(UK_Parliament_constituency)\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 364 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Wikipedia:About\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 444 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Wikipedia:Citation_needed\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 436 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Wikipedia:Citing_sources\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 452 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Wikipedia:Community_portal\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 404 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Wikipedia:Contact_us\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 516 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Wikipedia:Copying_within_Wikipedia\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 468 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Wikipedia:File_Upload_Wizard\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 468 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Wikipedia:General_disclaimer\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 548 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Wikipedia:Identifying_reliable_sources\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 362 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Wikipedia:NOTRS\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 404 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Wikipedia:Persondata\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 356 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Wikipedia:Stub\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 868 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Wikipedia:Text_of_Creative_Commons_Attribution-ShareAlike_3.0_Unported_License\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 412 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Wikipedia:Translation\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 372 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Wikipedia:V#SELF\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 580 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Wikipedia:Verifiability#Burden_of_evidence\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 324 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Will_Dyson\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 388 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Winchester_College\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 316 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Wisconsin\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 332 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/World_War_I\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 292 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Yahoo!\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 308 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Yao_Ming\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 348 as libc::c_int as size_t,
            url: b"http://en.wikipedia.org/wiki/Zaza_Pachulia\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 556 as libc::c_int as size_t,
            url: b"http://en.wikisource.org/wiki/Constitution_of_the_Republic_of_Turkey\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 428 as libc::c_int as size_t,
            url: b"http://eo.wikipedia.org/wiki/Vy%C5%A1n%C3%BD_Slavkov\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 340 as libc::c_int as size_t,
            url: b"http://es.wikipedia.org/wiki/Caron_Butler\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 380 as libc::c_int as size_t,
            url: b"http://es.wikipedia.org/wiki/Johann_David_Wyss\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 228 as libc::c_int as size_t,
            url: b"http://everything.yahoo.com\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 236 as libc::c_int as size_t,
            url: b"http://everything.yahoo.com/\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 420 as libc::c_int as size_t,
            url: b"http://feedback.yahoo.com/forums/206380-us-homepage\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 204 as libc::c_int as size_t,
            url: b"http://finance.yahoo.com\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 212 as libc::c_int as size_t,
            url: b"http://finance.yahoo.com/\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 602 as libc::c_int as size_t,
            url: b"http://finance.yahoo.com/news/brazil-looks-break-us-centric-040621384.html\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 554 as libc::c_int as size_t,
            url: b"http://finance.yahoo.com/news/did-ted-cruz-just-force-211204954.html\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 642 as libc::c_int as size_t,
            url: b"http://finance.yahoo.com/news/theyve-turned-over-costa-concordia-102731647.html\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 284 as libc::c_int as size_t,
            url: b"http://finance.yahoo.com/q?s=^IXIC\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 578 as libc::c_int as size_t,
            url: b"http://finance.yahoo.com/video/winning-lottery-beat-odds-193258229.html\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 148 as libc::c_int as size_t,
            url: b"http://flickr.com\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 404 as libc::c_int as size_t,
            url: b"http://flss.edu.vn/Home.asp?param=news&NewsID=147\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 980 as libc::c_int as size_t,
            url: b"http://football.fantasysports.yahoo.com/?ovchn=YAH&ovcpn=Front+Page&ovcrn=Front+page+P+Link+Nav+button&ovrfd=YAH&ovtac=AD\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 340 as libc::c_int as size_t,
            url: b"http://fr.wikipedia.org/wiki/Caron_Butler\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 380 as libc::c_int as size_t,
            url: b"http://fr.wikipedia.org/wiki/Johann_David_Wyss\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 428 as libc::c_int as size_t,
            url: b"http://fr.wikipedia.org/wiki/Vy%C5%A1n%C3%BD_Slavkov\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 196 as libc::c_int as size_t,
            url: b"http://games.yahoo.com/\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 697 as libc::c_int as size_t,
            url: b"http://genome.ucsc.edu/cgi-bin/hgTracks?org=Human&db=hg19&position=chr22:24313554-24322660\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 689 as libc::c_int as size_t,
            url: b"http://genome.ucsc.edu/cgi-bin/hgTracks?org=Mouse&db=mm9&position=chr10:75771230-75773414\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 340 as libc::c_int as size_t,
            url: b"http://gl.wikipedia.org/wiki/Caron_Butler\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 196 as libc::c_int as size_t,
            url: b"http://green.yahoo.com/\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 204 as libc::c_int as size_t,
            url: b"http://groups.yahoo.com/\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 687 as libc::c_int as size_t,
            url: b"http://hangtime.blogs.nba.com/2011/01/04/caron-butler-out-for-season/?ls=iref:nbahpt2\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 476 as libc::c_int as size_t,
            url: b"http://hansard.millbanksystems.com/people/sir-massey-lopes\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 618 as libc::c_int as size_t,
            url: b"http://he.wikipedia.org/wiki/%D7%A7%D7%90%D7%A8%D7%95%D7%9F_%D7%91%D7%90%D7%98%D7%9C%D7%A8\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 196 as libc::c_int as size_t,
            url: b"http://health.yahoo.net\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 636 as libc::c_int as size_t,
            url: b"http://health.yahoo.net/articles/weight-loss/why-obese-people-cant-lose-weight\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 653 as libc::c_int as size_t,
            url: b"http://help.yahoo.com/kb/index?page=content&y=PROD_FRONT&locale=en_US&id=SLN14553\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 372 as libc::c_int as size_t,
            url: b"http://help.yahoo.com/l/us/yahoo/helpcentral/\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 268 as libc::c_int as size_t,
            url: b"http://homes.yahoo.com/own-rent/\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 340 as libc::c_int as size_t,
            url: b"http://hr.wikipedia.org/wiki/Caron_Butler\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 412 as libc::c_int as size_t,
            url: b"http://hu.wikipedia.org/wiki/Fels%C5%91szal%C3%B3k\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 180 as libc::c_int as size_t,
            url: b"http://info.yahoo.com\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 188 as libc::c_int as size_t,
            url: b"http://info.yahoo.com/\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 396 as libc::c_int as size_t,
            url: b"http://info.yahoo.com/legal/us/yahoo/utos/terms/\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 396 as libc::c_int as size_t,
            url: b"http://info.yahoo.com/privacy/us/yahoo/homepage/\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 452 as libc::c_int as size_t,
            url: b"http://info.yahoo.com/privacy/us/yahoo/relevantads.html\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 340 as libc::c_int as size_t,
            url: b"http://it.wikipedia.org/wiki/Caron_Butler\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 428 as libc::c_int as size_t,
            url: b"http://it.wikipedia.org/wiki/Vy%C5%A1n%C3%BD_Slavkov\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 651 as libc::c_int as size_t,
            url: b"http://ja.wikipedia.org/wiki/%E3%82%AB%E3%83%AD%E3%83%B3%E3%83%BB%E3%83%90%E3%83%88%E3%83%A9%E3%83%BC\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 898 as libc::c_int as size_t,
            url: b"http://ja.wikipedia.org/wiki/%E3%83%A8%E3%83%8F%E3%83%B3%E3%83%BB%E3%83%80%E3%83%93%E3%83%83%E3%83%88%E3%83%BB%E3%82%A6%E3%82%A3%E3%83%BC%E3%82%B9\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 564 as libc::c_int as size_t,
            url: b"http://jobsearch.monster.com/search/?cy=us&WT.mc_n=yta_trough_jsrtest\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 596 as libc::c_int as size_t,
            url: b"http://librivox.org/newcatalog/search.php?title=&author=Johann+David+Wyss\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 356 as libc::c_int as size_t,
            url: b"http://lv.wikipedia.org/wiki/Kerons_Batlers\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 252 as libc::c_int as size_t,
            url: b"http://mail.yahoo.com?.intl=us\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 188 as libc::c_int as size_t,
            url: b"http://maps.yahoo.com/\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 220 as libc::c_int as size_t,
            url: b"http://messenger.yahoo.com\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 196 as libc::c_int as size_t,
            url: b"http://movies.yahoo.com\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 522 as libc::c_int as size_t,
            url: b"http://movies.yahoo.com/video/short-game-crushing-224038083.html\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 428 as libc::c_int as size_t,
            url: b"http://ms.wikipedia.org/wiki/Vy%C5%A1n%C3%BD_Slavkov\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 188 as libc::c_int as size_t,
            url: b"http://music.yahoo.com\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 276 as libc::c_int as size_t,
            url: b"http://my.yahoo.com/?fr=yfp-t-403\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 634 as libc::c_int as size_t,
            url: b"http://myespn.go.com/blogs/truehoop/0-24-118/Caron-Butler-in-the-Basement.html\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 188 as libc::c_int as size_t,
            url: b"http://news.yahoo.com/\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 618 as libc::c_int as size_t,
            url: b"http://news.yahoo.com/amazing-roll-cloud-tumbles-over-dc-area-161409111.html\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 994 as libc::c_int as size_t,
            url: b"http://news.yahoo.com/blogs/trending-now/family-sells-home--goes-on-epic-yearlong-trip-to-all-50-states-185712913.html?vp=1\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 388 as libc::c_int as size_t,
            url: b"http://news.yahoo.com/comics/dilbert-slideshow/\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 746 as libc::c_int as size_t,
            url: b"http://news.yahoo.com/cost-cutting-may-have-played-role-in-navy-yard-shooting-134106229.html\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 682 as libc::c_int as size_t,
            url: b"http://news.yahoo.com/giant-underground-blob-magma-puzzles-scientists-210726200.html\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 746 as libc::c_int as size_t,
            url: b"http://news.yahoo.com/lightbox/dilbert-slideshow/20130523-dt130523-gif-photo-050423497.html\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 516 as libc::c_int as size_t,
            url: b"http://news.yahoo.com/photos/costa-concordia-salvage-slideshow/\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 599 as libc::c_int as size_t,
            url: b"http://news.yahoo.com/photos/turtle-rescue-in-israel-1379421810-slideshow/\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 666 as libc::c_int as size_t,
            url: b"http://news.yahoo.com/put-business-trauma-doctor-pleads-u-navy-yard-181522657.html\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 698 as libc::c_int as size_t,
            url: b"http://news.yahoo.com/touting-obamacare-us-says-millions-could-pay-less-212243340.html\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 578 as libc::c_int as size_t,
            url: b"http://news.yahoo.com/video/old-soda-bottles-giving-life-001700872.html\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 380 as libc::c_int as size_t,
            url: b"http://nl.wikipedia.org/wiki/Johann_David_Wyss\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 428 as libc::c_int as size_t,
            url: b"http://nl.wikipedia.org/wiki/Vy%C5%A1n%C3%BD_Slavkov\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 380 as libc::c_int as size_t,
            url: b"http://no.wikipedia.org/wiki/Johann_David_Wyss\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 180 as libc::c_int as size_t,
            url: b"http://omg.yahoo.com/\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 650 as libc::c_int as size_t,
            url: b"http://omg.yahoo.com/blogs/celeb-news/brad-pitt-finally-makes-cut-223549188.html\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 794 as libc::c_int as size_t,
            url: b"http://omg.yahoo.com/blogs/celeb-news/fashion-faceoff-miley-cyrus-vs-rachel-mcadams-190957403.html\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 882 as libc::c_int as size_t,
            url: b"http://omg.yahoo.com/blogs/celeb-news/justin-theroux-reveals-one-thing-jennifer-aniston-wouldn-113635271.html\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 882 as libc::c_int as size_t,
            url: b"http://omg.yahoo.com/news/courteney-cox-david-arquette-selling-beverly-hills-mansion-184500328-us-weekly.html\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 232 as libc::c_int as size_t,
            url: b"http://omim.org/entry/602750\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 3387 as libc::c_int as size_t,
            url: b"http://open.login.yahoo.net/openid/yrp/hr_signin?.intl=us&idp=facebook&ts=1379461518&rpcrumb=&.src=home&appid=90376669494&spid=b9ed13cc-ddec-11de-9c83-001b784d35e1&perms=email,user_birthday,user_education_history,user_likes,user_location,user_relationships,user_subscriptions,user_work_history,friends_birthday,friends_education_history,friends_likes,friends_location,friends_work_history&.done=http%3A%2F%2Fwww.yahoo.com%2F\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 320 as libc::c_int as size_t,
            url: b"http://openlibrary.org/authors/OL342629A\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 340 as libc::c_int as size_t,
            url: b"http://pl.wikipedia.org/wiki/Caron_Butler\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 380 as libc::c_int as size_t,
            url: b"http://pl.wikipedia.org/wiki/Johann_David_Wyss\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 428 as libc::c_int as size_t,
            url: b"http://pl.wikipedia.org/wiki/Vy%C5%A1n%C3%BD_Slavkov\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 855 as libc::c_int as size_t,
            url: b"http://probasketballtalk.nbcsports.com/2010/07/21/caron-butler-used-to-work-at-a-burger-king-now-owns-six/\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 340 as libc::c_int as size_t,
            url: b"http://pt.wikipedia.org/wiki/Caron_Butler\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 659 as libc::c_int as size_t,
            url: b"http://ru.wikipedia.org/wiki/%D0%91%D0%B0%D1%82%D0%BB%D0%B5%D1%80,_%D0%9A%D1%8D%D1%80%D0%BE%D0%BD\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 917 as libc::c_int as size_t,
            url: b"http://ru.wikipedia.org/wiki/%D0%9A%D0%BE%D0%BD%D1%81%D1%82%D0%B8%D1%82%D1%83%D1%86%D0%B8%D1%8F_%D0%A2%D1%83%D1%80%D1%86%D0%B8%D0%B8_(1921)\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 204 as libc::c_int as size_t,
            url: b"http://screen.yahoo.com/\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 453 as libc::c_int as size_t,
            url: b"http://screen.yahoo.com/11-old-boy-made-9-134626064.html\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 738 as libc::c_int as size_t,
            url: b"http://screen.yahoo.com/snl-women-skits/girl-wish-hadnt-started-conversation-000000658.html\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 204 as libc::c_int as size_t,
            url: b"http://search.yahoo.com/\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 244 as libc::c_int as size_t,
            url: b"http://search.yahoo.com/local\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 741 as libc::c_int as size_t,
            url: b"http://search.yahoo.com/search?cs=bz&p=Biblical-era%20town&fr=fp-tts-900&fr2=ps&woeid=23424856\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 765 as libc::c_int as size_t,
            url: b"http://search.yahoo.com/search?cs=bz&p=Green%20River%20Killer&fr=fp-tts-900&fr2=ps&woeid=23424856\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 733 as libc::c_int as size_t,
            url: b"http://search.yahoo.com/search?cs=bz&p=Hostile%20takeover&fr=fp-tts-900&fr2=ps&woeid=23424856\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 741 as libc::c_int as size_t,
            url: b"http://search.yahoo.com/search?cs=bz&p=Jennifer%20Lopez%20&fr=fp-tts-900&fr2=ps&woeid=23424856\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 701 as libc::c_int as size_t,
            url: b"http://search.yahoo.com/search?cs=bz&p=Jessa%20Duggar&fr=fp-tts-900&fr2=ps&woeid=23424856\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 717 as libc::c_int as size_t,
            url: b"http://search.yahoo.com/search?cs=bz&p=Kim%20Zolciak%20&fr=fp-tts-900&fr2=ps&woeid=23424856\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 717 as libc::c_int as size_t,
            url: b"http://search.yahoo.com/search?cs=bz&p=LSU%20fraternity&fr=fp-tts-900&fr2=ps&woeid=23424856\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 741 as libc::c_int as size_t,
            url: b"http://search.yahoo.com/search?cs=bz&p=Linda%20Ronstadt%20&fr=fp-tts-900&fr2=ps&woeid=23424856\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 749 as libc::c_int as size_t,
            url: b"http://search.yahoo.com/search?cs=bz&p=Star%20Trek%20NSA%20&fr=fp-tts-900&fr2=ps&woeid=23424856\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 781 as libc::c_int as size_t,
            url: b"http://search.yahoo.com/search?cs=bz&p=Walking%20Dead%20spinoff&fr=fp-tts-900&fr2=ps&woeid=23424856\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 188 as libc::c_int as size_t,
            url: b"http://shine.yahoo.com\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 196 as libc::c_int as size_t,
            url: b"http://shine.yahoo.com/\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 276 as libc::c_int as size_t,
            url: b"http://shine.yahoo.com/horoscope/\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 548 as libc::c_int as size_t,
            url: b"http://shine.yahoo.com/horoscope/virgo/extended-daily-20130917.html\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 548 as libc::c_int as size_t,
            url: b"http://shine.yahoo.com/horoscope/virgo/overview-daily-20130917.html\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 754 as libc::c_int as size_t,
            url: b"http://shine.yahoo.com/shine-food/5-things-didnt-know-could-pillsbury-biscuits-205500571.html\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 730 as libc::c_int as size_t,
            url: b"http://shine.yahoo.com/shine-food/fabios-ultimate-pulled-pork-sandwich-011700797.html?vp=1\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 220 as libc::c_int as size_t,
            url: b"http://shopping.yahoo.com/\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 412 as libc::c_int as size_t,
            url: b"http://simple.wikipedia.org/wiki/Johann_David_Wyss\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 428 as libc::c_int as size_t,
            url: b"http://sk.wikipedia.org/wiki/Vy%C5%A1n%C3%BD_Slavkov\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 436 as libc::c_int as size_t,
            url: b"http://smallbusiness.yahoo.com/?s_fptrough=ysb_acq_fp\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 402 as libc::c_int as size_t,
            url: b"http://sports.espn.go.com/nba/news/story?id=3290089\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 400 as libc::c_int as size_t,
            url: b"http://sports.espn.go.com/nba/recap?gameId=270117027\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 400 as libc::c_int as size_t,
            url: b"http://sports.espn.go.com/nba/recap?gameId=280313027\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 196 as libc::c_int as size_t,
            url: b"http://sports.yahoo.com\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 204 as libc::c_int as size_t,
            url: b"http://sports.yahoo.com/\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 911 as libc::c_int as size_t,
            url: b"http://sports.yahoo.com/blogs/mlb-big-league-stew/fan-dashes-onto-field-during-rays-game-only-192151790--mlb.html\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 991 as libc::c_int as size_t,
            url: b"http://sports.yahoo.com/blogs/nfl-shutdown-corner/does-andy-reid-still-wish-philadelphia-jamaal-charles-180705907--nfl.html\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 260 as libc::c_int as size_t,
            url: b"http://sports.yahoo.com/fantasy\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 228 as libc::c_int as size_t,
            url: b"http://sports.yahoo.com/mlb\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 376 as libc::c_int as size_t,
            url: b"http://sports.yahoo.com/mlb/preview?gid=330917104\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 360 as libc::c_int as size_t,
            url: b"http://sports.yahoo.com/mlb/recap?gid=330917220\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 308 as libc::c_int as size_t,
            url: b"http://sports.yahoo.com/mlb/teams/atl\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 939 as libc::c_int as size_t,
            url: b"http://sports.yahoo.com/nba/blog/ball_dont_lie/post/More-on-Caron-Butler-s-extreme-Mountain-Dew-addi?urn=nba%2C198240\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 332 as libc::c_int as size_t,
            url: b"http://sports.yahoo.com/nba/players/3608\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 647 as libc::c_int as size_t,
            url: b"http://sports.yahoo.com/news/team-report-san-francisco-49ers-211200051--nfl.html\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 228 as libc::c_int as size_t,
            url: b"http://sports.yahoo.com/nfl\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 383 as libc::c_int as size_t,
            url: b"http://sports.yahoo.com/nfl/preview?gid=20130919021\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 367 as libc::c_int as size_t,
            url: b"http://sports.yahoo.com/nfl/recap?gid=20130916004\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 308 as libc::c_int as size_t,
            url: b"http://sports.yahoo.com/nfl/teams/cin\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 228 as libc::c_int as size_t,
            url: b"http://sports.yahoo.com/nhl\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 308 as libc::c_int as size_t,
            url: b"http://sports.yahoo.com/nhl/teams/was\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 684 as libc::c_int as size_t,
            url: b"http://sr.wikipedia.org/wiki/%D0%92%D0%B8%D1%88%D0%BD%D0%B8_%D0%A1%D0%BB%D0%B0%D0%B2%D0%BA%D0%BE%D0%B2\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 832 as libc::c_int as size_t,
            url: b"http://ta.wikipedia.org/wiki/%E0%AE%95%E0%AE%B0%E0%AE%BE%E0%AE%A9%E0%AF%8D_%E0%AE%AA%E0%AE%9F%E0%AF%8D%E0%AE%B2%E0%AE%B0%E0%AF%8D\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 931 as libc::c_int as size_t,
            url: b"http://th.wikipedia.org/wiki/%E0%B9%81%E0%B8%84%E0%B8%A3%E0%B8%AD%E0%B8%99_%E0%B8%9A%E0%B8%B1%E0%B8%95%E0%B9%80%E0%B8%A5%E0%B8%AD%E0%B8%A3%E0%B9%8C\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 818 as libc::c_int as size_t,
            url: b"http://thptbinhxuyen.edu.vn/vp/Top-200-truong-THPT-co-diem-thi-dai-hoc-cao-nhat-2011-t11736-7938.html\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 972 as libc::c_int as size_t,
            url: b"http://tools.wmflabs.org/geohack/geohack.php?pagename=Vy%C5%A1n%C3%BD_Slavkov&params=49_04_N_20_52_E_region:SK_type:city\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 340 as libc::c_int as size_t,
            url: b"http://tr.wikipedia.org/wiki/Caron_Butler\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 604 as libc::c_int as size_t,
            url: b"http://tr.wikipedia.org/wiki/Te%C5%9Fkil%C3%A2t-%C4%B1_Esas%C3%AEye_Kanunu\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 204 as libc::c_int as size_t,
            url: b"http://travel.yahoo.com/\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 172 as libc::c_int as size_t,
            url: b"http://tv.yahoo.com/\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 770 as libc::c_int as size_t,
            url: b"http://tv.yahoo.com/news/ncis-exclusive-sopranos-alum-eyed-zivas-successor-debut-205821229.html\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 684 as libc::c_int as size_t,
            url: b"http://tv.yahoo.com/photos/emmys-unrecognizable-emmy-star-transformations-slideshow/\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 717 as libc::c_int as size_t,
            url: b"http://uk.wikipedia.org/wiki/%D0%92%D0%B8%D1%88%D0%BD%D1%96%D0%B9_%D0%A1%D0%BB%D0%B0%D0%B2%D0%BA%D0%BE%D0%B2\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 801 as libc::c_int as size_t,
            url: b"http://vi.wikipedia.org/wiki/Tr%C6%B0%E1%BB%9Dng_THPT_chuy%C3%AAn_L%C6%B0%C6%A1ng_V%C4%83n_Ch%C3%A1nh\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 1046 as libc::c_int as size_t,
            url: b"http://vi.wikipedia.org/wiki/X%E1%BA%BFp_h%E1%BA%A1ng_tr%C6%B0%E1%BB%9Dng_trung_h%E1%BB%8Dc_ph%E1%BB%95_th%C3%B4ng_Vi%E1%BB%87t_Nam\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 232 as libc::c_int as size_t,
            url: b"http://viaf.org/viaf/94287141\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 684 as libc::c_int as size_t,
            url: b"http://voices.washingtonpost.com/dcsportsbog/2007/04/on_caron_butler_and_straws.html\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 436 as libc::c_int as size_t,
            url: b"http://war.wikipedia.org/wiki/Vy%C5%A1n%C3%BD_Slavkov\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 505 as libc::c_int as size_t,
            url: b"http://washingtontimes.com/sports/20060417-122458-4991r_page2.htm\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 204 as libc::c_int as size_t,
            url: b"http://weather.yahoo.com\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 404 as libc::c_int as size_t,
            url: b"http://weather.yahoo.com/forecast/JAXX0077_f.html\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 740 as libc::c_int as size_t,
            url: b"http://web.archive.org/web/20040603025509/http://www.usabasketball.com/history/ymwc_2001.html\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 260 as libc::c_int as size_t,
            url: b"http://wikimediafoundation.org/\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 412 as libc::c_int as size_t,
            url: b"http://wikimediafoundation.org/wiki/Privacy_policy\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 396 as libc::c_int as size_t,
            url: b"http://wikimediafoundation.org/wiki/Terms_of_Use\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 308 as libc::c_int as size_t,
            url: b"http://www.anayasa.gen.tr/1921tek.htm\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 492 as libc::c_int as size_t,
            url: b"http://www.basketball-reference.com/players/b/butleca01.html\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 587 as libc::c_int as size_t,
            url: b"http://www.basketballreference.com/players/playerpage.htm?ilkid=BUTLECA01\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 380 as libc::c_int as size_t,
            url: b"http://www.bilkent.edu.tr/~genckaya/1921C.html\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 308 as libc::c_int as size_t,
            url: b"http://www.caronbutlersummercamp.com/\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 308 as libc::c_int as size_t,
            url: b"http://www.daao.org.au/main/read/2355\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 386 as libc::c_int as size_t,
            url: b"http://www.ebi.ac.uk/QuickGO/GProtein?ac=P30046\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 1416 as libc::c_int as size_t,
            url: b"http://www.ebi.ac.uk/pdbe/searchResults.html?display=both&term=P30046%20or%20Q53Y51%20or%20E2RQU0%20or%20A5PK65%20or%20O35215%20or%20Q3UNI8%20or%20P80254%20or%20Q5ZMG0%20or%20Q6IQL4\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 574 as libc::c_int as size_t,
            url: b"http://www.ensembl.org/Homo_sapiens/geneview?gene=ENSG00000099977;db=core\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 591 as libc::c_int as size_t,
            url: b"http://www.ensembl.org/Mus_musculus/geneview?gene=ENSMUSG00000001666;db=core\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 188 as libc::c_int as size_t,
            url: b"http://www.flickr.com/\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 550 as libc::c_int as size_t,
            url: b"http://www.flickr.com/photos/yahooeditorspicks/galleries/72157635598104765\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 901 as libc::c_int as size_t,
            url: b"http://www.flickr.com/photos/yahooeditorspicks/galleries/72157635598104765/with/7037921709/lightbox/?yc=www.yahoo.com\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 952 as libc::c_int as size_t,
            url: b"http://www.fool.com/investing/general/2013/09/16/3-car-brands-that-may-disappear-before-2020.aspx?source=eogyholnk0000001\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 576 as libc::c_int as size_t,
            url: b"http://www.genecards.org/cgi-bin/carddisp.pl?id_type=entrezgene&id=1652\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 456 as libc::c_int as size_t,
            url: b"http://www.genenames.org/data/hgnc_data.php?hgnc_id=2732\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 443 as libc::c_int as size_t,
            url: b"http://www.genome.jp/dbget-bin/www_bget?enzyme+4.1.1.84\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 404 as libc::c_int as size_t,
            url: b"http://www.gutenberg.org/author/Johann_David_Wyss\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 597 as libc::c_int as size_t,
            url: b"http://www.informatics.jax.org/searches/accession_report.cgi?id=MGI:1298381\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 212 as libc::c_int as size_t,
            url: b"http://www.intonow.com/ci\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 348 as libc::c_int as size_t,
            url: b"http://www.leighrayment.com/baronetage.htm\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 444 as libc::c_int as size_t,
            url: b"http://www.london-gazette.co.uk/issues/21964/pages/379\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 436 as libc::c_int as size_t,
            url: b"http://www.mapress.com/zootaxa/2010/f/z02583p119f.pdf\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 340 as libc::c_int as size_t,
            url: b"http://www.mapress.com/zootaxa/index.html\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 212 as libc::c_int as size_t,
            url: b"http://www.mediawiki.org/\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 544 as libc::c_int as size_t,
            url: b"http://www.nba.com/allstar2007/news/allstar_reserves_070201.html\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 332 as libc::c_int as size_t,
            url: b"http://www.nba.com/news/pow_070122.html\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 348 as libc::c_int as size_t,
            url: b"http://www.nba.com/playerfile/caron_butler\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 356 as libc::c_int as size_t,
            url: b"http://www.nba.com/playerfile/caron_butler/\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 492 as libc::c_int as size_t,
            url: b"http://www.nba.com/suns/news/suns-trade-kravtsov-smith-bucks\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 588 as libc::c_int as size_t,
            url: b"http://www.nba.com/suns/suns-complete-deal-eric-bledsoe-and-caron-butler\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 832 as libc::c_int as size_t,
            url: b"http://www.ncbi.nlm.nih.gov/entrez/query.fcgi?cmd=Retrieve&db=homologene&dopt=HomoloGene&list_uids=1038\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 812 as libc::c_int as size_t,
            url: b"http://www.ncbi.nlm.nih.gov/entrez/query.fcgi?db=gene&cmd=retrieve&dopt=default&list_uids=13202&rn=1\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 804 as libc::c_int as size_t,
            url: b"http://www.ncbi.nlm.nih.gov/entrez/query.fcgi?db=gene&cmd=retrieve&dopt=default&list_uids=1652&rn=1\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 488 as libc::c_int as size_t,
            url: b"http://www.ncbi.nlm.nih.gov/entrez/viewer.fcgi?val=NM_001084392\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 478 as libc::c_int as size_t,
            url: b"http://www.ncbi.nlm.nih.gov/entrez/viewer.fcgi?val=NM_010027\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 405 as libc::c_int as size_t,
            url: b"http://www.ncbi.nlm.nih.gov/pmc/articles/PMC1219731\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 400 as libc::c_int as size_t,
            url: b"http://www.ncbi.nlm.nih.gov/pmc/articles/PMC139241\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 405 as libc::c_int as size_t,
            url: b"http://www.ncbi.nlm.nih.gov/pmc/articles/PMC1847948\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 344 as libc::c_int as size_t,
            url: b"http://www.ncbi.nlm.nih.gov/pubmed/10079069\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 341 as libc::c_int as size_t,
            url: b"http://www.ncbi.nlm.nih.gov/pubmed/1286669\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 341 as libc::c_int as size_t,
            url: b"http://www.ncbi.nlm.nih.gov/pubmed/8267597\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 688 as libc::c_int as size_t,
            url: b"http://www.ncbi.nlm.nih.gov/sites/entrez?Db=gene&Cmd=ShowDetailView&TermToSearch=1652\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 747 as libc::c_int as size_t,
            url: b"http://www.ncbi.nlm.nih.gov/sites/entrez?db=gene&cmd=Link&LinkName=gene_pubmed&from_uid=13202\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 744 as libc::c_int as size_t,
            url: b"http://www.ncbi.nlm.nih.gov/sites/entrez?db=gene&cmd=Link&LinkName=gene_pubmed&from_uid=1652\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 428 as libc::c_int as size_t,
            url: b"http://www.oprah.com/oprahshow/Overcoming-the-Odds/8\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 428 as libc::c_int as size_t,
            url: b"http://www.oprah.com/oprahshow/Overcoming-the-Odds/9\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 412 as libc::c_int as size_t,
            url: b"http://www.rcsb.org/pdb/cgi/explore.cgi?pdbId=1DPT\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 1322 as libc::c_int as size_t,
            url: b"http://www.rcsb.org/pdb/search/smartSubquery.do?smartSearchSubtype=UpAccessionIdQuery&accessionIdList=P30046,Q53Y51,E2RQU0,A5PK65,O35215,Q3UNI8,P80254,Q5ZMG0,Q6IQL4\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 806 as libc::c_int as size_t,
            url: b"http://www.sbnation.com/golf/2013/9/17/4737628/bmw-championship-2013-tiger-woods-penalty-johnny-miller\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 364 as libc::c_int as size_t,
            url: b"http://www.statistics.sk/mosmis/eng/run.html\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 188 as libc::c_int as size_t,
            url: b"http://www.tbmm.gov.tr\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 372 as libc::c_int as size_t,
            url: b"http://www.tbmm.gov.tr/english/about_tgna.htm\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 212 as libc::c_int as size_t,
            url: b"http://www.thepeerage.com\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 284 as libc::c_int as size_t,
            url: b"http://www.thepeerage.com/info.htm\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 585 as libc::c_int as size_t,
            url: b"http://www.uconnhuskies.com/AllStories/MBasketball/2002/06/26/20020626.html\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 304 as libc::c_int as size_t,
            url: b"http://www.uniprot.org/uniprot/O35215\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 304 as libc::c_int as size_t,
            url: b"http://www.uniprot.org/uniprot/P30046\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 644 as libc::c_int as size_t,
            url: b"http://www.washingtonpost.com/wp-dyn/content/article/2008/02/16/AR2008021600752_2.html\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 293 as libc::c_int as size_t,
            url: b"http://www.wikidata.org/wiki/Q115822\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 460 as libc::c_int as size_t,
            url: b"http://www.wikidata.org/wiki/Q115822#sitelinks-wikipedia\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 292 as libc::c_int as size_t,
            url: b"http://www.wikimediafoundation.org/\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 172 as libc::c_int as size_t,
            url: b"http://www.yahoo.com\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 180 as libc::c_int as size_t,
            url: b"http://www.yahoo.com/\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 188 as libc::c_int as size_t,
            url: b"http://www.yahoo.com/#\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 276 as libc::c_int as size_t,
            url: b"http://www.yahoo.com/#suggestions\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 244 as libc::c_int as size_t,
            url: b"http://www.yahoo.com/?hps=210\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 444 as libc::c_int as size_t,
            url: b"http://yahoo.match.com?trackingid=526100&bannerid=673168\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 535 as libc::c_int as size_t,
            url: b"http://zh.wikipedia.org/wiki/%E5%8D%A1%E9%9A%86%C2%B7%E5%B7%B4%E7%89%B9%E5%8B%92\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 1164 as libc::c_int as size_t,
            url: b"https://donate.wikimedia.org/wiki/Special:FundraiserRedirector?utm_source=donate&utm_medium=sidebar&utm_campaign=C13_en.wikipedia.org&uselang=en\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 740 as libc::c_int as size_t,
            url: b"https://edit.yahoo.com/registration?.src=fpctx&.intl=us&.done=http%3A%2F%2Fwww.yahoo.com%2F\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 748 as libc::c_int as size_t,
            url: b"https://login.yahoo.com/config/login?.src=fpctx&.intl=us&.done=http%3A%2F%2Fwww.yahoo.com%2F\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 548 as libc::c_int as size_t,
            url: b"https://www.mediawiki.org/wiki/Special:MyLanguage/How_to_contribute\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = TestSet {
            expected_length: 0 as libc::c_int as size_t,
            url: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
];
unsafe extern "C" fn encodeURLandCompare(
    mut url: *mut libc::c_char,
    mut expected_length: size_t,
) -> libc::c_int {
    let mut input: *mut QRinput = 0 as *mut QRinput;
    let mut bstream: *mut BitStream = 0 as *mut BitStream;
    let mut ret: libc::c_int = 0 as libc::c_int;
    input = QRinput_new2(0 as libc::c_int, QR_ECLEVEL_L);
    Split_splitStringToQRinput(url, input, QR_MODE_8, 1 as libc::c_int);
    bstream = BitStream_new();
    QRinput_mergeBitStream(input, bstream);
    let mut length: size_t = (*bstream).length;
    if length > expected_length {
        printf(
            b"The length of the encode stream is longer than expected: %zu over %zu\n\0"
                as *const u8 as *const libc::c_char,
            length,
            expected_length,
        );
        printQRinput(input);
        ret = 1 as libc::c_int;
    } else if length < expected_length {
        printf(
            b"The length of the encode stream is shorter than expected: %zu under %zu\n\0"
                as *const u8 as *const libc::c_char,
            length,
            expected_length,
        );
        printQRinput(input);
        ret = 1 as libc::c_int;
    }
    QRinput_free(input);
    BitStream_free(bstream);
    return ret;
}
unsafe extern "C" fn test_bitstream_length() {
    let mut ts: *mut TestSet = testset.as_mut_ptr();
    let mut err: libc::c_int = 0 as libc::c_int;
    testStartReal(
        (*::std::mem::transmute::<
            &[u8; 22],
            &[libc::c_char; 22],
        >(b"test_bitstream_length\0"))
            .as_ptr(),
        b"Split_URL test: compare bitstream length\0" as *const u8 as *const libc::c_char,
    );
    while !((*ts).url).is_null() {
        err += encodeURLandCompare((*ts).url, (*ts).expected_length);
        ts = ts.offset(1);
        ts;
    }
    testEnd(err);
}
unsafe fn main_0() -> libc::c_int {
    let mut tests: libc::c_int = 1 as libc::c_int;
    testInit(tests);
    test_bitstream_length();
    testReport(tests);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
