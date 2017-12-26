extern {
    static mut advs_ : Struct6;
    fn itime_(arg1 : *mut i32, arg2 : *mut i32, arg3 : *mut i32);
    static mut objcts_ : Struct4;
    static mut prsvec_ : Struct2;
    fn qhere_(arg1 : i32, arg2 : i32) -> i32;
    fn rnd_(arg1 : i32) -> i32;
    static mut rooms_ : Struct5;
    fn rspeak_(arg1 : i32);
    static mut time_ : Struct1;
    static vindex_ : Struct3;
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct1 {
    pub pltime : i32,
    pub shour : i32,
    pub smin : i32,
    pub ssec : i32,
}

impl Clone for Struct1 {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub unsafe extern fn gttime_(mut t : *mut i32) {
    let mut h : i32;
    let mut m : i32;
    let mut s : i32;
    itime_(
        &mut h as (*mut i32),
        &mut m as (*mut i32),
        &mut s as (*mut i32)
    );
    *t = h * 60i32 + m - (time_.shour * 60i32 + time_.smin);
    if *t < 0i32 {
        *t = *t + 1440i32;
    }
    *t = *t + time_.pltime;
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct2 {
    pub prsa : i32,
    pub prsi : i32,
    pub prso : i32,
    pub prswon : i32,
    pub prscon : i32,
}

impl Clone for Struct2 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct3 {
    pub cintw : i32,
    pub deadxw : i32,
    pub frstqw : i32,
    pub inxw : i32,
    pub outxw : i32,
    pub walkiw : i32,
    pub fightw : i32,
    pub foow : i32,
    pub meltw : i32,
    pub readw : i32,
    pub inflaw : i32,
    pub deflaw : i32,
    pub alarmw : i32,
    pub exorcw : i32,
    pub plugw : i32,
    pub kickw : i32,
    pub wavew : i32,
    pub raisew : i32,
    pub lowerw : i32,
    pub rubw : i32,
    pub pushw : i32,
    pub untiew : i32,
    pub tiew : i32,
    pub tieupw : i32,
    pub turnw : i32,
    pub breatw : i32,
    pub knockw : i32,
    pub lookw : i32,
    pub examiw : i32,
    pub shakew : i32,
    pub movew : i32,
    pub trnonw : i32,
    pub trnofw : i32,
    pub openw : i32,
    pub closew : i32,
    pub findw : i32,
    pub waitw : i32,
    pub spinw : i32,
    pub boardw : i32,
    pub unboaw : i32,
    pub takew : i32,
    pub invenw : i32,
    pub fillw : i32,
    pub eatw : i32,
    pub drinkw : i32,
    pub burnw : i32,
    pub mungw : i32,
    pub killw : i32,
    pub attacw : i32,
    pub swingw : i32,
    pub walkw : i32,
    pub tellw : i32,
    pub putw : i32,
    pub dropw : i32,
    pub givew : i32,
    pub pourw : i32,
    pub throww : i32,
    pub digw : i32,
    pub leapw : i32,
    pub stayw : i32,
    pub follow : i32,
    pub hellow : i32,
    pub lookiw : i32,
    pub lookuw : i32,
    pub pumpw : i32,
    pub windw : i32,
    pub clmbw : i32,
    pub clmbuw : i32,
    pub clmbdw : i32,
    pub trntow : i32,
}

impl Clone for Struct3 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct4 {
    pub olnt : i32,
    pub odesc1 : [i32; 220],
    pub odesc2 : [i32; 220],
    pub odesco : [i32; 220],
    pub oactio : [i32; 220],
    pub oflag1 : [i32; 220],
    pub oflag2 : [i32; 220],
    pub ofval : [i32; 220],
    pub otval : [i32; 220],
    pub osize : [i32; 220],
    pub ocapac : [i32; 220],
    pub oroom : [i32; 220],
    pub oadv : [i32; 220],
    pub ocan : [i32; 220],
    pub oread : [i32; 220],
}

impl Clone for Struct4 {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub unsafe extern fn opncls_(
    mut obj : i32, mut so : i32, mut sc : i32
) -> i32 {
    let mut i__1 : i32;
    let mut ret_val : i32;
    ret_val = 1i32;
    if prsvec_.prsa == vindex_.closew {
        if !!(objcts_.oflag2[(obj - 1i32) as (usize)] & 8i32 != 0i32) {
            rspeak_(sc);
            let _rhs = !8i32;
            let _lhs = &mut objcts_.oflag2[(obj - 1i32) as (usize)];
            *_lhs = *_lhs & _rhs;
            return ret_val;
        }
    } else if prsvec_.prsa == vindex_.openw {
        if !(objcts_.oflag2[(obj - 1i32) as (usize)] & 8i32 != 0i32) {
            rspeak_(so);
            let _rhs = 8i32;
            let _lhs = &mut objcts_.oflag2[(obj - 1i32) as (usize)];
            *_lhs = *_lhs | _rhs;
            return ret_val;
        }
    } else {
        ret_val = 0i32;
        return ret_val;
    }
    i__1 = rnd_(3i32) + 125i32;
    rspeak_(i__1);
    ret_val
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct5 {
    pub rlnt : i32,
    pub rdesc1 : [i32; 200],
    pub rdesc2 : [i32; 200],
    pub rexit : [i32; 200],
    pub ractio : [i32; 200],
    pub rval : [i32; 200],
    pub rflag : [i32; 200],
}

impl Clone for Struct5 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct6 {
    pub alnt : i32,
    pub aroom : [i32; 4],
    pub ascore : [i32; 4],
    pub avehic : [i32; 4],
    pub aobj : [i32; 4],
    pub aactio : [i32; 4],
    pub astren : [i32; 4],
    pub aflag : [i32; 4],
}

impl Clone for Struct6 {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub unsafe extern fn lit_(mut rm : i32) -> i32 {
    let mut _currentBlock;
    let mut i__1 : i32;
    let mut i__2 : i32;
    let mut ret_val : i32;
    let mut i : i32;
    let mut j : i32;
    let mut oa : i32;
    ret_val = 1i32;
    if rooms_.rflag[(rm - 1i32) as (usize)] & 16384i32 != 0i32 {
        ret_val
    } else {
        i__1 = objcts_.olnt;
        i = 1i32;
        'loop2: loop {
            if !(i <= i__1) {
                _currentBlock = 3;
                break;
            }
            if qhere_(i,rm) != 0 {
                _currentBlock = 7;
            } else {
                oa = objcts_.oadv[(i - 1i32) as (usize)];
                if oa <= 0i32 {
                    _currentBlock = 14;
                } else if advs_.aroom[(oa - 1i32) as (usize)] != rm {
                    _currentBlock = 14;
                } else {
                    _currentBlock = 7;
                }
            }
            if _currentBlock == 7 {
                if objcts_.oflag1[(i - 1i32) as (usize)] & 1i32 != 0i32 {
                    _currentBlock = 15;
                    break;
                }
                if !(objcts_.oflag1[
                         (i - 1i32) as (usize)
                     ] & 32768i32 == 0i32 || objcts_.oflag1[
                                                 (i - 1i32) as (usize)
                                             ] & 2048i32 == 0i32 && (objcts_.oflag2[
                                                                         (i - 1i32) as (usize)
                                                                     ] & 8i32 == 0i32)) {
                    i__2 = objcts_.olnt;
                    j = 1i32;
                    'loop10: loop {
                        if !(j <= i__2) {
                            break;
                        }
                        if objcts_.ocan[(j - 1i32) as (usize)] == i && (objcts_.oflag1[
                                                                            (j - 1i32) as (usize)
                                                                        ] & 1i32 != 0i32) {
                            _currentBlock = 13;
                            break 'loop2;
                        }
                        j = j + 1;
                    }
                }
            }
            i = i + 1;
        }
        (if _currentBlock == 3 {
             ret_val = 0i32;
             ret_val
         } else if _currentBlock == 13 {
             ret_val
         } else {
             ret_val
         })
    }
}

#[no_mangle]
pub unsafe extern fn weight_(
    mut rm : i32, mut cn : i32, mut ad : i32
) -> i32 {
    let mut _currentBlock;
    let mut ret_val : i32;
    let mut i__1 : i32;
    let mut i : i32;
    let mut j : i32;
    ret_val = 0i32;
    i__1 = objcts_.olnt;
    i = 1i32;
    'loop1: loop {
        if !(i <= i__1) {
            break;
        }
        if !(objcts_.osize[(i - 1i32) as (usize)] >= 10000i32) {
            if qhere_(i,rm) != 0 && (rm != 0i32) || objcts_.oadv[
                                                        (i - 1i32) as (usize)
                                                    ] == ad && (ad != 0i32) {
                _currentBlock = 8;
            } else {
                j = i;
                'loop6: loop {
                    j = objcts_.ocan[(j - 1i32) as (usize)];
                    if j == 0i32 {
                        _currentBlock = 9;
                        break;
                    }
                    if !(j != cn) {
                        _currentBlock = 8;
                        break;
                    }
                }
            }
            if _currentBlock == 9 {
            } else {
                ret_val = ret_val + objcts_.osize[(i - 1i32) as (usize)];
            }
        }
        i = i + 1;
    }
    ret_val
}
