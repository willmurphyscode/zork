extern {
    static mut advs_ : Struct9;
    static aindex_ : Struct7;
    fn bug_(arg1 : i32, arg2 : i32);
    static mut cevent_ : Struct4;
    static cindex_ : Struct5;
    static mut findex_ : Struct10;
    static mut hack_ : Struct13;
    fn lit_(arg1 : i32) -> i32;
    fn moveto_(arg1 : i32, arg2 : i32) -> i32;
    fn newsta_(
        arg1 : i32, arg2 : i32, arg3 : i32, arg4 : i32, arg5 : i32
    );
    static mut objcts_ : Struct11;
    static oindex_ : Struct3;
    static mut play_ : Struct6;
    fn prob_(arg1 : i32, arg2 : i32) -> i32;
    static mut prsvec_ : Struct1;
    fn qhere_(arg1 : i32, arg2 : i32) -> i32;
    static rindex_ : Struct8;
    fn robadv_(arg1 : i32, arg2 : i32, arg3 : i32, arg4 : i32) -> i32;
    fn robrm_(
        arg1 : i32, arg2 : i32, arg3 : i32, arg4 : i32, arg5 : i32
    ) -> i32;
    static mut rooms_ : Struct14;
    fn rspeak_(arg1 : i32);
    fn rspsub_(arg1 : i32, arg2 : i32);
    static vindex_ : Struct2;
    fn winnin_(arg1 : i32, arg2 : i32) -> i32;
    static xsrch_ : Struct12;
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

impl Clone for Struct2 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct3 {
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

impl Clone for Struct3 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct4 {
    pub clnt : i32,
    pub ctick : [i32; 25],
    pub cactio : [i32; 25],
    pub cflag : [i32; 25],
}

impl Clone for Struct4 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct5 {
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

impl Clone for Struct5 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct6 {
    pub winner : i32,
    pub here : i32,
    pub telflg : i32,
}

impl Clone for Struct6 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct7 {
    pub player : i32,
    pub arobot : i32,
    pub amastr : i32,
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
    pub alnt : i32,
    pub aroom : [i32; 4],
    pub ascore : [i32; 4],
    pub avehic : [i32; 4],
    pub aobj : [i32; 4],
    pub aactio : [i32; 4],
    pub astren : [i32; 4],
    pub aflag : [i32; 4],
}

impl Clone for Struct9 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct10 {
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

impl Clone for Struct10 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct11 {
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

impl Clone for Struct11 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct12 {
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

impl Clone for Struct12 {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub unsafe extern fn aappli_(mut ri : i32) -> i32 {
    let mut ret_val : i32;
    let mut f : i32;
    let mut i : i32;
    if !(ri == 0i32) {
        ret_val = 1i32;
        if ri == 2i32 {
            if objcts_.oflag2[
                   (oindex_.qdoor - 1i32) as (usize)
               ] & 8i32 != 0i32 {
                if prsvec_.prsa != vindex_.walkw {
                    if !(prsvec_.prsa == vindex_.takew || prsvec_.prsa == vindex_.dropw || prsvec_.prsa == vindex_.putw || prsvec_.prsa == vindex_.throww || prsvec_.prsa == vindex_.pushw || prsvec_.prsa == vindex_.turnw || prsvec_.prsa == vindex_.spinw || prsvec_.prsa == vindex_.trntow || prsvec_.prsa == vindex_.follow || prsvec_.prsa == vindex_.stayw || prsvec_.prsa == vindex_.openw || prsvec_.prsa == vindex_.closew || prsvec_.prsa == vindex_.killw) {
                        rspeak_(786i32);
                        return ret_val;
                    }
                } else {
                    i = 784i32;
                    if play_.here == rindex_.scorr && (prsvec_.prso == xsrch_.xnorth || prsvec_.prso == xsrch_.xenter) || play_.here == rindex_.ncorr && (prsvec_.prso == xsrch_.xsouth || prsvec_.prso == xsrch_.xenter) {
                        i = 785i32;
                    }
                    rspeak_(i);
                    return ret_val;
                }
            } else {
                rspeak_(783i32);
                return ret_val;
            }
        } else if ri == 1i32 {
            if prsvec_.prsa != vindex_.raisew || prsvec_.prso != oindex_.rcage {
                if prsvec_.prsa != vindex_.drinkw && (prsvec_.prsa != vindex_.eatw) {
                    if prsvec_.prsa != vindex_.readw {
                        if !(prsvec_.prsa == vindex_.walkw || prsvec_.prsa == vindex_.takew || prsvec_.prsa == vindex_.dropw || prsvec_.prsa == vindex_.putw || prsvec_.prsa == vindex_.pushw || prsvec_.prsa == vindex_.throww || prsvec_.prsa == vindex_.turnw || prsvec_.prsa == vindex_.leapw) {
                            rspeak_(570i32);
                            return ret_val;
                        }
                    } else {
                        rspeak_(569i32);
                        return ret_val;
                    }
                } else {
                    rspeak_(568i32);
                    return ret_val;
                }
            } else {
                cevent_.cflag[(cindex_.cevsph - 1i32) as (usize)] = 0i32;
                play_.winner = aindex_.player;
                f = moveto_(rindex_.cager,play_.winner);
                newsta_(oindex_.cage,567i32,rindex_.cager,0i32,0i32);
                newsta_(oindex_.robot,0i32,rindex_.cager,0i32,0i32);
                advs_.aroom[(aindex_.arobot - 1i32) as (usize)] = rindex_.cager;
                findex_.cagesf = 1i32;
                let _rhs = !512i32;
                let _lhs = &mut objcts_.oflag1[(oindex_.robot - 1i32) as (usize)];
                *_lhs = *_lhs & _rhs;
                let _rhs = 8192i32;
                let _lhs = &mut objcts_.oflag1[(oindex_.spher - 1i32) as (usize)];
                *_lhs = *_lhs | _rhs;
                return ret_val;
            }
        } else {
            bug_(11i32,ri);
        }
    }
    ret_val = 0i32;
    ret_val
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct13 {
    pub thfpos : i32,
    pub thfflg : i32,
    pub thfact : i32,
    pub swdact : i32,
    pub swdsta : i32,
}

impl Clone for Struct13 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct14 {
    pub rlnt : i32,
    pub rdesc1 : [i32; 200],
    pub rdesc2 : [i32; 200],
    pub rexit : [i32; 200],
    pub ractio : [i32; 200],
    pub rval : [i32; 200],
    pub rflag : [i32; 200],
}

impl Clone for Struct14 {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub unsafe extern fn thiefd_() {
    let mut _currentBlock;
    let mut i__1 : i32;
    let mut i__2 : i32;
    let mut i : i32;
    let mut j : i32;
    let mut nr : i32;
    let mut once : i32;
    let mut rhere : i32;
    once = 0i32;
    'loop1: loop {
        rhere = objcts_.oroom[(oindex_.thief - 1i32) as (usize)];
        if rhere != 0i32 {
            hack_.thfpos = rhere;
        }
        if hack_.thfpos == play_.here {
            if hack_.thfpos == rindex_.treas {
                _currentBlock = 53;
            } else if rooms_.rflag[
                          (hack_.thfpos - 1i32) as (usize)
                      ] & 16384i32 != 0i32 {
                _currentBlock = 38;
            } else {
                if hack_.thfflg != 0 {
                    if rhere == 0i32 {
                        _currentBlock = 53;
                    } else {
                        _currentBlock = 27;
                    }
                } else if rhere != 0i32 || prob_(70i32,70i32) != 0 {
                    if rhere == 0i32 || objcts_.oflag2[
                                            (oindex_.thief - 1i32) as (usize)
                                        ] & 256i32 == 0i32 {
                        _currentBlock = 22;
                    } else {
                        if winnin_(oindex_.thief,play_.winner) == 0 {
                            _currentBlock = 18;
                            break;
                        }
                        if prob_(90i32,90i32) != 0 {
                            _currentBlock = 53;
                        } else {
                            _currentBlock = 22;
                        }
                    }
                    if _currentBlock == 53 {
                    } else {
                        if !(rhere == 0i32 || prob_(70i32,70i32) != 0) {
                            _currentBlock = 23;
                            break;
                        }
                        _currentBlock = 27;
                    }
                } else {
                    if !(objcts_.ocan[
                             (oindex_.still - 1i32) as (usize)
                         ] != oindex_.thief) {
                        _currentBlock = 15;
                        break;
                    }
                    _currentBlock = 53;
                }
                if _currentBlock == 53 {
                } else {
                    if prob_(70i32,70i32) != 0 {
                        _currentBlock = 37;
                        break;
                    }
                    hack_.thfflg = 1i32;
                    i__1 = -oindex_.thief;
                    i__2 = -oindex_.thief;
                    nr = robrm_(hack_.thfpos,100i32,0i32,0i32,i__1) + robadv_(
                                                                          play_.winner,
                                                                          0i32,
                                                                          0i32,
                                                                          i__2
                                                                      );
                    i = 586i32;
                    if rhere != 0i32 {
                        i = 588i32;
                    }
                    if nr != 0i32 {
                        i = i + 1;
                    }
                    newsta_(oindex_.thief,i,0i32,0i32,0i32);
                    if qhere_(oindex_.still,hack_.thfpos) != 0 || objcts_.oadv[
                                                                      (oindex_.still - 1i32) as (usize)
                                                                  ] == -oindex_.thief {
                        newsta_(oindex_.still,0i32,0i32,oindex_.thief,0i32);
                    }
                    if nr != 0i32 && (lit_(hack_.thfpos) == 0) {
                        rspeak_(406i32);
                    }
                    rhere = 0i32;
                    _currentBlock = 53;
                }
            }
        } else if hack_.thfpos != rindex_.treas {
            _currentBlock = 38;
        } else {
            if !(rhere == 0i32) {
                newsta_(oindex_.thief,0i32,0i32,0i32,0i32);
                rhere = 0i32;
                if qhere_(oindex_.still,rindex_.treas) != 0 || objcts_.oadv[
                                                                   (oindex_.still - 1i32) as (usize)
                                                               ] == -oindex_.thief {
                    newsta_(oindex_.still,0i32,0i32,oindex_.thief,0i32);
                }
            }
            i__1 = -oindex_.thief;
            i = robadv_(i__1,hack_.thfpos,0i32,0i32);
            if qhere_(oindex_.egg,hack_.thfpos) != 0 {
                let _rhs = 8i32;
                let _lhs = &mut objcts_.oflag2[(oindex_.egg - 1i32) as (usize)];
                *_lhs = *_lhs | _rhs;
                _currentBlock = 53;
            } else {
                _currentBlock = 53;
            }
        }
        if _currentBlock == 38 {
            newsta_(oindex_.thief,0i32,0i32,0i32,0i32);
            rhere = 0i32;
            if qhere_(oindex_.still,hack_.thfpos) != 0 || objcts_.oadv[
                                                              (oindex_.still - 1i32) as (usize)
                                                          ] == -oindex_.thief {
                newsta_(oindex_.still,0i32,0i32,oindex_.thief,0i32);
            }
            if !(rooms_.rflag[
                     (hack_.thfpos - 1i32) as (usize)
                 ] & 32768i32 == 0i32) {
                i__1 = -oindex_.thief;
                i = robrm_(hack_.thfpos,75i32,0i32,0i32,i__1);
                if hack_.thfpos < rindex_.maze1 || hack_.thfpos > rindex_.maz15 || play_.here < rindex_.maze1 || play_.here > rindex_.maz15 {
                    i__1 = objcts_.olnt;
                    i = 1i32;
                    'loop49: loop {
                        if !(i <= i__1) {
                            _currentBlock = 53;
                            break;
                        }
                        if !(qhere_(i,hack_.thfpos) == 0 || objcts_.otval[
                                                                (i - 1i32) as (usize)
                                                            ] != 0i32 || prob_(
                                                                             80i32,
                                                                             60i32
                                                                         ) != 0 || objcts_.oflag1[
                                                                                       (i - 1i32) as (usize)
                                                                                   ] & 32768i32 + 8192i32 != 32768i32 + 8192i32) {
                            _currentBlock = 51;
                            break;
                        }
                        i = i + 1;
                    }
                    if _currentBlock == 53 {
                    } else {
                        i__2 = -oindex_.thief;
                        newsta_(i,0i32,0i32,0i32,i__2);
                        let _rhs = 4i32;
                        let _lhs = &mut objcts_.oflag2[(i - 1i32) as (usize)];
                        *_lhs = *_lhs | _rhs;
                    }
                } else {
                    i__1 = objcts_.olnt;
                    i = 1i32;
                    'loop43: loop {
                        if !(i <= i__1) {
                            _currentBlock = 53;
                            break;
                        }
                        if !(qhere_(i,hack_.thfpos) == 0 || prob_(
                                                                60i32,
                                                                60i32
                                                            ) != 0 || objcts_.oflag1[
                                                                          (i - 1i32) as (usize)
                                                                      ] & 32768i32 + 8192i32 != 32768i32 + 8192i32) {
                            _currentBlock = 45;
                            break;
                        }
                        i = i + 1;
                    }
                    if _currentBlock == 53 {
                    } else {
                        rspsub_(590i32,objcts_.odesc2[(i - 1i32) as (usize)]);
                        if prob_(40i32,20i32) == 0 {
                            i__2 = -oindex_.thief;
                            newsta_(i,0i32,0i32,0i32,i__2);
                            let _rhs = 4i32;
                            let _lhs = &mut objcts_.oflag2[(i - 1i32) as (usize)];
                            *_lhs = *_lhs | _rhs;
                        }
                    }
                }
            }
        }
        if objcts_.oadv[
               (oindex_.rope - 1i32) as (usize)
           ] == -oindex_.thief {
            findex_.domef = 0i32;
        }
        if once != 0 {
            _currentBlock = 61;
            break;
        }
        once = (once == 0) as (i32);
        'loop57: loop {
            hack_.thfpos = hack_.thfpos - 1;
            if hack_.thfpos <= 0i32 {
                hack_.thfpos = rooms_.rlnt;
            }
            if !(rooms_.rflag[
                     (hack_.thfpos - 1i32) as (usize)
                 ] & 8192i32 + 1024i32 + 16i32 != 8192i32) {
                break;
            }
        }
        hack_.thfflg = 0i32;
    }
    if _currentBlock == 15 {
        newsta_(oindex_.thief,583i32,hack_.thfpos,0i32,0i32);
        hack_.thfflg = 1i32;
    } else if _currentBlock == 18 {
        newsta_(oindex_.thief,584i32,0i32,0i32,0i32);
        let _rhs = !256i32;
        let _lhs = &mut objcts_.oflag2[(oindex_.thief - 1i32) as (usize)];
        *_lhs = *_lhs & _rhs;
        if qhere_(oindex_.still,hack_.thfpos) != 0 || objcts_.oadv[
                                                          (oindex_.still - 1i32) as (usize)
                                                      ] == -oindex_.thief {
            newsta_(oindex_.still,0i32,0i32,oindex_.thief,0i32);
        }
    } else if _currentBlock == 23 {
        newsta_(oindex_.thief,585i32,0i32,0i32,0i32);
        if qhere_(oindex_.still,hack_.thfpos) != 0 || objcts_.oadv[
                                                          (oindex_.still - 1i32) as (usize)
                                                      ] == -oindex_.thief {
            newsta_(oindex_.still,0i32,0i32,oindex_.thief,0i32);
        }
    } else if _currentBlock == 37 {
    } else if hack_.thfpos == rindex_.treas {
    } else {
        j = 591i32;
        if hack_.thfpos != play_.here {
            j = 0i32;
        }
        i__1 = objcts_.olnt;
        i = 1i32;
        'loop65: loop {
            if !(i <= i__1) {
                break;
            }
            if !(objcts_.oadv[
                     (i - 1i32) as (usize)
                 ] != -oindex_.thief || prob_(70i32,70i32) != 0 || objcts_.otval[
                                                                       (i - 1i32) as (usize)
                                                                   ] > 0i32) {
                newsta_(i,j,hack_.thfpos,0i32,0i32);
                j = 0i32;
            }
            i = i + 1;
        }
    }
}
