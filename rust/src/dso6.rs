extern {
    fn bug_(arg1 : i32, arg2 : i32);
    static mut findex_ : Struct5;
    static rindex_ : Struct3;
    static mut rooms_ : Struct4;
    static mut star_ : Struct2;
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct2 {
    pub mbase : i32,
    pub strbit : i32,
}

impl Clone for Struct2 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct3 {
    pub whous : i32,
    pub lroom : i32,
    pub cella : i32,
    pub mtrol : i32,
    pub maze1 : i32,
    pub mgrat : i32,
    pub maz15 : i32,
    pub fore1 : i32,
    pub fore3 : i32,
    pub clear : i32,
    pub reser : i32,
    pub strea : i32,
    pub egypt : i32,
    pub echor : i32,
    pub tshaf : i32,
    pub bshaf : i32,
    pub mmach : i32,
    pub dome : i32,
    pub mtorc : i32,
    pub carou : i32,
    pub riddl : i32,
    pub lld2 : i32,
    pub temp1 : i32,
    pub temp2 : i32,
    pub maint : i32,
    pub blroo : i32,
    pub treas : i32,
    pub rivr1 : i32,
    pub rivr2 : i32,
    pub rivr3 : i32,
    pub mcycl : i32,
    pub rivr4 : i32,
    pub rivr5 : i32,
    pub fchmp : i32,
    pub falls : i32,
    pub mbarr : i32,
    pub mrain : i32,
    pub pog : i32,
    pub vlbot : i32,
    pub vair1 : i32,
    pub vair2 : i32,
    pub vair3 : i32,
    pub vair4 : i32,
    pub ledg2 : i32,
    pub ledg3 : i32,
    pub ledg4 : i32,
    pub msafe : i32,
    pub cager : i32,
    pub caged : i32,
    pub twell : i32,
    pub bwell : i32,
    pub alice : i32,
    pub alism : i32,
    pub alitr : i32,
    pub mtree : i32,
    pub bkent : i32,
    pub bkvw : i32,
    pub bktwi : i32,
    pub bkvau : i32,
    pub bkbox : i32,
    pub crypt : i32,
    pub tstrs : i32,
    pub mrant : i32,
    pub mreye : i32,
    pub mra : i32,
    pub mrb : i32,
    pub mrc : i32,
    pub mrg : i32,
    pub mrd : i32,
    pub fdoor : i32,
    pub mrae : i32,
    pub mrce : i32,
    pub mrcw : i32,
    pub mrge : i32,
    pub mrgw : i32,
    pub mrdw : i32,
    pub inmir : i32,
    pub scorr : i32,
    pub ncorr : i32,
    pub parap : i32,
    pub cell : i32,
    pub pcell : i32,
    pub ncell : i32,
    pub cpant : i32,
    pub cpout : i32,
    pub cpuzz : i32,
}

impl Clone for Struct3 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct4 {
    pub rlnt : i32,
    pub rdesc1 : [i32; 200],
    pub rdesc2 : [i32; 200],
    pub rexit : [i32; 200],
    pub ractio : [i32; 200],
    pub rval : [i32; 200],
    pub rflag : [i32; 200],
}

impl Clone for Struct4 {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub unsafe extern fn ghere_(mut obj : i32, mut rm : i32) -> i32 {
    let mut ret_val : i32;
    ret_val = 1i32;
    let switch1 = obj - star_.strbit;
    if switch1 == 25i32 {
        ret_val = (rm == rindex_.cpuzz) as (i32);
        ret_val
    } else if switch1 == 24i32 {
        ret_val = (rm == rindex_.fdoor || rm == rindex_.ncorr || rm == rindex_.parap || rm == rindex_.cell) as (i32);
        ret_val
    } else if switch1 == 23i32 || switch1 == 20i32 {
        ret_val = (rm >= rindex_.mra && (rm <= rindex_.mrd) || rm == rindex_.inmir) as (i32);
        ret_val
    } else {
        if switch1 == 22i32 {
            if rm == rindex_.fdoor {
                return ret_val;
            }
        } else if !(switch1 == 21i32) {
            if switch1 == 19i32 {
                ret_val = (rm >= rindex_.mrc && (rm <= rindex_.mrd) || rm >= rindex_.mrce && (rm <= rindex_.mrdw) || rm == rindex_.inmir) as (i32);
                return ret_val;
            } else if switch1 == 18i32 {
                ret_val = (rooms_.rflag[
                               (rm - 1i32) as (usize)
                           ] & 4096i32 + 512i32 != 0i32) as (i32);
                return ret_val;
            } else if switch1 == 17i32 || switch1 == 16i32 || switch1 == 15i32 {
                ret_val = (rm >= rindex_.bkvw && (rm < rindex_.bkbox) || rm == rindex_.cpuzz) as (i32);
                return ret_val;
            } else if switch1 == 14i32 {
                ret_val = (rm >= rindex_.bkvw && (rm <= rindex_.bkbox) || rm == rindex_.cpuzz) as (i32);
                return ret_val;
            } else if switch1 == 13i32 {
                ret_val = (rm >= rindex_.fore1 && (rm < rindex_.clear) && (rm != rindex_.fore3)) as (i32);
                return ret_val;
            } else if switch1 == 12i32 {
                ret_val = (rm >= rindex_.fore1 && (rm < rindex_.clear) || rm == rindex_.mtree) as (i32);
                return ret_val;
            } else {
                if !(switch1 == 11i32 || switch1 == 10i32 || switch1 == 9i32 || switch1 == 8i32 || switch1 == 7i32 || switch1 == 6i32 || switch1 == 5i32 || switch1 == 4i32 || switch1 == 3i32 || switch1 == 2i32 || switch1 == 1i32) {
                    bug_(60i32,obj);
                }
                return ret_val;
            }
        }
        ret_val = (rm >= rindex_.mra && (rm <= rindex_.mrc) || rm >= rindex_.mrae && (rm <= rindex_.mrcw)) as (i32);
        ret_val
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct5 {
    pub trollf : i32,
    pub cagesf : i32,
    pub bucktf : i32,
    pub caroff : i32,
    pub carozf : i32,
    pub lwtidf : i32,
    pub domef : i32,
    pub glacrf : i32,
    pub echof : i32,
    pub riddlf : i32,
    pub lldf : i32,
    pub cyclof : i32,
    pub magicf : i32,
    pub litldf : i32,
    pub safef : i32,
    pub gnomef : i32,
    pub gnodrf : i32,
    pub mirrmf : i32,
    pub egyptf : i32,
    pub onpolf : i32,
    pub blabf : i32,
    pub brieff : i32,
    pub superf : i32,
    pub buoyf : i32,
    pub grunlf : i32,
    pub gatef : i32,
    pub rainbf : i32,
    pub cagetf : i32,
    pub empthf : i32,
    pub deflaf : i32,
    pub glacmf : i32,
    pub frobzf : i32,
    pub endgmf : i32,
    pub badlkf : i32,
    pub thfenf : i32,
    pub singsf : i32,
    pub mrpshf : i32,
    pub mropnf : i32,
    pub wdopnf : i32,
    pub mr1f : i32,
    pub mr2f : i32,
    pub inqstf : i32,
    pub follwf : i32,
    pub spellf : i32,
    pub cpoutf : i32,
    pub cpushf : i32,
    pub btief : i32,
    pub binff : i32,
    pub rvmnt : i32,
    pub rvclr : i32,
    pub rvcyc : i32,
    pub rvsnd : i32,
    pub rvgua : i32,
    pub orrug : i32,
    pub orcand : i32,
    pub ormtch : i32,
    pub orlamp : i32,
    pub mdir : i32,
    pub mloc : i32,
    pub poleuf : i32,
    pub quesno : i32,
    pub nqatt : i32,
    pub corrct : i32,
    pub lcell : i32,
    pub pnumb : i32,
    pub acell : i32,
    pub dcell : i32,
    pub cphere : i32,
}

impl Clone for Struct5 {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub unsafe extern fn mrhere_(mut rm : i32) -> i32 {
    let mut ret_val : i32;
    let mut i__1 : i32;
    if rm < rindex_.mrae || rm > rindex_.mrdw {
        ret_val = 0i32;
        (if {
                i__1 = findex_.mloc - rm;
                if i__1 >= 0i32 { i__1 } else { -i__1 }
            } != 1i32 || findex_.mdir % 180i32 == 0i32 {
             ret_val
         } else {
             ret_val = 1i32;
             if rm < findex_.mloc && (findex_.mdir < 180i32) || rm > findex_.mloc && (findex_.mdir > 180i32) {
                 ret_val = 2i32;
             }
             ret_val
         })
    } else {
        ret_val = 1i32;
        if (rm - rindex_.mrae) % 2i32 == findex_.mdir / 180i32 {
            ret_val = 2i32;
        }
        ret_val
    }
}
