extern {
    static mut advs_ : Struct3;
    static aindex_ : Struct17;
    fn bug_(arg1 : i32, arg2 : i32);
    static mut cevent_ : Struct9;
    static cindex_ : Struct10;
    fn cpgoto_(arg1 : i32);
    fn cpinfo_(arg1 : i32, arg2 : i32);
    static mut findex_ : Struct12;
    static hyper_ : Struct16;
    fn moveto_(arg1 : i32, arg2 : i32) -> i32;
    fn mrhere_(arg1 : i32) -> i32;
    fn newsta_(
        arg1 : i32, arg2 : i32, arg3 : i32, arg4 : i32, arg5 : i32
    );
    static mut objcts_ : Struct2;
    static oindex_ : Struct7;
    fn opncls_(arg1 : i32, arg2 : i32, arg3 : i32) -> i32;
    static mut play_ : Struct4;
    fn princr_(arg1 : i32, arg2 : i32);
    static mut prsvec_ : Struct1;
    static mut puzzle_ : Struct14;
    fn qhere_(arg1 : i32, arg2 : i32) -> i32;
    static rindex_ : Struct8;
    fn rnd_(arg1 : i32) -> i32;
    static mut rooms_ : Struct13;
    fn rspeak_(arg1 : i32);
    fn rspsub_(arg1 : i32, arg2 : i32);
    static mut screen_ : Struct11;
    static vindex_ : Struct6;
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct1 {
    pub prsa : i32,
    pub prsi : i32,
    pub prso : i32,
    pub prswon : i32,
    pub prscon : i32,
}

impl Clone for Struct1 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct2 {
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

impl Clone for Struct2 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct3 {
    pub alnt : i32,
    pub aroom : [i32; 4],
    pub ascore : [i32; 4],
    pub avehic : [i32; 4],
    pub aobj : [i32; 4],
    pub aactio : [i32; 4],
    pub astren : [i32; 4],
    pub aflag : [i32; 4],
}

impl Clone for Struct3 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct4 {
    pub winner : i32,
    pub here : i32,
    pub telflg : i32,
}

impl Clone for Struct4 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct6 {
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

impl Clone for Struct6 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct7 {
    pub garli : i32,
    pub food : i32,
    pub gunk : i32,
    pub coal : i32,
    pub machi : i32,
    pub diamo : i32,
    pub tcase : i32,
    pub bottl : i32,
    pub water : i32,
    pub rope : i32,
    pub knife : i32,
    pub sword : i32,
    pub lamp : i32,
    pub blamp : i32,
    pub rug : i32,
    pub leave : i32,
    pub troll : i32,
    pub axe : i32,
    pub rknif : i32,
    pub keys : i32,
    pub ice : i32,
    pub bar : i32,
    pub coffi : i32,
    pub torch : i32,
    pub tbask : i32,
    pub fbask : i32,
    pub irbox : i32,
    pub ghost : i32,
    pub trunk : i32,
    pub bell : i32,
    pub book : i32,
    pub candl : i32,
    pub match_ : i32,
    pub tube : i32,
    pub putty : i32,
    pub wrenc : i32,
    pub screw : i32,
    pub cyclo : i32,
    pub chali : i32,
    pub thief : i32,
    pub still : i32,
    pub windo : i32,
    pub grate : i32,
    pub door : i32,
    pub hpole : i32,
    pub leak : i32,
    pub rbutt : i32,
    pub raili : i32,
    pub pot : i32,
    pub statu : i32,
    pub iboat : i32,
    pub dboat : i32,
    pub pump : i32,
    pub rboat : i32,
    pub stick : i32,
    pub buoy : i32,
    pub shove : i32,
    pub ballo : i32,
    pub recep : i32,
    pub guano : i32,
    pub brope : i32,
    pub hook1 : i32,
    pub hook2 : i32,
    pub safe : i32,
    pub sslot : i32,
    pub brick : i32,
    pub fuse : i32,
    pub gnome : i32,
    pub blabe : i32,
    pub dball : i32,
    pub tomb : i32,
    pub lcase : i32,
    pub cage : i32,
    pub rcage : i32,
    pub spher : i32,
    pub sqbut : i32,
    pub flask : i32,
    pub pool : i32,
    pub saffr : i32,
    pub bucke : i32,
    pub ecake : i32,
    pub orice : i32,
    pub rdice : i32,
    pub blice : i32,
    pub robot : i32,
    pub ftree : i32,
    pub bills : i32,
    pub portr : i32,
    pub scol : i32,
    pub zgnom : i32,
    pub egg : i32,
    pub begg : i32,
    pub baubl : i32,
    pub canar : i32,
    pub bcana : i32,
    pub ylwal : i32,
    pub rdwal : i32,
    pub pindr : i32,
    pub rbeam : i32,
    pub odoor : i32,
    pub qdoor : i32,
    pub cdoor : i32,
    pub num1 : i32,
    pub num8 : i32,
    pub warni : i32,
    pub cslit : i32,
    pub gcard : i32,
    pub stldr : i32,
    pub hands : i32,
    pub wall : i32,
    pub lungs : i32,
    pub sailo : i32,
    pub aviat : i32,
    pub teeth : i32,
    pub itobj : i32,
    pub every : i32,
    pub valua : i32,
    pub oplay : i32,
    pub wnort : i32,
    pub gwate : i32,
    pub master : i32,
}

impl Clone for Struct7 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct8 {
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

impl Clone for Struct8 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct9 {
    pub clnt : i32,
    pub ctick : [i32; 25],
    pub cactio : [i32; 25],
    pub cflag : [i32; 25],
}

impl Clone for Struct9 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct10 {
    pub cevcur : i32,
    pub cevmnt : i32,
    pub cevlnt : i32,
    pub cevmat : i32,
    pub cevcnd : i32,
    pub cevbal : i32,
    pub cevbrn : i32,
    pub cevfus : i32,
    pub cevled : i32,
    pub cevsaf : i32,
    pub cevvlg : i32,
    pub cevgno : i32,
    pub cevbuc : i32,
    pub cevsph : i32,
    pub cevegh : i32,
    pub cevfor : i32,
    pub cevscl : i32,
    pub cevzgi : i32,
    pub cevzgo : i32,
    pub cevste : i32,
    pub cevmrs : i32,
    pub cevpin : i32,
    pub cevinq : i32,
    pub cevfol : i32,
}

impl Clone for Struct10 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct11 {
    pub fromdr : i32,
    pub scolrm : i32,
    pub scolac : i32,
    pub scoldr : [i32; 8],
    pub scolwl : [i32; 12],
}

impl Clone for Struct11 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct12 {
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

impl Clone for Struct12 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct13 {
    pub rlnt : i32,
    pub rdesc1 : [i32; 200],
    pub rdesc2 : [i32; 200],
    pub rexit : [i32; 200],
    pub ractio : [i32; 200],
    pub rval : [i32; 200],
    pub rflag : [i32; 200],
}

impl Clone for Struct13 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct14 {
    pub cpdr : [i32; 16],
    pub cpwl : [i32; 8],
    pub cpvec : [i32; 64],
}

impl Clone for Struct14 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct16 {
    pub hfactr : i32,
}

impl Clone for Struct16 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct17 {
    pub player : i32,
    pub arobot : i32,
    pub amastr : i32,
}

impl Clone for Struct17 {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub unsafe extern fn nobjs_(mut ri : i32, mut arg : i32) -> i32 {
    let mut _currentBlock;
    let mut i__1 : i32;
    let mut i__2 : i32;
    let mut ret_val : i32;
    let mut f : i32;
    let mut target : i32 = 0;
    let mut i : i32 = 0;
    let mut j : i32;
    let mut av : i32;
    let mut wl : i32;
    let mut nxt : i32;
    let mut odi2 : i32 = 0i32;
    let mut odo2 : i32 = 0i32;
    if prsvec_.prso != 0i32 {
        odo2 = objcts_.odesc2[(prsvec_.prso - 1i32) as (usize)];
    }
    if prsvec_.prsi != 0i32 {
        odi2 = objcts_.odesc2[(prsvec_.prsi - 1i32) as (usize)];
    }
    av = advs_.avehic[(play_.winner - 1i32) as (usize)];
    ret_val = 1i32;
    let switch5 = ri - 31i32;
    if switch5 == 21i32 {
        if !(prsvec_.prsa != vindex_.putw || prsvec_.prsi != oindex_.cslit) {
            if prsvec_.prso != oindex_.gcard {
                if objcts_.oflag1[
                       (prsvec_.prso - 1i32) as (usize)
                   ] & 32i32 == 0i32 && (objcts_.oflag2[
                                             (prsvec_.prso - 1i32) as (usize)
                                         ] & 128i32 == 0i32) {
                    newsta_(prsvec_.prso,0i32,0i32,0i32,0i32);
                    rspsub_(864i32,odo2);
                    return ret_val;
                } else {
                    i__1 = rnd_(5i32) + 552i32;
                    rspeak_(i__1);
                    return ret_val;
                }
            } else {
                newsta_(prsvec_.prso,863i32,0i32,0i32,0i32);
                findex_.cpoutf = 1i32;
                let _rhs = !32768i32;
                let _lhs = &mut objcts_.oflag1[(oindex_.stldr - 1i32) as (usize)];
                *_lhs = *_lhs & _rhs;
                return ret_val;
            }
        }
    } else if switch5 == 20i32 {
        if play_.here != rindex_.fdoor {
            ret_val = mirpan_(838i32,1i32);
            return ret_val;
        } else if !(prsvec_.prsa != vindex_.openw && (prsvec_.prsa != vindex_.closew)) {
            rspeak_(843i32);
            return ret_val;
        }
    } else if switch5 == 19i32 {
        ret_val = mirpan_(832i32,0i32);
        return ret_val;
    } else if switch5 == 18i32 {
        if prsvec_.prsa != vindex_.spinw {
            if !(prsvec_.prsa != vindex_.movew && (prsvec_.prsa != vindex_.putw) && (prsvec_.prsa != vindex_.trntow)) {
                if prsvec_.prsi != 0i32 {
                    if prsvec_.prsi >= oindex_.num1 && (prsvec_.prsi <= oindex_.num8) {
                        findex_.pnumb = prsvec_.prsi - oindex_.num1 + 1i32;
                        i__1 = findex_.pnumb + 712i32;
                        rspsub_(808i32,i__1);
                        return ret_val;
                    } else {
                        rspeak_(807i32);
                        return ret_val;
                    }
                } else {
                    rspeak_(806i32);
                    return ret_val;
                }
            }
        } else {
            findex_.pnumb = rnd_(8i32) + 1i32;
            i__1 = findex_.pnumb + 712i32;
            rspsub_(797i32,i__1);
            return ret_val;
        }
    } else if switch5 == 17i32 {
        if !(prsvec_.prsa != vindex_.pushw) {
            rspeak_(809i32);
            if objcts_.oflag2[
                   (oindex_.cdoor - 1i32) as (usize)
               ] & 8i32 != 0i32 {
                rspeak_(810i32);
            }
            i__1 = objcts_.olnt;
            i = 1i32;
            'loop161: loop {
                if !(i <= i__1) {
                    break;
                }
                if objcts_.oroom[
                       (i - 1i32) as (usize)
                   ] == rindex_.cell && (objcts_.oflag1[
                                             (i - 1i32) as (usize)
                                         ] & 4096i32 == 0i32) {
                    i__2 = findex_.lcell * hyper_.hfactr;
                    newsta_(i,0i32,i__2,0i32,0i32);
                }
                if objcts_.oroom[
                       (i - 1i32) as (usize)
                   ] == findex_.pnumb * hyper_.hfactr {
                    newsta_(i,0i32,rindex_.cell,0i32,0i32);
                }
                i = i + 1;
            }
            let _rhs = !8i32;
            let _lhs = &mut objcts_.oflag2[(oindex_.odoor - 1i32) as (usize)];
            *_lhs = *_lhs & _rhs;
            let _rhs = !8i32;
            let _lhs = &mut objcts_.oflag2[(oindex_.cdoor - 1i32) as (usize)];
            *_lhs = *_lhs & _rhs;
            let _rhs = !32768i32;
            let _lhs = &mut objcts_.oflag1[(oindex_.odoor - 1i32) as (usize)];
            *_lhs = *_lhs & _rhs;
            if findex_.pnumb == 4i32 {
                let _rhs = 32768i32;
                let _lhs = &mut objcts_.oflag1[(oindex_.odoor - 1i32) as (usize)];
                *_lhs = *_lhs | _rhs;
            }
            if !(advs_.aroom[
                     (aindex_.player - 1i32) as (usize)
                 ] != rindex_.cell) {
                if findex_.lcell != 4i32 {
                    f = moveto_(rindex_.pcell,aindex_.player);
                } else {
                    let _rhs = 32768i32;
                    let _lhs = &mut objcts_.oflag1[(oindex_.odoor - 1i32) as (usize)];
                    *_lhs = *_lhs | _rhs;
                    f = moveto_(rindex_.ncell,aindex_.player);
                }
            }
            findex_.lcell = findex_.pnumb;
            return ret_val;
        }
    } else if switch5 == 16i32 {
        ret_val = opncls_(oindex_.cdoor,779i32,780i32);
        return ret_val;
    } else if switch5 == 15i32 {
        if !(prsvec_.prsa != vindex_.openw) {
            rspeak_(778i32);
            return ret_val;
        }
    } else if switch5 == 14i32 {
        if prsvec_.prsa != vindex_.openw && (prsvec_.prsa != vindex_.closew) {
            if !(prsvec_.prsa != vindex_.knockw) {
                if findex_.inqstf != 0 {
                    rspeak_(798i32);
                    return ret_val;
                } else {
                    findex_.inqstf = 1i32;
                    cevent_.cflag[(cindex_.cevinq - 1i32) as (usize)] = 1i32;
                    cevent_.ctick[(cindex_.cevinq - 1i32) as (usize)] = 2i32;
                    findex_.quesno = rnd_(8i32);
                    findex_.nqatt = 0i32;
                    findex_.corrct = 0i32;
                    rspeak_(768i32);
                    rspeak_(769i32);
                    i__1 = findex_.quesno + 770i32;
                    rspeak_(i__1);
                    return ret_val;
                }
            }
        } else {
            rspeak_(767i32);
            return ret_val;
        }
    } else if switch5 == 13i32 {
        if play_.here == rindex_.ncell || findex_.lcell == 4i32 && (play_.here == rindex_.cell || play_.here == rindex_.scorr) {
            if !(opncls_(oindex_.odoor,764i32,765i32) == 0) {
                if play_.here == rindex_.ncell && (objcts_.oflag2[
                                                       (oindex_.odoor - 1i32) as (usize)
                                                   ] & 8i32 != 0i32) {
                    rspeak_(766i32);
                }
                return ret_val;
            }
        } else {
            rspeak_(763i32);
            return ret_val;
        }
    } else if switch5 == 12i32 {
        if prsvec_.prsa != vindex_.takew || prsvec_.prso != oindex_.rbeam {
            i = prsvec_.prso;
            if prsvec_.prsa == vindex_.putw && (prsvec_.prsi == oindex_.rbeam) {
                _currentBlock = 137;
            } else if prsvec_.prsa != vindex_.mungw || prsvec_.prso != oindex_.rbeam || prsvec_.prsi == 0i32 {
                _currentBlock = 193;
            } else {
                i = prsvec_.prsi;
                _currentBlock = 137;
            }
            if _currentBlock == 193 {
            } else if objcts_.oadv[(i - 1i32) as (usize)] != play_.winner {
                j = 761i32;
                if qhere_(j,play_.here) != 0 {
                    i = 762i32;
                }
                rspsub_(j,objcts_.odesc2[(i - 1i32) as (usize)]);
                return ret_val;
            } else {
                newsta_(i,0i32,play_.here,0i32,0i32);
                rspsub_(760i32,objcts_.odesc2[(i - 1i32) as (usize)]);
                return ret_val;
            }
        } else {
            rspeak_(759i32);
            return ret_val;
        }
    } else if switch5 == 11i32 {
        if !(prsvec_.prsa != vindex_.pushw) {
            if findex_.mrpshf != 0 {
                rspeak_(758i32);
                return ret_val;
            } else {
                rspeak_(756i32);
                i__1 = objcts_.olnt;
                i = 1i32;
                'loop126: loop {
                    if !(i <= i__1) {
                        _currentBlock = 127;
                        break;
                    }
                    if qhere_(i,rindex_.mreye) != 0 && (i != oindex_.rbeam) {
                        _currentBlock = 130;
                        break;
                    }
                    i = i + 1;
                }
                if _currentBlock == 127 {
                    rspeak_(757i32);
                    return ret_val;
                } else {
                    cevent_.cflag[(cindex_.cevmrs - 1i32) as (usize)] = 1i32;
                    cevent_.ctick[(cindex_.cevmrs - 1i32) as (usize)] = 7i32;
                    findex_.mrpshf = 1i32;
                    findex_.mropnf = 1i32;
                    return ret_val;
                }
            }
        }
    } else if switch5 == 10i32 {
        if prsvec_.prsa != vindex_.raisew {
            if !(prsvec_.prsa != vindex_.lowerw && (prsvec_.prsa != vindex_.pushw)) {
                if findex_.poleuf != 0i32 {
                    if findex_.mdir % 180i32 != 0i32 {
                        if findex_.mdir != 270i32 || findex_.mloc != rindex_.mrb {
                            i__1 = findex_.poleuf + 753i32;
                            rspeak_(i__1);
                            findex_.poleuf = 1i32;
                            return ret_val;
                        } else {
                            findex_.poleuf = 0i32;
                            rspeak_(753i32);
                            return ret_val;
                        }
                    } else {
                        findex_.poleuf = 0i32;
                        rspeak_(752i32);
                        return ret_val;
                    }
                } else {
                    rspeak_(751i32);
                    return ret_val;
                }
            }
        } else {
            i = 749i32;
            if findex_.poleuf == 2i32 {
                i = 750i32;
            }
            rspeak_(i);
            findex_.poleuf = 2i32;
            return ret_val;
        }
    } else {
        if switch5 == 9i32 {
            if play_.here != rindex_.cpuzz {
                if play_.here != screen_.scolac {
                    _currentBlock = 98;
                } else {
                    i = 1i32;
                    'loop95: loop {
                        if !(i <= 12i32) {
                            _currentBlock = 98;
                            break;
                        }
                        target = screen_.scolwl[i as (usize)];
                        if screen_.scolwl[(i - 1i32) as (usize)] == play_.here {
                            _currentBlock = 100;
                            break;
                        }
                        i = i + 3i32;
                    }
                }
                if _currentBlock == 100 {
                } else if play_.here != rindex_.bkbox {
                    _currentBlock = 193;
                } else {
                    target = oindex_.wnort;
                    _currentBlock = 100;
                }
            } else if prsvec_.prsa != vindex_.pushw {
                _currentBlock = 193;
            } else {
                i = 1i32;
                'loop79: loop {
                    if !(i <= 8i32) {
                        _currentBlock = 80;
                        break;
                    }
                    if prsvec_.prso == puzzle_.cpwl[(i - 1i32) as (usize)] {
                        _currentBlock = 83;
                        break;
                    }
                    i = i + 2i32;
                }
                if _currentBlock == 80 {
                    bug_(80i32,prsvec_.prso);
                }
                j = puzzle_.cpwl[i as (usize)];
                nxt = findex_.cphere + j;
                wl = puzzle_.cpvec[(nxt - 1i32) as (usize)];
                let switch15 = wl + 4i32;
                if !(switch15 == 5i32) {
                    if switch15 == 4i32 {
                        _currentBlock = 91;
                    } else if switch15 == 3i32 || switch15 == 2i32 || switch15 == 1i32 {
                        if puzzle_.cpvec[(nxt + j - 1i32) as (usize)] == 0i32 {
                            i = 878i32;
                            if findex_.cpushf != 0 {
                                i = 879i32;
                            }
                            findex_.cpushf = 1i32;
                            puzzle_.cpvec[(nxt + j - 1i32) as (usize)] = wl;
                            puzzle_.cpvec[(nxt - 1i32) as (usize)] = 0i32;
                            cpgoto_(nxt);
                            cpinfo_(i,nxt);
                            princr_(1i32,play_.here);
                            let _rhs = 32768i32;
                            let _lhs = &mut rooms_.rflag[(play_.here - 1i32) as (usize)];
                            *_lhs = *_lhs | _rhs;
                            return ret_val;
                        } else {
                            _currentBlock = 92;
                        }
                    } else {
                        _currentBlock = 91;
                    }
                    if _currentBlock == 92 {
                    } else {
                        rspeak_(876i32);
                        return ret_val;
                    }
                }
                rspeak_(877i32);
                return ret_val;
            }
        } else if switch5 == 8i32 {
            if prsvec_.prsa != vindex_.findw {
                if !(prsvec_.prsa != vindex_.examiw) {
                    rspeak_(667i32);
                    return ret_val;
                }
            } else {
                rspeak_(666i32);
                return ret_val;
            }
            _currentBlock = 193;
        } else if switch5 == 7i32 {
            if {
                   i__1 = play_.here - findex_.mloc;
                   if i__1 >= 0i32 { i__1 } else { -i__1 }
               } != 1i32 || mrhere_(
                                play_.here
                            ) != 0i32 || prsvec_.prsa != vindex_.pushw {
                if !(rooms_.rflag[
                         (play_.here - 1i32) as (usize)
                     ] & 32i32 == 0i32) {
                    rspeak_(662i32);
                    return ret_val;
                }
            } else {
                rspeak_(860i32);
                return ret_val;
            }
            _currentBlock = 193;
        } else if switch5 == 6i32 {
            if prsvec_.prsa != vindex_.clmbw && (prsvec_.prsa != vindex_.clmbuw) && (prsvec_.prsa != vindex_.clmbdw) {
                _currentBlock = 193;
            } else {
                rspeak_(648i32);
                return ret_val;
            }
        } else if switch5 == 5i32 {
            if prsvec_.prsa != vindex_.windw {
                _currentBlock = 193;
            } else if prsvec_.prso == oindex_.canar {
                if findex_.singsf == 0 && (play_.here == rindex_.mtree || play_.here >= rindex_.fore1 && (play_.here < rindex_.clear)) {
                    findex_.singsf = 1i32;
                    i = play_.here;
                    if i == rindex_.mtree {
                        i = rindex_.fore3;
                    }
                    newsta_(oindex_.baubl,647i32,i,0i32,0i32);
                    return ret_val;
                } else {
                    rspeak_(646i32);
                    return ret_val;
                }
            } else {
                rspeak_(645i32);
                return ret_val;
            }
        } else if switch5 == 4i32 {
            if prsvec_.prsa != vindex_.openw || prsvec_.prso != oindex_.egg {
                if prsvec_.prsa != vindex_.openw && (prsvec_.prsa != vindex_.mungw) {
                    if prsvec_.prsa != vindex_.dropw || play_.here != rindex_.mtree {
                        _currentBlock = 193;
                    } else {
                        newsta_(oindex_.begg,658i32,rindex_.fore3,0i32,0i32);
                        newsta_(oindex_.egg,0i32,0i32,0i32,0i32);
                        objcts_.otval[(oindex_.begg - 1i32) as (usize)] = 2i32;
                        if objcts_.ocan[(oindex_.canar - 1i32) as (usize)] != oindex_.egg {
                            _currentBlock = 57;
                        } else {
                            objcts_.otval[(oindex_.bcana - 1i32) as (usize)] = 1i32;
                            return ret_val;
                        }
                    }
                } else {
                    i = 655i32;
                    _currentBlock = 52;
                }
            } else {
                if !(objcts_.oflag2[
                         (oindex_.egg - 1i32) as (usize)
                     ] & 8i32 != 0i32) {
                    if prsvec_.prsi != 0i32 {
                        if prsvec_.prsi != oindex_.hands {
                            i = 652i32;
                            if !(objcts_.oflag1[
                                     (prsvec_.prsi - 1i32) as (usize)
                                 ] & 4i32 != 0i32 || objcts_.oflag2[
                                                         (prsvec_.prsi - 1i32) as (usize)
                                                     ] & 512i32 != 0i32) {
                                i = 653i32;
                                if objcts_.oflag2[
                                       (prsvec_.prso - 1i32) as (usize)
                                   ] & 256i32 != 0i32 {
                                    i = 654i32;
                                }
                                let _rhs = 256i32;
                                let _lhs = &mut objcts_.oflag2[(prsvec_.prso - 1i32) as (usize)];
                                *_lhs = *_lhs | _rhs;
                                rspsub_(i,odi2);
                                return ret_val;
                            }
                        } else {
                            rspeak_(651i32);
                            return ret_val;
                        }
                    } else {
                        rspeak_(650i32);
                        return ret_val;
                    }
                } else {
                    rspeak_(649i32);
                    return ret_val;
                }
                _currentBlock = 52;
            }
            if _currentBlock == 193 {
            } else {
                if _currentBlock == 52 {
                    newsta_(
                        oindex_.begg,
                        i,
                        objcts_.oroom[(oindex_.egg - 1i32) as (usize)],
                        objcts_.ocan[(oindex_.egg - 1i32) as (usize)],
                        objcts_.oadv[(oindex_.egg - 1i32) as (usize)]
                    );
                    newsta_(oindex_.egg,0i32,0i32,0i32,0i32);
                    objcts_.otval[(oindex_.begg - 1i32) as (usize)] = 2i32;
                    if !(objcts_.ocan[
                             (oindex_.canar - 1i32) as (usize)
                         ] != oindex_.egg) {
                        rspeak_(objcts_.odesco[(oindex_.bcana - 1i32) as (usize)]);
                        objcts_.otval[(oindex_.bcana - 1i32) as (usize)] = 1i32;
                        return ret_val;
                    }
                }
                newsta_(oindex_.bcana,0i32,0i32,0i32,0i32);
                return ret_val;
            }
        } else if switch5 == 3i32 {
            if prsvec_.prsa != vindex_.givew && (prsvec_.prsa != vindex_.throww) {
                if prsvec_.prsa != vindex_.attacw && (prsvec_.prsa != vindex_.killw) && (prsvec_.prsa != vindex_.mungw) {
                    rspeak_(644i32);
                    return ret_val;
                } else {
                    newsta_(oindex_.zgnom,643i32,0i32,0i32,0i32);
                    cevent_.ctick[(cindex_.cevzgo - 1i32) as (usize)] = 0i32;
                    return ret_val;
                }
            } else if objcts_.otval[(prsvec_.prso - 1i32) as (usize)] != 0i32 {
                newsta_(prsvec_.prso,0i32,0i32,0i32,0i32);
                rspsub_(642i32,odo2);
                newsta_(oindex_.zgnom,0i32,0i32,0i32,0i32);
                cevent_.ctick[(cindex_.cevzgo - 1i32) as (usize)] = 0i32;
                f = moveto_(rindex_.bkent,play_.winner);
                return ret_val;
            } else {
                newsta_(prsvec_.prso,641i32,0i32,0i32,0i32);
                return ret_val;
            }
        } else if switch5 == 2i32 {
            target = oindex_.scol;
            _currentBlock = 100;
        } else {
            if switch5 == 1i32 {
                if prsvec_.prsa != vindex_.eatw {
                    if prsvec_.prsa == vindex_.burnw {
                        rspeak_(640i32);
                    }
                } else {
                    rspeak_(639i32);
                    return ret_val;
                }
            } else {
                bug_(6i32,ri);
            }
            _currentBlock = 193;
        }
        if _currentBlock == 193 {
        } else {
            if !(prsvec_.prso != target) {
                if prsvec_.prsa != vindex_.pushw && (prsvec_.prsa != vindex_.movew) && (prsvec_.prsa != vindex_.takew) && (prsvec_.prsa != vindex_.rubw) {
                    if !(prsvec_.prsa != vindex_.killw && (prsvec_.prsa != vindex_.attacw) && (prsvec_.prsa != vindex_.mungw)) {
                        rspsub_(674i32,odi2);
                        return ret_val;
                    }
                } else {
                    rspeak_(673i32);
                    return ret_val;
                }
            }
            if !(prsvec_.prsa != vindex_.throww || prsvec_.prsi != target) {
                if play_.here == rindex_.bkbox {
                    if screen_.scolrm == 0i32 {
                        rspeak_(213i32);
                        return ret_val;
                    } else {
                        newsta_(prsvec_.prso,0i32,screen_.scolrm,0i32,0i32);
                        rspsub_(676i32,odo2);
                        cevent_.ctick[(cindex_.cevscl - 1i32) as (usize)] = 0i32;
                        screen_.scolrm = 0i32;
                        return ret_val;
                    }
                } else {
                    newsta_(prsvec_.prso,0i32,rindex_.bkbox,0i32,0i32);
                    rspsub_(675i32,odo2);
                    cevent_.ctick[(cindex_.cevscl - 1i32) as (usize)] = 0i32;
                    screen_.scolrm = 0i32;
                    return ret_val;
                }
            }
        }
    }
    ret_val = 0i32;
    ret_val
}

unsafe extern fn mirpan_(mut st : i32, mut pnf : i32) -> i32 {
    let mut i__1 : i32;
    let mut ret_val : i32;
    let mut num : i32;
    let mut mrbf : i32;
    ret_val = 1i32;
    num = mrhere_(play_.here);
    if num != 0i32 {
        mrbf = 0i32;
        if num == 1i32 && (findex_.mr1f == 0) || num == 2i32 && (findex_.mr2f == 0) {
            mrbf = 1i32;
        }
        (if prsvec_.prsa != vindex_.movew && (prsvec_.prsa != vindex_.openw) {
             (if pnf != 0 || prsvec_.prsa != vindex_.lookiw && (prsvec_.prsa != vindex_.examiw) && (prsvec_.prsa != vindex_.lookw) {
                  (if prsvec_.prsa != vindex_.mungw {
                       (if pnf != 0 || mrbf == 0i32 {
                            (if prsvec_.prsa != vindex_.pushw {
                                 ret_val = 0i32;
                                 ret_val
                             } else {
                                 i__1 = st + 3i32 + num;
                                 rspeak_(i__1);
                                 ret_val
                             })
                        } else {
                            rspeak_(846i32);
                            ret_val
                        })
                   } else {
                       i__1 = st + 2i32 + mrbf;
                       rspeak_(i__1);
                       if num == 1i32 && (pnf == 0) {
                           findex_.mr1f = 0i32;
                       }
                       if num == 2i32 && (pnf == 0) {
                           findex_.mr2f = 0i32;
                       }
                       ret_val
                   })
              } else {
                  i__1 = mrbf + 844i32;
                  rspeak_(i__1);
                  ret_val
              })
         } else {
             i__1 = st + 1i32;
             rspeak_(i__1);
             ret_val
         })
    } else {
        rspeak_(st);
        ret_val
    }
}
