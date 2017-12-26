extern {
    fn bug_(arg1 : i32, arg2 : i32);
    static mut cevent_ : Struct3;
    static cindex_ : Struct4;
    static mut findex_ : Struct2;
    fn newsta_(
        arg1 : i32, arg2 : i32, arg3 : i32, arg4 : i32, arg5 : i32
    );
    static mut objcts_ : Struct7;
    static oindex_ : Struct1;
    static mut prsvec_ : Struct5;
    fn rspeak_(arg1 : i32);
    static vindex_ : Struct6;
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct1 {
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

impl Clone for Struct1 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct2 {
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

impl Clone for Struct2 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct3 {
    pub clnt : i32,
    pub ctick : [i32; 25],
    pub cactio : [i32; 25],
    pub cflag : [i32; 25],
}

impl Clone for Struct3 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct4 {
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

impl Clone for Struct4 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct5 {
    pub prsa : i32,
    pub prsi : i32,
    pub prso : i32,
    pub prswon : i32,
    pub prscon : i32,
}

impl Clone for Struct5 {
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

impl Clone for Struct7 {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub unsafe extern fn lightp_(mut obj : i32) -> i32 {
    let mut ret_val : i32;
    let mut flobts : i32;
    let mut i : i32;
    ret_val = 1i32;
    flobts = 8i32 + 64i32 + 1i32;
    if obj != oindex_.candl {
        if obj != oindex_.match_ {
            bug_(6i32,obj);
        }
        if prsvec_.prsa != vindex_.trnonw || prsvec_.prso != oindex_.match_ {
            if !(prsvec_.prsa != vindex_.trnofw || objcts_.oflag1[
                                                       (oindex_.match_ - 1i32) as (usize)
                                                   ] & 1i32 == 0i32) {
                let _rhs = !flobts;
                let _lhs = &mut objcts_.oflag1[(oindex_.match_ - 1i32) as (usize)];
                *_lhs = *_lhs & _rhs;
                cevent_.ctick[(cindex_.cevmat - 1i32) as (usize)] = 0i32;
                rspeak_(185i32);
                return ret_val;
            }
        } else if findex_.ormtch != 0i32 {
            findex_.ormtch = findex_.ormtch - 1;
            let _rhs = flobts;
            let _lhs = &mut objcts_.oflag1[(oindex_.match_ - 1i32) as (usize)];
            *_lhs = *_lhs | _rhs;
            cevent_.ctick[(cindex_.cevmat - 1i32) as (usize)] = 2i32;
            rspeak_(184i32);
            return ret_val;
        } else {
            rspeak_(183i32);
            return ret_val;
        }
    } else {
        if !(findex_.orcand != 0i32) {
            findex_.orcand = 1i32;
            cevent_.ctick[(cindex_.cevcnd - 1i32) as (usize)] = 50i32;
        }
        if !(prsvec_.prsi == oindex_.candl) {
            if prsvec_.prsa != vindex_.trnofw {
                if !(prsvec_.prsa != vindex_.burnw && (prsvec_.prsa != vindex_.trnonw)) {
                    if objcts_.oflag1[
                           (oindex_.candl - 1i32) as (usize)
                       ] & 64i32 != 0i32 {
                        if prsvec_.prsi != 0i32 {
                            if prsvec_.prsi != oindex_.match_ || !(objcts_.oflag1[
                                                                       (oindex_.match_ - 1i32) as (usize)
                                                                   ] & 1i32 != 0i32) {
                                if prsvec_.prsi != oindex_.torch || !(objcts_.oflag1[
                                                                          (oindex_.torch - 1i32) as (usize)
                                                                      ] & 1i32 != 0i32) {
                                    rspeak_(519i32);
                                    return ret_val;
                                } else if objcts_.oflag1[
                                              (oindex_.candl - 1i32) as (usize)
                                          ] & 1i32 != 0i32 {
                                    rspeak_(520i32);
                                    return ret_val;
                                } else {
                                    newsta_(oindex_.candl,521i32,0i32,0i32,0i32);
                                    return ret_val;
                                }
                            } else {
                                i = 517i32;
                                if objcts_.oflag1[
                                       (oindex_.candl - 1i32) as (usize)
                                   ] & 1i32 != 0i32 {
                                    i = 518i32;
                                }
                                let _rhs = 1i32;
                                let _lhs = &mut objcts_.oflag1[(oindex_.candl - 1i32) as (usize)];
                                *_lhs = *_lhs | _rhs;
                                cevent_.cflag[(cindex_.cevcnd - 1i32) as (usize)] = 1i32;
                                rspeak_(i);
                                return ret_val;
                            }
                        } else {
                            rspeak_(516i32);
                            prsvec_.prswon = 0i32;
                            return ret_val;
                        }
                    } else {
                        rspeak_(515i32);
                        return ret_val;
                    }
                }
            } else {
                i = 513i32;
                if objcts_.oflag1[
                       (oindex_.candl - 1i32) as (usize)
                   ] & 1i32 != 0i32 {
                    i = 514i32;
                }
                cevent_.cflag[(cindex_.cevcnd - 1i32) as (usize)] = 0i32;
                let _rhs = !1i32;
                let _lhs = &mut objcts_.oflag1[(oindex_.candl - 1i32) as (usize)];
                *_lhs = *_lhs & _rhs;
                rspeak_(i);
                return ret_val;
            }
        }
    }
    ret_val = 0i32;
    ret_val
}
