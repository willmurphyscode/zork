extern {
    static mut advs_ : Struct8;
    static aflags_ : Struct10;
    static aindex_ : Struct9;
    static mut cevent_ : Struct13;
    static cindex_ : Struct14;
    static mut curxt_ : Struct17;
    fn fights_(arg1 : i32, arg2 : i32) -> i32;
    static mut findex_ : Struct5;
    fn findxt_(arg1 : i32, arg2 : i32) -> i32;
    fn fwim_(
        arg1 : i32,
        arg2 : i32,
        arg3 : i32,
        arg4 : i32,
        arg5 : i32,
        arg6 : i32
    ) -> i32;
    static mut hack_ : Struct18;
    fn jigsup_(arg1 : i32);
    fn newsta_(
        arg1 : i32, arg2 : i32, arg3 : i32, arg4 : i32, arg5 : i32
    );
    fn oappli_(arg1 : i32, arg2 : i32) -> i32;
    static mut objcts_ : Struct2;
    static oindex_ : Struct4;
    static mut play_ : Struct3;
    fn prob_(arg1 : i32, arg2 : i32) -> i32;
    static mut prsvec_ : Struct6;
    static rindex_ : Struct19;
    fn rnd_(arg1 : i32) -> i32;
    fn rspeak_(arg1 : i32);
    fn rspsub_(arg1 : i32, arg2 : i32);
    static mut star_ : Struct11;
    static mut vill_ : Struct1;
    fn vilstr_(arg1 : i32) -> i32;
    static vindex_ : Struct7;
    static xsrch_ : Struct15;
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct1 {
    pub vlnt : i32,
    pub villns : [i32; 4],
    pub vprob : [i32; 4],
    pub vopps : [i32; 4],
    pub vbest : [i32; 4],
    pub vmelee : [i32; 4],
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
    pub winner : i32,
    pub here : i32,
    pub telflg : i32,
}

impl Clone for Struct3 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct4 {
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

impl Clone for Struct4 {
    fn clone(&self) -> Self { *self }
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

#[derive(Copy)]
#[repr(C)]
pub struct Struct6 {
    pub prsa : i32,
    pub prsi : i32,
    pub prso : i32,
    pub prswon : i32,
    pub prscon : i32,
}

impl Clone for Struct6 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct7 {
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

impl Clone for Struct7 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct8 {
    pub alnt : i32,
    pub aroom : [i32; 4],
    pub ascore : [i32; 4],
    pub avehic : [i32; 4],
    pub aobj : [i32; 4],
    pub aactio : [i32; 4],
    pub astren : [i32; 4],
    pub aflag : [i32; 4],
}

impl Clone for Struct8 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct9 {
    pub player : i32,
    pub arobot : i32,
    pub amastr : i32,
}

impl Clone for Struct9 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct10 {
    pub astag : i32,
}

impl Clone for Struct10 {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub unsafe extern fn fightd_() {
    let mut _currentBlock;
    let rout : i32 = 1i32;
    let mut i__1 : i32;
    let mut i__2 : i32;
    let mut f : i32;
    let mut i : i32;
    let mut j : i32;
    let mut ra : i32;
    let mut obj : i32;
    let mut res : i32;
    let mut out : i32;
    i__1 = vill_.vlnt;
    i = 1i32;
    'loop1: loop {
        if !(i <= i__1) {
            break;
        }
        vill_.vopps[(i - 1i32) as (usize)] = 0i32;
        obj = vill_.villns[(i - 1i32) as (usize)];
        ra = objcts_.oactio[(obj - 1i32) as (usize)];
        if play_.here != objcts_.oroom[(obj - 1i32) as (usize)] {
            if !(objcts_.oflag2[
                     (obj - 1i32) as (usize)
                 ] & 256i32 == 0i32 || ra == 0i32) {
                prsvec_.prsa = vindex_.fightw;
                f = oappli_(ra,0i32);
            }
            if obj == oindex_.thief {
                findex_.thfenf = 0i32;
            }
            let _rhs = !aflags_.astag;
            let _lhs = &mut advs_.aflag[(aindex_.player - 1i32) as (usize)];
            *_lhs = *_lhs & _rhs;
            let _rhs = !(64i32 + 256i32);
            let _lhs = &mut objcts_.oflag2[(obj - 1i32) as (usize)];
            *_lhs = *_lhs & _rhs;
            if !(objcts_.ocapac[
                     (obj - 1i32) as (usize)
                 ] >= 0i32 || ra == 0i32) {
                prsvec_.prsa = vindex_.inxw;
                f = oappli_(ra,0i32);
                objcts_.ocapac[(obj - 1i32) as (usize)] = {
                                                              i__2 = objcts_.ocapac[
                                                                         (obj - 1i32) as (usize)
                                                                     ];
                                                              if i__2 >= 0i32 {
                                                                  i__2
                                                              } else {
                                                                  -i__2
                                                              }
                                                          };
            }
        } else if !(obj == oindex_.thief && (findex_.thfenf != 0)) {
            if objcts_.ocapac[(obj - 1i32) as (usize)] >= 0i32 {
                if objcts_.oflag2[(obj - 1i32) as (usize)] & 256i32 == 0i32 {
                    if !(ra == 0i32) {
                        prsvec_.prsa = vindex_.frstqw;
                        if !(oappli_(ra,0i32) == 0) {
                            let _rhs = 256i32;
                            let _lhs = &mut objcts_.oflag2[(obj - 1i32) as (usize)];
                            *_lhs = *_lhs | _rhs;
                            vill_.vopps[(i - 1i32) as (usize)] = obj;
                        }
                    }
                } else {
                    vill_.vopps[(i - 1i32) as (usize)] = obj;
                }
            } else if vill_.vprob[(i - 1i32) as (usize)] == 0i32 || prob_(
                                                                        vill_.vprob[
                                                                            (i - 1i32) as (usize)
                                                                        ],
                                                                        vill_.vprob[
                                                                            (i - 1i32) as (usize)
                                                                        ]
                                                                    ) == 0 {
                let _rhs = 10i32;
                let _lhs = &mut vill_.vprob[(i - 1i32) as (usize)];
                *_lhs = *_lhs + _rhs;
            } else {
                objcts_.ocapac[(obj - 1i32) as (usize)] = {
                                                              i__2 = objcts_.ocapac[
                                                                         (obj - 1i32) as (usize)
                                                                     ];
                                                              if i__2 >= 0i32 {
                                                                  i__2
                                                              } else {
                                                                  -i__2
                                                              }
                                                          };
                vill_.vprob[(i - 1i32) as (usize)] = 0i32;
                if !(ra == 0i32) {
                    prsvec_.prsa = vindex_.inxw;
                    f = oappli_(ra,0i32);
                }
            }
        }
        i = i + 1;
    }
    out = 0i32;
    'loop3: loop {
        i__1 = vill_.vlnt;
        i = 1i32;
        'loop4: loop {
            if !(i <= i__1) {
                break;
            }
            j = vill_.vopps[(i - 1i32) as (usize)];
            if !(j == 0i32) {
                prsvec_.prscon = 1i32;
                ra = objcts_.oactio[(j - 1i32) as (usize)];
                if ra == 0i32 {
                    _currentBlock = 10;
                } else {
                    prsvec_.prsa = vindex_.fightw;
                    if oappli_(ra,0i32) != 0 {
                        _currentBlock = 14;
                    } else {
                        _currentBlock = 10;
                    }
                }
                if _currentBlock == 14 {
                } else {
                    res = blow_(
                              aindex_.player,
                              j,
                              vill_.vmelee[(i - 1i32) as (usize)],
                              0i32,
                              out
                          );
                    if res < 0i32 {
                        _currentBlock = 13;
                        break 'loop3;
                    }
                    if res == rout {
                        out = rnd_(3i32) + 2i32;
                    }
                }
            }
            i = i + 1;
        }
        out = out - 1;
        if !(out > 0i32) {
            _currentBlock = 6;
            break;
        }
    }
    if _currentBlock == 6 { }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct11 {
    pub mbase : i32,
    pub strbit : i32,
}

impl Clone for Struct11 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct13 {
    pub clnt : i32,
    pub ctick : [i32; 25],
    pub cactio : [i32; 25],
    pub cflag : [i32; 25],
}

impl Clone for Struct13 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct14 {
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

impl Clone for Struct14 {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub unsafe extern fn blow_(
    mut h : i32,
    mut v : i32,
    mut rmk : i32,
    mut hflg : i32,
    mut out : i32
) -> i32 {
    let rmiss : i32 = 0i32;
    let rout : i32 = 1i32;
    let rkill : i32 = 2i32;
    let rstag : i32 = 5i32;
    let rlose : i32 = 6i32;
    let rhes : i32 = 7i32;
    let rsit : i32 = 8i32;
    static mut def1r : [i32; 3] = [ 1i32, 2i32, 3i32 ];
    static mut def2r : [i32; 4] = [ 13i32, 23i32, 24i32, 25i32 ];
    static mut def3r
        : [i32; 5]
        = [ 35i32, 36i32, 46i32, 47i32, 57i32 ];
    static mut rvectr
        : [i32; 66]
        = [   0i32,
              0i32,
              0i32,
              0i32,
              5i32,
              5i32,
              1i32,
              1i32,
              2i32,
              2i32,
              2i32,
              2i32,
              0i32,
              0i32,
              0i32,
              0i32,
              0i32,
              5i32,
              5i32,
              3i32,
              3i32,
              1i32,
              0i32,
              0i32,
              0i32,
              5i32,
              5i32,
              3i32,
              3i32,
              3i32,
              1i32,
              2i32,
              2i32,
              2i32,
              0i32,
              0i32,
              0i32,
              0i32,
              0i32,
              5i32,
              5i32,
              3i32,
              3i32,
              4i32,
              4i32,
              0i32,
              0i32,
              0i32,
              5i32,
              5i32,
              3i32,
              3i32,
              3i32,
              4i32,
              4i32,
              4i32,
              0i32,
              5i32,
              5i32,
              3i32,
              3i32,
              3i32,
              3i32,
              4i32,
              4i32,
              4i32
          ];
    static mut rstate
        : [i32; 45]
        = [   5000i32,
              3005i32,
              3008i32,
              4011i32,
              3015i32,
              3018i32,
              1021i32,
              0i32,
              0i32,
              5022i32,
              3027i32,
              3030i32,
              4033i32,
              3037i32,
              3040i32,
              1043i32,
              0i32,
              0i32,
              4044i32,
              2048i32,
              4050i32,
              4054i32,
              5058i32,
              4063i32,
              4067i32,
              3071i32,
              1074i32,
              4075i32,
              1079i32,
              4080i32,
              4084i32,
              4088i32,
              4092i32,
              4096i32,
              4100i32,
              1104i32,
              4105i32,
              2109i32,
              4111i32,
              4115i32,
              4119i32,
              4123i32,
              4127i32,
              3131i32,
              3134i32
          ];
    let mut ret_val : i32;
    let mut i__1 : i32;
    let mut i__2 : i32;
    let mut f : i32;
    let mut i : i32;
    let mut j : i32;
    let mut oa : i32;
    let mut ra : i32;
    let mut od : i32;
    let mut mi : i32;
    let mut dv : i32;
    let mut def : i32;
    let mut tbl : i32;
    let mut att : i32;
    let mut res : i32;
    let mut dweap : i32;
    let mut pblose : i32;
    ra = objcts_.oactio[(v - 1i32) as (usize)];
    dv = objcts_.odesc2[(v - 1i32) as (usize)];
    ret_val = rmiss;
    if hflg == 0 {
        pblose = 50i32;
        let _rhs = !aflags_.astag;
        let _lhs = &mut advs_.aflag[(h - 1i32) as (usize)];
        *_lhs = *_lhs & _rhs;
        if objcts_.oflag2[(v - 1i32) as (usize)] & 64i32 == 0i32 {
            att = vilstr_(v);
            oa = att;
            def = fights_(h,1i32);
            if def <= 0i32 {
                return ret_val;
            } else {
                od = fights_(h,0i32);
                dweap = {
                            i__1 = fwim_(0i32,512i32,0i32,0i32,h,1i32);
                            if i__1 >= 0i32 { i__1 } else { -i__1 }
                        };
            }
        } else {
            let _rhs = !64i32;
            let _lhs = &mut objcts_.oflag2[(v - 1i32) as (usize)];
            *_lhs = *_lhs & _rhs;
            rspsub_(594i32,dv);
            return ret_val;
        }
    } else {
        pblose = 10i32;
        let _rhs = 256i32;
        let _lhs = &mut objcts_.oflag2[(v - 1i32) as (usize)];
        *_lhs = *_lhs | _rhs;
        if advs_.aflag[(h - 1i32) as (usize)] & aflags_.astag == 0i32 {
            att = fights_(h,1i32);
            oa = att;
            def = vilstr_(v);
            od = def;
            dweap = 0i32;
            i__1 = objcts_.olnt;
            i = 1i32;
            'loop4: loop {
                if !(i <= i__1) {
                    break;
                }
                if objcts_.ocan[(i - 1i32) as (usize)] == v && (objcts_.oflag2[
                                                                    (i - 1i32) as (usize)
                                                                ] & 512i32 != 0i32) {
                    dweap = i;
                }
                i = i + 1;
            }
            if v == advs_.aobj[(aindex_.player - 1i32) as (usize)] {
                jigsup_(593i32);
                return ret_val;
            } else if !(def != 0i32) {
                rspsub_(592i32,dv);
                return ret_val;
            }
        } else {
            rspeak_(591i32);
            let _rhs = !aflags_.astag;
            let _lhs = &mut advs_.aflag[(h - 1i32) as (usize)];
            *_lhs = *_lhs & _rhs;
            return ret_val;
        }
    }
    if def > 0i32 {
        if {
               i__1 = def - 2i32;
               i__1
           } < 0i32 {
            att = if att <= 3i32 { att } else { 3i32 };
            tbl = def1r[(att - 1i32) as (usize)];
        } else if i__1 == 0i32 {
            att = if att <= 4i32 { att } else { 4i32 };
            tbl = def2r[(att - 1i32) as (usize)];
        } else {
            att = att - def;
            i__1 = 2i32;
            i__2 = if -2i32 >= att { -2i32 } else { att };
            att = if i__1 <= i__2 { i__1 } else { i__2 } + 3i32;
            tbl = def3r[(att - 1i32) as (usize)];
        }
        res = rvectr[(tbl + rnd_(10i32) - 1i32) as (usize)];
        if !(out == 0i32) {
            if res == rstag {
                res = rhes;
            } else {
                res = rsit;
            }
        }
        if res == rstag && (dweap != 0i32) && (prob_(25i32,pblose) != 0) {
            res = rlose;
        }
        mi = rstate[((rmk - 1i32) * 9i32 + res) as (usize)];
        if !(mi == 0i32) {
            i__1 = mi / 1000i32;
            i = mi % 1000i32 + rnd_(i__1) + star_.mbase + 1i32;
            j = dv;
            if hflg == 0 && (dweap != 0i32) {
                j = objcts_.odesc2[(dweap - 1i32) as (usize)];
            }
            rspsub_(i,j);
        }
    } else {
        res = rkill;
        if hflg != 0 {
            rspsub_(595i32,dv);
        }
    }
    let switch12 = res + 1i32;
    if switch12 == 9i32 || switch12 == 3i32 {
        def = 0i32;
    } else if !(switch12 == 8i32 || switch12 == 1i32) {
        if switch12 == 7i32 {
            newsta_(dweap,0i32,play_.here,0i32,0i32);
            dweap = 0i32;
            if hflg == 0 {
                dweap = {
                            i__1 = fwim_(0i32,512i32,0i32,0i32,h,1i32);
                            if i__1 >= 0i32 { i__1 } else { -i__1 }
                        };
                if dweap != 0i32 {
                    rspsub_(605i32,objcts_.odesc2[(dweap - 1i32) as (usize)]);
                }
            }
        } else if switch12 == 6i32 {
            if hflg != 0 {
                let _rhs = 64i32;
                let _lhs = &mut objcts_.oflag2[(v - 1i32) as (usize)];
                *_lhs = *_lhs | _rhs;
            } else {
                let _rhs = aflags_.astag;
                let _lhs = &mut advs_.aflag[(h - 1i32) as (usize)];
                *_lhs = *_lhs | _rhs;
            }
        } else if switch12 == 5i32 {
            i__1 = 0i32;
            i__2 = def - 2i32;
            def = if i__1 >= i__2 { i__1 } else { i__2 };
        } else if switch12 == 4i32 {
            i__1 = 0i32;
            i__2 = def - 1i32;
            def = if i__1 >= i__2 { i__1 } else { i__2 };
        } else {
            switch12 == 2i32;
            if hflg != 0 {
                def = -def;
            }
        }
    }
    ret_val = res;
    if hflg == 0 {
        advs_.astren[(h - 1i32) as (usize)] = -10000i32;
        if def != 0i32 {
            advs_.astren[(h - 1i32) as (usize)] = def - od;
        }
        if !(def >= od) {
            cevent_.ctick[(cindex_.cevcur - 1i32) as (usize)] = 30i32;
            cevent_.cflag[(cindex_.cevcur - 1i32) as (usize)] = 1i32;
        }
        (if fights_(h,1i32) > 0i32 {
             ret_val
         } else {
             advs_.astren[(h - 1i32) as (usize)] = 1i32 - fights_(h,0i32);
             jigsup_(596i32);
             ret_val = -1i32;
             ret_val
         })
    } else {
        objcts_.ocapac[(v - 1i32) as (usize)] = def;
        (if def != 0i32 {
             (if res != rout || ra == 0i32 {
                  ret_val
              } else {
                  prsvec_.prsa = vindex_.outxw;
                  f = oappli_(ra,0i32);
                  ret_val
              })
         } else {
             let _rhs = !256i32;
             let _lhs = &mut objcts_.oflag2[(v - 1i32) as (usize)];
             *_lhs = *_lhs & _rhs;
             rspsub_(572i32,dv);
             newsta_(v,0i32,0i32,0i32,0i32);
             (if ra == 0i32 {
                  ret_val
              } else {
                  prsvec_.prsa = vindex_.deadxw;
                  f = oappli_(ra,0i32);
                  ret_val
              })
         })
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct15 {
    pub xmin : i32,
    pub xmax : i32,
    pub xdown : i32,
    pub xup : i32,
    pub xnorth : i32,
    pub xsouth : i32,
    pub xenter : i32,
    pub xexit : i32,
    pub xeast : i32,
    pub xwest : i32,
}

impl Clone for Struct15 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct17 {
    pub xtype : i32,
    pub xroom1 : i32,
    pub xstrng : i32,
    pub xactio : i32,
    pub xobj : i32,
}

impl Clone for Struct17 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct18 {
    pub thfpos : i32,
    pub thfflg : i32,
    pub thfact : i32,
    pub swdact : i32,
    pub swdsta : i32,
}

impl Clone for Struct18 {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub unsafe extern fn swordd_() {
    let mut _currentBlock;
    let mut i__1 : i32;
    let mut i__2 : i32;
    let mut i : i32;
    let mut ng : i32;
    if objcts_.oadv[
           (oindex_.sword - 1i32) as (usize)
       ] != aindex_.player {
        hack_.swdact = 0i32;
    } else {
        ng = 2i32;
        if infest_(play_.here) == 0 {
            ng = 1i32;
            i__1 = xsrch_.xmax;
            i__2 = xsrch_.xmin;
            i = xsrch_.xmin;
            'loop3: loop {
                if if i__2 < 0i32 {
                       (i >= i__1) as (i32)
                   } else {
                       (i <= i__1) as (i32)
                   } == 0 {
                    _currentBlock = 4;
                    break;
                }
                if !(findxt_(i,play_.here) == 0) {
                    let switch16 = curxt_.xtype;
                    if !(switch16 == 2i32) {
                        switch16 == 4i32 || switch16 == 3i32 || switch16 == 1i32;
                        if infest_(curxt_.xroom1) != 0 {
                            _currentBlock = 11;
                            break;
                        }
                    }
                }
                i = i + i__2;
            }
            if _currentBlock == 11 {
            } else {
                ng = 0i32;
            }
        }
        (if ng == hack_.swdsta {
         } else {
             i__2 = ng + 495i32;
             rspeak_(i__2);
             hack_.swdsta = ng;
         })
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct19 {
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

impl Clone for Struct19 {
    fn clone(&self) -> Self { *self }
}

unsafe extern fn infest_(mut r : i32) -> i32 {
    let mut ret_val : i32;
    if findex_.endgmf == 0 {
        ret_val = (objcts_.oroom[
                       (oindex_.cyclo - 1i32) as (usize)
                   ] == r || objcts_.oroom[
                                 (oindex_.troll - 1i32) as (usize)
                             ] == r || objcts_.oroom[
                                           (oindex_.thief - 1i32) as (usize)
                                       ] == r && (hack_.thfact != 0)) as (i32);
    } else {
        ret_val = (r == rindex_.mrg || r == rindex_.mrge || r == rindex_.mrgw || r == rindex_.inmir && (findex_.mloc == rindex_.mrg)) as (i32);
    }
    ret_val
}
